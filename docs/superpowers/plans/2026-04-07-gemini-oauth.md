# Google Gemini OAuth 集成实现方案

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 让用户通过 Google 账号登录，免费使用 Gemini 模型（通过 Antigravity/CloudCode 内部 API），无需 API Key。

**Architecture:** 新增 `gemini_oauth.rs` 处理 Google OAuth PKCE 登录和 token 管理。新增 `gemini_api.rs` 调用 `cloudcode-pa.googleapis.com/v1internal` 端点（Antigravity envelope 格式，SSE 流式）。复用 OpenAI OAuth 的前端 UI 模式，在 Google 提供商区域增加登录按钮。

**Tech Stack:** Rust (tokio, reqwest, sha2, base64), Tauri 2 commands, React/TypeScript

**已验证的参数：**
- Client ID: `1071006060591-tmhssin2h21lcre235vtolojh4g403ep.apps.googleusercontent.com`
- Client Secret: `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`
- Auth URL: `https://accounts.google.com/o/oauth2/v2/auth`
- Token URL: `https://oauth2.googleapis.com/token`
- Callback: `http://localhost:51121/oauth-callback`
- Scopes: `cloud-platform userinfo.email userinfo.profile cclog experimentsandconfigs`
- API URL: `https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse`
- loadCodeAssist URL: `https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist`
- 支持的模型: `gemini-3-flash`, `gemini-3-pro`, `gemini-3.1-pro`
- 请求格式: Antigravity envelope（project + model + request）
- 响应格式: SSE，解析 `response.candidates[0].content.parts[].text`

---

## 文件结构

| 文件 | 操作 | 职责 |
|------|------|------|
| `src-tauri/src/ai/gemini_oauth.rs` | 新建 | Google OAuth PKCE 登录、token 管理、projectId 获取 |
| `src-tauri/src/ai/gemini_api.rs` | 新建 | Antigravity API 调用（envelope 封装 + SSE 解析） |
| `src-tauri/src/ai/mod.rs` | 修改 | 注册新模块 |
| `src-tauri/src/commands/oauth.rs` | 修改 | 添加 Gemini OAuth 命令 |
| `src-tauri/src/lib.rs` | 修改 | 注册新命令 |
| `src-tauri/src/commands/capture.rs` | 修改 | 摘要生成支持 Gemini OAuth |
| `src-tauri/src/commands/attention.rs` | 修改 | 雷达分析支持 Gemini OAuth |
| `src-tauri/src/ai/attention_analyzer.rs` | 修改 | 添加 try_gemini_codex_call |
| `src/stores/settingsStore.ts` | 修改 | 添加 Gemini OAuth 状态、Google 模型列表更新 |
| `src/features/settings/SettingsView.tsx` | 修改 | Google 提供商增加登录 UI |

---

### Task 1: Gemini OAuth 核心模块

**Files:**
- Create: `src-tauri/src/ai/gemini_oauth.rs`
- Modify: `src-tauri/src/ai/mod.rs`

- [ ] **Step 1: 注册模块**

`src-tauri/src/ai/mod.rs` 添加：
```rust
pub mod gemini_api;
pub mod gemini_oauth;
```

- [ ] **Step 2: 创建 gemini_oauth.rs**

结构与 `oauth.rs` 类似，但用 Google 的 OAuth 端点和参数。关键区别：
- 用 `client_secret`（Google OAuth 需要）
- 回调端口 `51121`
- 需要额外获取 `project_id`（通过 loadCodeAssist）
- Token 存储在 `gemini_oauth_token` 设置键下

```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::storage::database::Database;
use crate::storage::repository::Repository;

const CLIENT_ID: &str = "1071006060591-tmhssin2h21lcre235vtolojh4g403ep.apps.googleusercontent.com";
const CLIENT_SECRET: &str = "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";
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const REDIRECT_URI: &str = "http://localhost:51121/oauth-callback";
const CALLBACK_PORT: u16 = 51121;
const SCOPES: &str = "https://www.googleapis.com/auth/cloud-platform https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/cclog https://www.googleapis.com/auth/experimentsandconfigs";
const LOAD_CODE_ASSIST_ENDPOINT: &str = "https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist";
const DEFAULT_PROJECT_ID: &str = "rising-fact-p41fc";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiOAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub project_id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeminiOAuthStatus {
    pub logged_in: bool,
    pub email: Option<String>,
    pub expires_at: Option<i64>,
}

pub static GEMINI_OAUTH_STATE: once_cell::sync::Lazy<std::sync::Arc<std::sync::Mutex<Option<GeminiOAuthToken>>>> =
    once_cell::sync::Lazy::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));
```

