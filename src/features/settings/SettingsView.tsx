import { useState } from "react";
import {
  useSettingsStore,
  MODELS_BY_PROVIDER,
  PROVIDER_LABELS,
  type AIProvider,
  type ThemeMode,
} from "../../stores/settingsStore";

const THEME_OPTIONS: { value: ThemeMode; label: string; icon: string }[] = [
  { value: "light", label: "浅色", icon: "☀️" },
  { value: "dark", label: "深色", icon: "🌙" },
  { value: "system", label: "跟随系统", icon: "💻" },
];

export function SettingsView() {
  const {
    apiKey,
    provider,
    model,
    theme,
    captureEnabled,
    sensitiveFilterEnabled,
    urlReadingEnabled,
    screenshotDir,
    totalItems,
    diskUsageMB,
    setApiKey,
    setProvider,
    setModel,
    setTheme,
    setCaptureEnabled,
    setSensitiveFilterEnabled,
    setUrlReadingEnabled,
  } = useSettingsStore();

  const [showApiKey, setShowApiKey] = useState(false);

  const availableModels = MODELS_BY_PROVIDER[provider];

  return (
    <div className="max-w-2xl mx-auto p-6 space-y-6">
      {/* Appearance */}
      <section>
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
          <span className="text-xl">🎨</span>
          外观
        </h2>
        <div className="bg-white dark:bg-slate-800 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm">
          <div className="p-4">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              主题模式
            </label>
            <div className="flex gap-2">
              {THEME_OPTIONS.map((opt) => (
                <button
                  key={opt.value}
                  onClick={() => setTheme(opt.value)}
                  className={`
                    flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 text-sm font-medium rounded-lg border
                    transition-all duration-150
                    ${
                      theme === opt.value
                        ? "bg-blue-50 dark:bg-blue-500/15 border-blue-300 dark:border-blue-500/50 text-blue-700 dark:text-blue-400 shadow-sm"
                        : "bg-white dark:bg-slate-700 border-gray-200 dark:border-slate-600 text-gray-600 dark:text-slate-300 hover:bg-gray-50 dark:hover:bg-slate-600"
                    }
                  `}
                >
                  <span>{opt.icon}</span>
                  <span>{opt.label}</span>
                </button>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* AI Configuration */}
      <section>
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
          <span className="text-xl">🤖</span>
          AI 配置
        </h2>
        <div className="bg-white dark:bg-slate-800 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm divide-y divide-gray-100 dark:divide-slate-700">
          {/* API Key */}
          <div className="p-4">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
              API Key
            </label>
            <div className="relative">
              <input
                type={showApiKey ? "text" : "password"}
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="输入你的 API Key..."
                className="w-full px-3 py-2 pr-20 text-sm border border-gray-200 dark:border-slate-600 rounded-lg
                           bg-gray-50 dark:bg-slate-700 text-gray-800 dark:text-gray-200
                           placeholder-gray-400 dark:placeholder-slate-500
                           focus:bg-white dark:focus:bg-slate-600 focus:border-blue-400 dark:focus:border-blue-500
                           focus:ring-1 focus:ring-blue-400 dark:focus:ring-blue-500 outline-none transition-all"
              />
              <button
                type="button"
                onClick={() => setShowApiKey(!showApiKey)}
                className="absolute right-2 top-1/2 -translate-y-1/2 px-2 py-1
                           text-xs text-gray-500 dark:text-slate-400 hover:text-gray-700 dark:hover:text-slate-200
                           bg-gray-100 dark:bg-slate-600 hover:bg-gray-200 dark:hover:bg-slate-500 rounded transition-colors"
              >
                {showApiKey ? "隐藏" : "显示"}
              </button>
            </div>
            <p className="mt-1.5 text-xs text-gray-400 dark:text-slate-500">
              Key 将安全存储在本地，不会上传到任何服务器
            </p>
          </div>

          {/* AI Provider */}
          <div className="p-4">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
              AI 服务商
            </label>
            <div className="flex gap-2">
              {(["anthropic", "openai", "openrouter"] as AIProvider[]).map((p) => (
                <button
                  key={p}
                  onClick={() => setProvider(p)}
                  className={`
                    flex-1 px-3 py-2 text-sm font-medium rounded-lg border
                    transition-all duration-150
                    ${
                      provider === p
                        ? "bg-blue-50 dark:bg-blue-500/15 border-blue-300 dark:border-blue-500/50 text-blue-700 dark:text-blue-400 shadow-sm"
                        : "bg-white dark:bg-slate-700 border-gray-200 dark:border-slate-600 text-gray-600 dark:text-slate-300 hover:bg-gray-50 dark:hover:bg-slate-600"
                    }
                  `}
                >
                  {PROVIDER_LABELS[p]}
                </button>
              ))}
            </div>
            {provider === "openrouter" && (
              <p className="mt-1.5 text-xs text-gray-400 dark:text-slate-500">
                OpenRouter 支持多种模型，使用统一 API Key，
                <a
                  href="https://openrouter.ai/keys"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-blue-500 dark:text-blue-400 hover:underline"
                >
                  前往获取 Key
                </a>
              </p>
            )}
          </div>

          {/* Model Selection */}
          <div className="p-4">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
              模型
            </label>
            <select
              value={model}
              onChange={(e) => setModel(e.target.value)}
              className="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-600 rounded-lg
                         bg-gray-50 dark:bg-slate-700 text-gray-800 dark:text-gray-200
                         focus:bg-white dark:focus:bg-slate-600 focus:border-blue-400 dark:focus:border-blue-500
                         focus:ring-1 focus:ring-blue-400 dark:focus:ring-blue-500 outline-none transition-all cursor-pointer"
            >
              {(() => {
                const hasGroups = availableModels.some((m) => m.group);
                if (!hasGroups) {
                  return availableModels.map((m) => (
                    <option key={m.id} value={m.id}>{m.label}</option>
                  ));
                }
                const groups: string[] = [];
                for (const m of availableModels) {
                  const g = m.group || "其他";
                  if (!groups.includes(g)) groups.push(g);
                }
                return groups.map((g) => (
                  <optgroup key={g} label={g}>
                    {availableModels
                      .filter((m) => (m.group || "其他") === g)
                      .map((m) => (
                        <option key={m.id} value={m.id}>
                          {m.free ? `🆓 ${m.label}` : m.label}
                        </option>
                      ))}
                  </optgroup>
                ));
              })()}
            </select>
            {availableModels.find((m) => m.id === model)?.free && (
              <p className="mt-1.5 text-xs text-green-600 dark:text-green-400">
                ✅ 当前模型免费，无需消耗额度
              </p>
            )}
          </div>
        </div>
      </section>

      {/* Capture Settings */}
      <section>
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
          <span className="text-xl">📸</span>
          捕获设置
        </h2>
        <div className="bg-white dark:bg-slate-800 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm divide-y divide-gray-100 dark:divide-slate-700">
          {/* Capture Toggle */}
          <div className="p-4 flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-gray-700 dark:text-gray-300">
                内容捕获
              </div>
              <div className="text-xs text-gray-400 dark:text-slate-500 mt-0.5">
                开启后将自动检测剪贴板和截图变化
              </div>
            </div>
            <button
              onClick={() => setCaptureEnabled(!captureEnabled)}
              className={`
                relative w-11 h-6 rounded-full transition-colors duration-200
                ${captureEnabled ? "bg-blue-500" : "bg-gray-300 dark:bg-slate-600"}
              `}
            >
              <span
                className="absolute top-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform duration-200"
                style={{
                  transform: captureEnabled
                    ? "translateX(22px)"
                    : "translateX(2px)",
                }}
              />
            </button>
          </div>

          {/* Sensitive Data Filter Toggle */}
          <div className="p-4 flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-gray-700 dark:text-gray-300">
                敏感数据过滤
              </div>
              <div className="text-xs text-gray-400 dark:text-slate-500 mt-0.5">
                自动过滤密码、私钥、API Key、Token 等敏感内容
              </div>
            </div>
            <button
              onClick={() => setSensitiveFilterEnabled(!sensitiveFilterEnabled)}
              className={`
                relative w-11 h-6 rounded-full transition-colors duration-200
                ${sensitiveFilterEnabled ? "bg-amber-500" : "bg-gray-300 dark:bg-slate-600"}
              `}
            >
              <span
                className="absolute top-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform duration-200"
                style={{
                  transform: sensitiveFilterEnabled
                    ? "translateX(22px)"
                    : "translateX(2px)",
                }}
              />
            </button>
          </div>

          {/* URL Reading Toggle */}
          <div className="p-4 flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-gray-700 dark:text-gray-300">
                链接内容读取
              </div>
              <div className="text-xs text-gray-400 dark:text-slate-500 mt-0.5">
                复制链接时自动获取网页正文，提升周报分析质量
              </div>
            </div>
            <button
              onClick={() => setUrlReadingEnabled(!urlReadingEnabled)}
              className={`
                relative w-11 h-6 rounded-full transition-colors duration-200
                ${urlReadingEnabled ? "bg-green-500" : "bg-gray-300 dark:bg-slate-600"}
              `}
            >
              <span
                className="absolute top-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform duration-200"
                style={{
                  transform: urlReadingEnabled
                    ? "translateX(22px)"
                    : "translateX(2px)",
                }}
              />
            </button>
          </div>

          {/* Screenshot Directory */}
          <div className="p-4">
            <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
              截图存储目录
            </div>
            <div
              className="px-3 py-2 text-xs text-gray-500 dark:text-slate-400 bg-gray-50 dark:bg-slate-700 rounded-lg
                         border border-gray-100 dark:border-slate-600 font-mono break-all"
            >
              {screenshotDir}
            </div>
          </div>
        </div>
      </section>

      {/* Storage Info */}
      <section>
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
          <span className="text-xl">💾</span>
          存储信息
        </h2>
        <div className="bg-white dark:bg-slate-800 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm">
          <div className="p-4 grid grid-cols-2 gap-4">
            <div className="text-center p-3 bg-gray-50 dark:bg-slate-700 rounded-lg">
              <div className="text-2xl font-bold text-gray-800 dark:text-gray-100">
                {totalItems}
              </div>
              <div className="text-xs text-gray-500 dark:text-slate-400 mt-1">已保存内容</div>
            </div>
            <div className="text-center p-3 bg-gray-50 dark:bg-slate-700 rounded-lg">
              <div className="text-2xl font-bold text-gray-800 dark:text-gray-100">
                {diskUsageMB.toFixed(1)} MB
              </div>
              <div className="text-xs text-gray-500 dark:text-slate-400 mt-1">磁盘占用</div>
            </div>
          </div>
        </div>
      </section>

      {/* Version Info */}
      <div className="text-center pb-4">
        <p className="text-xs text-gray-400 dark:text-slate-600">小云 v0.1.0</p>
      </div>
    </div>
  );
}