- [ ] **Step 3: 实现 PKCE 和辅助函数**

复用与 oauth.rs 相同的 PKCE 逻辑（generate_code_verifier, generate_code_challenge, url_encode, url_decode）。

- [ ] **Step 4: 实现 OAuth 登录流程**

```rust
pub async fn start_gemini_oauth_login() -> Result<GeminiOAuthToken, String> {
    // 1. 生成 PKCE
    // 2. 在 localhost:51121 启动回调服务器
    // 3. 构建 Google Auth URL（包含 client_id, scope, PKCE, access_type=offline, prompt=consent）
    // 4. 打开浏览器
    // 5. 等待回调（180秒超时）
    // 6. 用 code + client_secret + code_verifier 换 token
    // 7. 获取用户邮箱（GET https://www.googleapis.com/oauth2/v1/userinfo?alt=json）
    // 8. 获取 projectId（POST loadCodeAssist）
    // 9. 返回 GeminiOAuthToken
}
```

Token 交换请求（与 OpenAI 不同，需要 `client_secret`）：
```
POST https://oauth2.googleapis.com/token
Content-Type: application/x-www-form-urlencoded;charset=UTF-8
User-Agent: google-api-nodejs-client/9.15.1

client_id=...&client_secret=...&code=...&grant_type=authorization_code&redirect_uri=...&code_verifier=...
```

- [ ] **Step 5: 实现 loadCodeAssist 获取 projectId**

```rust
async fn load_project_id(access_token: &str) -> String {
    // POST https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist
    // Headers: Authorization, Content-Type, User-Agent, Client-Metadata
    // Body: {"metadata":{"ideType":"ANTIGRAVITY","platform":"PLATFORM_UNSPECIFIED","pluginType":"GEMINI"}}
    // 返回 cloudaicompanionProject 字段
    // 失败则用默认值 "rising-fact-p41fc"
}
```

- [ ] **Step 6: 实现 token 刷新（需要 client_secret）**

```rust
pub async fn refresh_gemini_token(refresh: &str) -> Result<GeminiOAuthToken, String> {
    // POST https://oauth2.googleapis.com/token
    // Body: grant_type=refresh_token&refresh_token=...&client_id=...&client_secret=...
}
```

- [ ] **Step 7: 实现 get_valid_token / save_token / clear_token**

与 OpenAI 的 oauth.rs 模式相同，但存储键为 `"gemini_oauth_token"`。
`get_valid_token` 返回 `Option<(String, String)>` = `(access_token, project_id)`。

- [ ] **Step 8: 验证编译**

Run: `cd src-tauri && cargo check`

- [ ] **Step 9: Commit**

```bash
git add src-tauri/src/ai/gemini_oauth.rs src-tauri/src/ai/mod.rs
git commit -m "feat: add Google Gemini OAuth module (Antigravity PKCE + projectId)"
```

---

### Task 2: Gemini API 调用模块

**Files:**
- Create: `src-tauri/src/ai/gemini_api.rs`

- [ ] **Step 1: 创建 gemini_api.rs**

```rust
use reqwest::Client;
use std::time::Duration;

const GEMINI_API_ENDPOINT: &str = "https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse";

/// Call Gemini via Antigravity CloudCode API.
///
/// Request body uses Antigravity envelope format:
/// {
///   "project": "...",
///   "model": "gemini-3-flash",
///   "request": { contents, generationConfig, systemInstruction },
///   "requestType": "agent",
///   "userAgent": "antigravity",
///   "requestId": "agent-{uuid}"
/// }
pub async fn call_gemini_api(
    access_token: &str,
    project_id: &str,
    model: &str,
    system_prompt: &str,
    user_message: &str,
) -> Result<String, String> {
    let http_client = Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .map_err(|e| format!("HTTP client 创建失败: {}", e))?;

    let request_id = format!("agent-{}", uuid::Uuid::new_v4());

    let body = serde_json::json!({
        "project": project_id,
        "model": model,
        "request": {
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": user_message}]
                }
            ],
            "generationConfig": {
                "maxOutputTokens": 8000,
                "temperature": 0.3
            },
            "systemInstruction": {
                "role": "user",
                "parts": [{"text": if system_prompt.is_empty() { "You are a helpful assistant." } else { system_prompt }}]
            }
        },
        "requestType": "agent",
        "userAgent": "antigravity",
        "requestId": request_id
    });

    let resp = http_client
        .post(GEMINI_API_ENDPOINT)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("User-Agent", "antigravity/1.18.3 darwin/arm64")
        .header("Accept", "text/event-stream")
        .header("X-Goog-Api-Client", "google-cloud-sdk vscode_cloudshelleditor/0.1")
        .header("Client-Metadata", r#"{"ideType":"ANTIGRAVITY","platform":"PLATFORM_UNSPECIFIED","pluginType":"GEMINI"}"#)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Gemini API 请求失败: {}", e))?;

    let status = resp.status();
    let text = resp.text().await
        .map_err(|e| format!("读取 Gemini 响应失败: {}", e))?;

    if !status.is_success() {
        return Err(format!("Gemini API 错误 ({}): {}", status, text));
    }

    // Parse SSE: data: {"response":{"candidates":[{"content":{"parts":[{"text":"..."}]}}]}}
    let mut result = String::new();
    for line in text.lines() {
        if let Some(data) = line.strip_prefix("data:") {
            let data = data.trim();
            if data.is_empty() { continue; }
            if let Ok(event) = serde_json::from_str::<serde_json::Value>(data) {
                // Try both wrapped (response.candidates) and direct (candidates) format
                let candidates = event.get("response")
                    .and_then(|r| r.get("candidates"))
                    .or_else(|| event.get("candidates"));
                if let Some(arr) = candidates.and_then(|c| c.as_array()) {
                    for candidate in arr {
                        if let Some(parts) = candidate.get("content")
                            .and_then(|c| c.get("parts"))
                            .and_then(|p| p.as_array())
                        {
                            for part in parts {
                                // Skip thinking parts (thought: true)
                                if part.get("thought").and_then(|t| t.as_bool()).unwrap_or(false) {
                                    continue;
                                }
                                if let Some(t) = part.get("text").and_then(|t| t.as_str()) {
                                    result.push_str(t);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if result.is_empty() {
        Err("Gemini API 返回空响应".to_string())
    } else {
        log::info!("Gemini API 调用成功，响应长度: {}", result.len());
        Ok(result)
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo check`

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/ai/gemini_api.rs
git commit -m "feat: add Gemini Antigravity API caller with SSE parsing"
```

---

### Task 3: Tauri 命令 + 接入 AI 路径

**Files:**
- Modify: `src-tauri/src/commands/oauth.rs` — 添加 3 个 Gemini 命令
- Modify: `src-tauri/src/lib.rs` — 注册命令
- Modify: `src-tauri/src/ai/attention_analyzer.rs` — 添加 try_gemini_call
- Modify: `src-tauri/src/commands/capture.rs` — 摘要生成支持 Gemini OAuth
- Modify: `src-tauri/src/commands/attention.rs` — 雷达分析支持 Gemini OAuth

- [ ] **Step 1: 在 commands/oauth.rs 添加 Gemini 命令**

```rust
use crate::ai::gemini_oauth;

#[tauri::command]
pub async fn start_gemini_oauth(state: State<'_, AppState>) -> Result<gemini_oauth::GeminiOAuthStatus, String> {
    let token = gemini_oauth::start_gemini_oauth_login().await?;
    gemini_oauth::save_token(state.db.clone(), &token).await;
    Ok(gemini_oauth::GeminiOAuthStatus {
        logged_in: true,
        email: Some(token.email),
        expires_at: Some(token.expires_at),
    })
}

#[tauri::command]
pub async fn get_gemini_oauth_status(state: State<'_, AppState>) -> Result<gemini_oauth::GeminiOAuthStatus, String> {
    // 同 OpenAI 模式：get_valid_token → 返回状态
}

#[tauri::command]
pub async fn logout_gemini_oauth(state: State<'_, AppState>) -> Result<(), String> {
    gemini_oauth::clear_token(state.db.clone()).await;
    Ok(())
}
```

- [ ] **Step 2: 注册命令到 lib.rs**

```rust
commands::oauth::start_gemini_oauth,
commands::oauth::get_gemini_oauth_status,
commands::oauth::logout_gemini_oauth,
```

- [ ] **Step 3: 在 attention_analyzer.rs 添加 try_gemini_call**

```rust
pub async fn try_gemini_call(
    db: std::sync::Arc<Database>,
    system_prompt: &str,
    user_message: &str,
) -> Option<Result<String, String>> {
    let (access_token, project_id) = crate::ai::gemini_oauth::get_valid_token(db.clone()).await?;
    let repo = Repository::new(db);
    let model = repo.get_setting("ai_model").ok().flatten()
        .unwrap_or_else(|| "gemini-3-flash".to_string());
    Some(crate::ai::gemini_api::call_gemini_api(
        &access_token, &project_id, &model, system_prompt, user_message,
    ).await)
}
```

- [ ] **Step 4: 修改 capture.rs — 摘要生成支持 Gemini**

在现有的 OpenAI Codex 路径之后，添加 Google/Gemini 路径：
```rust
// Try Gemini OAuth (if provider is google/gemini and OAuth available)
if provider_str == "google" {
    if let Some(result) = try_gemini_call(state.db.clone(), "", &prompt).await {
        match result {
            Ok(raw) => { /* 同 Codex 成功路径 */ }
            Err(e) => { log::warn!("Gemini OAuth 失败，回退到 API Key: {}", e); }
        }
    }
}
```

- [ ] **Step 5: 修改 attention.rs — 雷达分析支持 Gemini**

同样的模式，在 OpenAI Codex 路径之后添加 Gemini 路径。

- [ ] **Step 6: 验证编译**

Run: `cd src-tauri && cargo check`

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/commands/oauth.rs src-tauri/src/lib.rs src-tauri/src/ai/attention_analyzer.rs src-tauri/src/commands/capture.rs src-tauri/src/commands/attention.rs
git commit -m "feat: add Gemini OAuth commands and integrate into AI call paths"
```

---

### Task 4: 前端 — Google 提供商 + 登录 UI

**Files:**
- Modify: `src/stores/settingsStore.ts`
- Modify: `src/features/settings/SettingsView.tsx`

- [ ] **Step 1: 更新 settingsStore.ts**

添加 `"google"` 到 `AIProvider` 类型和相关配置：

```typescript
export type AIProvider = "anthropic" | "openai" | "openrouter" | "dashscope" | "google";

// 在 MODELS_BY_PROVIDER 中添加：
google: [
    { id: "gemini-3-flash", label: "Gemini 3 Flash" },
    { id: "gemini-3-pro", label: "Gemini 3 Pro" },
    { id: "gemini-3.1-pro", label: "Gemini 3.1 Pro" },
],

// 在 PROVIDER_LABELS 中添加：
google: "Google",

// 在 VALID_PROVIDERS 中添加 "google"

// 添加 Gemini OAuth 状态字段：
geminiOauthLoggedIn: boolean;
geminiOauthEmail: string;
geminiOauthLoading: boolean;
loadGeminiOAuthStatus: () => Promise<void>;
startGeminiOAuthLogin: () => Promise<void>;
logoutGeminiOAuth: () => Promise<void>;
```

Actions 调用 `start_gemini_oauth`、`get_gemini_oauth_status`、`logout_gemini_oauth` 命令。

在 `loadFromDB` 中加载 Gemini OAuth 状态。

- [ ] **Step 2: 更新 SettingsView.tsx**

在 `provider === "google"` 时显示登录按钮（与 OpenAI 相同的 UI 模式，但按钮颜色用 Google 蓝 `#4285f4`）：

```tsx
{provider === "google" && (
    <div className="p-4">
        {/* 同 OpenAI OAuth UI 模式，但按钮文字为"登录 Google 账号"，颜色为 #4285f4 */}
    </div>
)}
```

- [ ] **Step 3: 更新后端 AnalysisProvider 枚举**

在 `attention_analyzer.rs` 的 `AnalysisProvider` 中添加 `Google` 变体（如果需要通过 API Key 调用的话）。或者如果 Google 只走 OAuth，可以跳过这步。

- [ ] **Step 4: 验证编译**

Run: `cd src-tauri && cargo check && npx tsc --noEmit`

- [ ] **Step 5: Commit**

```bash
git add src/stores/settingsStore.ts src/features/settings/SettingsView.tsx
git commit -m "feat: add Google provider with Gemini OAuth login UI"
```

---

### Task 5: 端到端测试 + 推送

- [ ] **Step 1: 启动 dev 服务器**
- [ ] **Step 2: 测试 Google OAuth 登录**（设置 → AI → Google → 登录 Google 账号）
- [ ] **Step 3: 测试 AI 调用**（保存内容 → 确认摘要由 Gemini 生成）
- [ ] **Step 4: 测试退出登录**
- [ ] **Step 5: 测试 OpenAI OAuth 仍正常工作**（切回 OpenAI 提供商）
- [ ] **Step 6: Commit + Push**

```bash
git add -A
git commit -m "feat: Google Gemini OAuth 集成 — 用 Google 账号免费调用 Gemini"
git push origin newid
```
