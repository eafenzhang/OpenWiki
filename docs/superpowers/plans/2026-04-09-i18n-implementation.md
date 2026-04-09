# OpenWiki i18n Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Chinese/English bilingual support with language switching to OpenWiki, auto-detecting OS language on first launch.

**Architecture:** react-i18next with static imports, localStorage persistence, 6 namespace JSON files per language. Language switcher in Settings > Appearance. Brand renamed from "小云" to "OpenWiki".

**Tech Stack:** i18next, react-i18next, i18next-browser-languagedetector

**Design spec:** `docs/superpowers/specs/2026-04-09-i18n-design.md`

---

## File Structure Overview

```
New files:
  src/lib/i18n.ts                          — i18n initialization
  src/locales/zh-CN/common.json            — shared text (nav, buttons, time)
  src/locales/zh-CN/settings.json          — settings page
  src/locales/zh-CN/content.json           — content list, cards
  src/locales/zh-CN/wiki.json              — knowledge base
  src/locales/zh-CN/digest.json            — insights/radar + digest
  src/locales/zh-CN/datahub.json           — data hub + export + weekly report
  src/locales/en/common.json
  src/locales/en/settings.json
  src/locales/en/content.json
  src/locales/en/wiki.json
  src/locales/en/digest.json
  src/locales/en/datahub.json

Modified files:
  package.json                             — add i18n dependencies
  src/main.tsx                             — import i18n init
  src/App.tsx                              — brand rename + useTranslation
  src/stores/settingsStore.ts              — move Chinese labels to i18n keys
  src/features/settings/SettingsView.tsx   — language switcher + useTranslation
  src/features/content-list/ContentList.tsx
  src/features/content-list/ContentCard.tsx
  src/features/content-list/ImagePreview.tsx
  src/features/wiki/WikiView.tsx
  src/features/wiki/WikiBrowseView.tsx
  src/features/wiki/WikiPageDetail.tsx
  src/features/wiki/WikiPageCard.tsx
  src/features/wiki/WikiAskSidebar.tsx
  src/features/wiki/WikiGraphView.tsx
  src/features/wiki/WikiLintSection.tsx
  src/features/digest/RadarView.tsx
  src/features/digest/DigestView.tsx
  src/features/digest/DigestCard.tsx
  src/features/digest/InsightDetail.tsx
  src/features/data-hub/DayDetail.tsx
  src/features/data-hub/DateSidebar.tsx
  src/features/data-hub/ExportPanel.tsx
  src/features/weekly-report/ReportView.tsx
  src/features/weekly-report/ReportCard.tsx
  src/features/weekly-report/ContentPreviewPanel.tsx
  src/features/weekly-report/FeedbackButtons.tsx
  src/features/weekly-report/TextContentList.tsx
  src/features/weekly-report/CompactLinkList.tsx
  src/features/weekly-report/ActivityStatsCard.tsx
  src/features/weekly-report/ImageFilmstrip.tsx
  src/components/FloatingBubble.tsx
  src/components/BubbleView.tsx
  src/features/spotlight/SpotlightView.tsx
```

---

## Task 1: Install Dependencies and Create i18n Foundation

**Files:**
- Modify: `package.json`
- Create: `src/lib/i18n.ts`
- Modify: `src/main.tsx`

- [ ] **Step 1: Install i18n packages**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm install i18next react-i18next i18next-browser-languagedetector
```

- [ ] **Step 2: Create locale directory structure**

```bash
mkdir -p src/locales/zh-CN src/locales/en
```

- [ ] **Step 3: Create Chinese common.json**

Create `src/locales/zh-CN/common.json`:

```json
{
  "brand": "OpenWiki",
  "nav.content": "内容",
  "nav.wiki": "知识",
  "nav.digest": "洞察",
  "nav.settings": "设置",
  "search.placeholder": "搜索内容...",
  "search.searching": "搜索中...",
  "search.noResults": "无结果",
  "search.wikiPages": "知识页面",
  "search.capturedContent": "捕获内容",
  "search.noContent": "无内容",
  "search.tooltip": "搜索",
  "btn.save": "保存",
  "btn.cancel": "取消",
  "btn.delete": "删除",
  "btn.confirm": "确认",
  "btn.dismiss": "丢弃",
  "btn.retry": "重试",
  "btn.export": "导出",
  "btn.expand": "展开",
  "btn.collapse": "收起",
  "time.justNow": "刚刚",
  "time.minutesAgo": "{{count}}分钟前",
  "time.hoursAgo": "{{count}}小时前",
  "time.daysAgo": "{{count}}天前",
  "time.monthsAgo": "{{count}}个月前",
  "time.yearsAgo": "{{count}}年前",
  "time.today": "今天",
  "time.yesterday": "昨天",
  "type.text": "文本",
  "type.image": "图片",
  "type.link": "链接",
  "type.mixed": "混合",
  "type.all": "全部",
  "status.loading": "加载中...",
  "status.saving": "保存中...",
  "status.exporting": "导出中...",
  "unit.items": "条",
  "unit.chars": "字"
}
```

- [ ] **Step 4: Create Chinese settings.json**

Create `src/locales/zh-CN/settings.json`:

```json
{
  "cat.appearance": "外观",
  "cat.capture": "采集",
  "cat.insight": "洞察",
  "cat.ai": "AI",
  "cat.connect": "连接",
  "cat.storage": "存储",
  "version": "OpenWiki v0.1.0",
  "lang.label": "语言 / Language",
  "appearance.title": "外观",
  "appearance.theme": "主题模式",
  "theme.light": "浅色",
  "theme.dark": "深色",
  "theme.system": "跟随系统",
  "capture.title": "采集",
  "capture.enabled": "内容捕获",
  "capture.enabledDesc": "开启后将自动检测剪贴板和截图变化",
  "capture.mode": "捕获模式",
  "capture.modeDesc": "自动保存所有内容，或逐个确认",
  "capture.modeConfirm": "确认",
  "capture.modeAuto": "自动",
  "capture.defaultAction": "默认操作",
  "capture.defaultActionDesc": "确认弹窗倒计时结束后的默认行为",
  "capture.actionDismiss": "丢弃",
  "capture.actionSave": "保存",
  "capture.bubbleStyle": "悬浮球样式",
  "capture.styleCircle": "圆形",
  "capture.styleBar": "长条",
  "capture.bubblePosition": "悬浮球位置",
  "capture.posBottomRight": "右下",
  "capture.posBottomCenter": "下方居中",
  "capture.posBottomLeft": "左下",
  "capture.posTopRight": "右上",
  "capture.posTopCenter": "上方居中",
  "capture.posTopLeft": "左上",
  "capture.countdown": "确认倒计时",
  "capture.countdownDesc": "悬浮球自动消失的等待时间",
  "capture.countdownUnit": "秒",
  "capture.sensitiveFilter": "敏感数据过滤",
  "capture.sensitiveFilterDesc": "自动过滤密码、私钥、API Key 等",
  "capture.urlReading": "链接内容读取",
  "capture.urlReadingDesc": "复制链接时自动获取网页正文",
  "insight.title": "洞察",
  "insight.frequency": "分析频率",
  "insight.frequencyDesc": "洞察报告自动分析的间隔时间",
  "insight.freqDaily": "每天",
  "insight.freqThreeDays": "每 3 天",
  "insight.freqWeekly": "每周",
  "insight.freqMonthly": "每月",
  "insight.hint": "洞察页右上角的刷新按钮可以随时手动触发分析",
  "ai.title": "AI 配置",
  "ai.provider": "AI 提供商",
  "ai.model": "模型",
  "ai.accountLogin": "账号登录",
  "ai.useSubscription": "使用 ChatGPT 订阅额度",
  "ai.loggedIn": "✓ 已登录",
  "ai.logout": "退出登录",
  "ai.loginOpenAI": "登录 OpenAI 账号",
  "ai.waitingAuth": "等待浏览器授权...",
  "ai.loginOpenAIHint": "登录后无需 API Key，直接使用 ChatGPT 订阅额度",
  "ai.freeGemini": "免费使用 Gemini 模型",
  "ai.loginGoogle": "登录 Google 账号",
  "ai.loginGoogleHint": "登录后无需 API Key，免费使用 Gemini 模型",
  "ai.apiKey": "API Key",
  "ai.apiKeySecure": "安全存储在本地",
  "ai.apiKeyPlaceholder": "输入你的 API Key",
  "ai.hide": "隐藏",
  "ai.show": "显示",
  "ai.saved": "✓ 已保存",
  "ai.testConnection": "测试连接",
  "ai.testing": "测试中...",
  "ai.testSuccess": "✓ 连接成功：",
  "provider.anthropic": "Anthropic",
  "provider.openai": "OpenAI",
  "provider.openrouter": "OpenRouter",
  "provider.dashscope": "阿里云百炼",
  "provider.google": "Google",
  "provider.minimax": "MiniMax",
  "modelGroup.freeRecommended": "免费推荐",
  "modelGroup.moreFree": "更多免费",
  "modelGroup.zhipu": "智谱",
  "modelHint.auto": "Auto (智能选择)",
  "modelHint.deepThinking": "深度推理",
  "modelHint.longContext": "100万上下文",
  "connect.title": "AI 助理连接",
  "connect.claudeDesktop": "Claude Desktop",
  "connect.askInClaude": "在 Claude 中问",
  "connect.connectedDesc": "已连接 — Claude Desktop 可以读取你保存的内容",
  "connect.disconnectedDesc": "一键让 Claude Desktop 读取你的数据",
  "connect.connected": "已连接",
  "connect.disconnected": "未连接",
  "connect.disconnect": "断开连接",
  "connect.connecting": "连接中...",
  "connect.processing": "处理中...",
  "connect.connectButton": "连接 Claude Desktop",
  "connect.summary": "内容摘要",
  "connect.summaryDesc": "复制最近 7 天的内容摘要，粘贴给 AI 助理",
  "connect.copySummary": "复制最近内容摘要",
  "connect.copied": "✓ 已复制到剪贴板",
  "storage.title": "存储",
  "storage.savedItems": "已保存内容",
  "storage.diskUsage": "磁盘占用",
  "storage.screenshotDir": "截图目录"
}
```

- [ ] **Step 5: Create Chinese content.json**

Create `src/locales/zh-CN/content.json`:

```json
{
  "filter.all": "全部",
  "filter.text": "文本",
  "filter.image": "图片",
  "filter.link": "链接",
  "date.all": "全部",
  "date.today": "今天",
  "date.week": "近一周",
  "date.halfMonth": "半个月",
  "export.confirm": "确认导出？",
  "export.exporting": "导出中...",
  "export.done": "✓ 已导出",
  "export.button": "↗ 导出",
  "status.capturing": "捕获中",
  "status.paused": "已暂停",
  "empty.title": "还没有保存任何内容",
  "empty.desc": "复制文本或截图后会自动保存到这里",
  "captureOn": "内容捕获已开启",
  "captureOff": "内容捕获已关闭",
  "noTypeContent": "暂无{{type}}类型的内容",
  "card.reading": "读取中",
  "card.readFailed": "读取失败",
  "card.retrying": "重试中...",
  "card.ocrChars": "识别了{{count}}字",
  "card.ocring": "正在识别文字...",
  "card.ocrFailed": "识别失败:",
  "card.noContent": "无内容",
  "imagePreview.hint": "滚轮缩放 · 拖拽平移 · R 重置 · Esc 关闭",
  "bubble.screenshot": "📷 截图 / 图片",
  "bubble.saving": "保存中...",
  "bubble.clickSave": "· 点击保存",
  "bubble.notePlaceholder": "添加备注，回车保存…",
  "bubble.waiting": "等待内容…"
}
```

- [ ] **Step 6: Create Chinese wiki.json**

Create `src/locales/zh-CN/wiki.json`:

```json
{
  "title": "知识库",
  "stats": "{{pages}}个知识页面 · {{edges}}个关联 · {{sources}}条来源",
  "pendingUpdates": "{{count}}个待更新",
  "btn.ask": "提问",
  "btn.browse": "浏览",
  "btn.graph": "图谱",
  "browse.filterAll": "全部",
  "browse.filterConcept": "概念",
  "browse.filterEntity": "实体",
  "browse.filterSource": "来源",
  "browse.filterComparison": "对比",
  "browse.filterOverview": "总览",
  "browse.emptyTitle": "知识库还是空的",
  "browse.emptyDesc": "捕获的内容会自动编译成知识页面，或在内容列表中点击「加入知识库」",
  "detail.staleWarning": "⚠ 部分来源已失效，正文待更新",
  "detail.confirmDelete": "确认删除",
  "detail.deleteTooltip": "删除页面",
  "detail.sourcesTitle": "基于以下内容编译",
  "detail.noSources": "无来源记录",
  "detail.contentDeleted": "内容已删除",
  "detail.unknownApp": "未知",
  "detail.confidence": "置信度",
  "detail.compiledAt": "编译于{{time}} · {{count}}个来源",
  "detail.notCompiled": "未编译",
  "card.concept": "概念",
  "card.entity": "实体",
  "card.source": "来源",
  "card.comparison": "对比",
  "card.overview": "总览",
  "card.stale": "⚠ 待更新",
  "ask.title": "知识问答",
  "ask.sessions": "对话列表",
  "ask.newSession": "新对话",
  "ask.emptySessions": "还没有对话记录",
  "ask.startAsking": "开始提问",
  "ask.defaultSessionName": "新对话",
  "ask.saveFailed": "保存失败:",
  "ask.requestFailed": "请求失败:",
  "graph.error": "图谱渲染出错",
  "graph.empty": "知识库还没有页面，暂无图谱可展示",
  "lint.title": "知识库健康",
  "lint.runCheck": "运行健康检查",
  "lint.healthy": "知识库状态良好，暂无问题",
  "lint.keep": "保留",
  "lint.recompile": "重编",
  "lint.delete": "删除"
}
```

- [ ] **Step 7: Create Chinese digest.json**

Create `src/locales/zh-CN/digest.json`:

```json
{
  "radar.title": "深度洞察",
  "radar.refresh": "刷新分析",
  "radar.subtitle": "AI 深度分析你的信息收藏行为",
  "radar.needAI": "需要配置 AI 服务",
  "radar.needAIDesc": "洞察报告需要 AI 来分析你的内容",
  "radar.needMore": "你离洞察只差几步",
  "radar.needMoreDesc": "继续保存你感兴趣的内容，积累到 5 条就能开始分析",
  "radar.scattered": "这两周比较分散",
  "radar.scatteredDesc": "没有特别集中的方向。继续保存，下次分析可能会有新发现。",
  "radar.loadError": "洞察加载出错",
  "radar.analysisError": "分析时出现错误",
  "radar.reanalyze": "重新分析",
  "radar.updating": "正在更新分析...",
  "radar.section.glance": "一眼看穿",
  "radar.section.glanceSub": "At a Glance",
  "radar.section.diet": "信息食谱",
  "radar.section.dietSub": "摄入结构",
  "radar.section.subconscious": "潜意识洞察",
  "radar.section.subconsciousSub": "没意识到的关注",
  "radar.section.graveyard": "收藏夹坟场",
  "radar.section.graveyardSub": "沉没风险",
  "radar.section.blindSpot": "知识空白",
  "radar.section.blindSpotSub": "被忽视的角度",
  "radar.section.action": "行动建议",
  "radar.section.actionSub": "可执行",
  "radar.section.heatmap": "时间热力图",
  "radar.section.heatmapSub": "每日分布",
  "radar.section.topics": "主题分布",
  "radar.section.verdict": "一句话总结",
  "radar.section.verdictSub": "Final Verdict",
  "digest.title": "本周消化",
  "digest.stats": "已消化{{done}}条 · 剩余{{remaining}}条",
  "digest.onboardTitle": "👋 欢迎来到「消化」",
  "digest.onboardDesc": "这里展示你最近一周保存的内容。逐张翻看，决定保留还是放手。",
  "digest.onboardButton": "知道了，开始消化",
  "digest.doneTitle": "本周内容消化完毕！",
  "digest.doneMessage": "你消化了{{count}}条内容",
  "digest.emptyTitle": "最近一周没有需要消化的内容",
  "digest.emptyDesc": "复制一些内容，小云会帮你记住",
  "digest.keep": "✓ 还要",
  "digest.drop": "✕ 放手",
  "digest.important": "★ 重要",
  "digest.digestedCount": "🔥 已消化{{count}}条",
  "digest.card.imageLabel": "[图片]",
  "digest.card.noContent": "无内容",
  "insight.back": "返回雷达",
  "insight.coreInterest": "核心关注",
  "insight.emergingInterest": "新兴关注",
  "insight.meta": "{{count}}条内容 · 持续{{days}}天",
  "insight.findings": "核心发现",
  "insight.suggestions": "建议",
  "insight.relatedContent": "相关内容 · {{count}}条"
}
```

- [ ] **Step 8: Create Chinese datahub.json**

Create `src/locales/zh-CN/datahub.json`:

```json
{
  "day.welcomeTitle": "选择左侧日期查看内容",
  "day.welcomeDesc": "浏览和管理你的所有历史数据",
  "day.totalItems": "总条目",
  "day.activeDays": "活跃天数",
  "day.itemCount": "{{count}}条内容",
  "day.exportDay": "导出此日",
  "day.empty": "这一天没有记录的内容",
  "day.month": "{{month}}月",
  "day.dateFormat": "{{day}}日 星期{{weekday}}",
  "sidebar.stats": "{{items}}条 · {{days}}天",
  "sidebar.noData": "暂无数据",
  "sidebar.dateFormat": "{{day}}日 周{{weekday}}",
  "sidebar.exportSettings": "导出设置",
  "sidebar.openFolder": "打开文件夹",
  "export.title": "导出设置",
  "export.directory": "导出目录",
  "export.notSet": "未设置",
  "export.exportAll": "导出全部",
  "export.exporting": "导出中...",
  "export.done": "已导出{{count}}个文件",
  "export.openFinder": "在 Finder 中打开",
  "report.title": "周报",
  "report.history": "历史",
  "report.generating": "生成中",
  "report.generate": "生成",
  "report.loadError": "加载周报失败，请稍后重试",
  "report.generateError": "生成周报失败，请确认已配置 AI 服务后重试",
  "report.viewSource": "查看原文",
  "report.relatedContent": "相关内容",
  "report.relatedCount": "{{count}}条",
  "report.interested": "感兴趣",
  "report.understood": "已了解",
  "report.noted": "已记录",
  "report.noTextContent": "本周没有文本内容",
  "report.textCount": "{{count}}条文本",
  "report.noLinkContent": "本周没有链接内容",
  "report.linkCount": "{{count}}个链接",
  "report.unknownLink": "未知链接",
  "report.noImageContent": "本周没有图片内容",
  "report.imageCount": "{{count}}张图片",
  "report.weekday": "周{{day}}"
}
```

- [ ] **Step 9: Create English common.json**

Create `src/locales/en/common.json`:

```json
{
  "brand": "OpenWiki",
  "nav.content": "Content",
  "nav.wiki": "Wiki",
  "nav.digest": "Insights",
  "nav.settings": "Settings",
  "search.placeholder": "Search...",
  "search.searching": "Searching...",
  "search.noResults": "No results",
  "search.wikiPages": "Wiki Pages",
  "search.capturedContent": "Captured Content",
  "search.noContent": "No content",
  "search.tooltip": "Search",
  "btn.save": "Save",
  "btn.cancel": "Cancel",
  "btn.delete": "Delete",
  "btn.confirm": "Confirm",
  "btn.dismiss": "Dismiss",
  "btn.retry": "Retry",
  "btn.export": "Export",
  "btn.expand": "Expand",
  "btn.collapse": "Collapse",
  "time.justNow": "Just now",
  "time.minutesAgo": "{{count}}m ago",
  "time.hoursAgo": "{{count}}h ago",
  "time.daysAgo": "{{count}}d ago",
  "time.monthsAgo": "{{count}}mo ago",
  "time.yearsAgo": "{{count}}y ago",
  "time.today": "Today",
  "time.yesterday": "Yesterday",
  "type.text": "Text",
  "type.image": "Image",
  "type.link": "Link",
  "type.mixed": "Mixed",
  "type.all": "All",
  "status.loading": "Loading...",
  "status.saving": "Saving...",
  "status.exporting": "Exporting...",
  "unit.items": "",
  "unit.chars": " chars"
}
```

- [ ] **Step 10: Create English settings.json**

Create `src/locales/en/settings.json`:

```json
{
  "cat.appearance": "Appearance",
  "cat.capture": "Capture",
  "cat.insight": "Insights",
  "cat.ai": "AI",
  "cat.connect": "Connect",
  "cat.storage": "Storage",
  "version": "OpenWiki v0.1.0",
  "lang.label": "语言 / Language",
  "appearance.title": "Appearance",
  "appearance.theme": "Theme",
  "theme.light": "Light",
  "theme.dark": "Dark",
  "theme.system": "System",
  "capture.title": "Capture",
  "capture.enabled": "Content Capture",
  "capture.enabledDesc": "Auto-detect clipboard and screenshot changes",
  "capture.mode": "Capture Mode",
  "capture.modeDesc": "Auto-save all content or confirm each one",
  "capture.modeConfirm": "Confirm",
  "capture.modeAuto": "Auto",
  "capture.defaultAction": "Default Action",
  "capture.defaultActionDesc": "Default behavior when confirmation countdown ends",
  "capture.actionDismiss": "Dismiss",
  "capture.actionSave": "Save",
  "capture.bubbleStyle": "Bubble Style",
  "capture.styleCircle": "Circle",
  "capture.styleBar": "Bar",
  "capture.bubblePosition": "Bubble Position",
  "capture.posBottomRight": "Bottom Right",
  "capture.posBottomCenter": "Bottom Center",
  "capture.posBottomLeft": "Bottom Left",
  "capture.posTopRight": "Top Right",
  "capture.posTopCenter": "Top Center",
  "capture.posTopLeft": "Top Left",
  "capture.countdown": "Confirmation Countdown",
  "capture.countdownDesc": "Wait time before bubble auto-dismisses",
  "capture.countdownUnit": "sec",
  "capture.sensitiveFilter": "Sensitive Data Filter",
  "capture.sensitiveFilterDesc": "Auto-filter passwords, private keys, API keys, etc.",
  "capture.urlReading": "Link Content Reading",
  "capture.urlReadingDesc": "Auto-fetch webpage content when copying links",
  "insight.title": "Insights",
  "insight.frequency": "Analysis Frequency",
  "insight.frequencyDesc": "Interval for automatic insight analysis",
  "insight.freqDaily": "Daily",
  "insight.freqThreeDays": "Every 3 days",
  "insight.freqWeekly": "Weekly",
  "insight.freqMonthly": "Monthly",
  "insight.hint": "Use the refresh button on the Insights page to trigger analysis anytime",
  "ai.title": "AI Configuration",
  "ai.provider": "AI Provider",
  "ai.model": "Model",
  "ai.accountLogin": "Account Login",
  "ai.useSubscription": "Use ChatGPT subscription quota",
  "ai.loggedIn": "✓ Logged in",
  "ai.logout": "Log out",
  "ai.loginOpenAI": "Log in with OpenAI",
  "ai.waitingAuth": "Waiting for browser auth...",
  "ai.loginOpenAIHint": "No API Key needed — uses your ChatGPT subscription",
  "ai.freeGemini": "Use Gemini models for free",
  "ai.loginGoogle": "Log in with Google",
  "ai.loginGoogleHint": "No API Key needed — use Gemini models for free",
  "ai.apiKey": "API Key",
  "ai.apiKeySecure": "Stored securely on your device",
  "ai.apiKeyPlaceholder": "Enter your API Key",
  "ai.hide": "Hide",
  "ai.show": "Show",
  "ai.saved": "✓ Saved",
  "ai.testConnection": "Test Connection",
  "ai.testing": "Testing...",
  "ai.testSuccess": "✓ Connected: ",
  "provider.anthropic": "Anthropic",
  "provider.openai": "OpenAI",
  "provider.openrouter": "OpenRouter",
  "provider.dashscope": "Alibaba DashScope",
  "provider.google": "Google",
  "provider.minimax": "MiniMax",
  "modelGroup.freeRecommended": "Free (Recommended)",
  "modelGroup.moreFree": "More Free",
  "modelGroup.zhipu": "Zhipu",
  "modelHint.auto": "Auto (Smart Select)",
  "modelHint.deepThinking": "Deep Thinking",
  "modelHint.longContext": "1M context",
  "connect.title": "AI Assistant Connection",
  "connect.claudeDesktop": "Claude Desktop",
  "connect.askInClaude": "Ask in Claude",
  "connect.connectedDesc": "Connected — Claude Desktop can read your saved content",
  "connect.disconnectedDesc": "Let Claude Desktop read your data with one click",
  "connect.connected": "Connected",
  "connect.disconnected": "Not connected",
  "connect.disconnect": "Disconnect",
  "connect.connecting": "Connecting...",
  "connect.processing": "Processing...",
  "connect.connectButton": "Connect Claude Desktop",
  "connect.summary": "Content Summary",
  "connect.summaryDesc": "Copy a summary of recent 7-day content to paste to AI assistant",
  "connect.copySummary": "Copy Recent Summary",
  "connect.copied": "✓ Copied to clipboard",
  "storage.title": "Storage",
  "storage.savedItems": "Saved Items",
  "storage.diskUsage": "Disk Usage",
  "storage.screenshotDir": "Screenshot Directory"
}
```

- [ ] **Step 11: Create English content.json**

Create `src/locales/en/content.json`:

```json
{
  "filter.all": "All",
  "filter.text": "Text",
  "filter.image": "Image",
  "filter.link": "Link",
  "date.all": "All",
  "date.today": "Today",
  "date.week": "Past week",
  "date.halfMonth": "Past 2 weeks",
  "export.confirm": "Confirm export?",
  "export.exporting": "Exporting...",
  "export.done": "✓ Exported",
  "export.button": "↗ Export",
  "status.capturing": "Capturing",
  "status.paused": "Paused",
  "empty.title": "No content saved yet",
  "empty.desc": "Copy text or take screenshots — they'll be saved here automatically",
  "captureOn": "Content capture is on",
  "captureOff": "Content capture is off",
  "noTypeContent": "No {{type}} content yet",
  "card.reading": "Reading",
  "card.readFailed": "Read failed",
  "card.retrying": "Retrying...",
  "card.ocrChars": "Recognized {{count}} chars",
  "card.ocring": "Recognizing text...",
  "card.ocrFailed": "Recognition failed:",
  "card.noContent": "No content",
  "imagePreview.hint": "Scroll to zoom · Drag to pan · R to reset · Esc to close",
  "bubble.screenshot": "📷 Screenshot / Image",
  "bubble.saving": "Saving...",
  "bubble.clickSave": "· Click to save",
  "bubble.notePlaceholder": "Add a note, press Enter to save…",
  "bubble.waiting": "Waiting for content…"
}
```

- [ ] **Step 12: Create English wiki.json**

Create `src/locales/en/wiki.json`:

```json
{
  "title": "Wiki",
  "stats": "{{pages}} pages · {{edges}} links · {{sources}} sources",
  "pendingUpdates": "{{count}} pending updates",
  "btn.ask": "Ask",
  "btn.browse": "Browse",
  "btn.graph": "Graph",
  "browse.filterAll": "All",
  "browse.filterConcept": "Concept",
  "browse.filterEntity": "Entity",
  "browse.filterSource": "Source",
  "browse.filterComparison": "Compare",
  "browse.filterOverview": "Overview",
  "browse.emptyTitle": "Wiki is empty",
  "browse.emptyDesc": "Captured content will be compiled into wiki pages, or click \"Add to Wiki\" in the content list",
  "detail.staleWarning": "⚠ Some sources are stale — content needs update",
  "detail.confirmDelete": "Confirm Delete",
  "detail.deleteTooltip": "Delete page",
  "detail.sourcesTitle": "Compiled from",
  "detail.noSources": "No source records",
  "detail.contentDeleted": "Content deleted",
  "detail.unknownApp": "Unknown",
  "detail.confidence": "Confidence",
  "detail.compiledAt": "Compiled {{time}} · {{count}} sources",
  "detail.notCompiled": "Not compiled",
  "card.concept": "Concept",
  "card.entity": "Entity",
  "card.source": "Source",
  "card.comparison": "Compare",
  "card.overview": "Overview",
  "card.stale": "⚠ Needs update",
  "ask.title": "Wiki Q&A",
  "ask.sessions": "Sessions",
  "ask.newSession": "New Session",
  "ask.emptySessions": "No sessions yet",
  "ask.startAsking": "Start asking",
  "ask.defaultSessionName": "New Session",
  "ask.saveFailed": "Save failed:",
  "ask.requestFailed": "Request failed:",
  "graph.error": "Graph rendering error",
  "graph.empty": "No wiki pages yet — nothing to show",
  "lint.title": "Wiki Health",
  "lint.runCheck": "Run health check",
  "lint.healthy": "Wiki is healthy — no issues found",
  "lint.keep": "Keep",
  "lint.recompile": "Recompile",
  "lint.delete": "Delete"
}
```

- [ ] **Step 13: Create English digest.json**

Create `src/locales/en/digest.json`:

```json
{
  "radar.title": "Deep Insights",
  "radar.refresh": "Refresh Analysis",
  "radar.subtitle": "AI-powered analysis of your information habits",
  "radar.needAI": "AI service required",
  "radar.needAIDesc": "Insights need AI to analyze your content",
  "radar.needMore": "Almost there",
  "radar.needMoreDesc": "Keep saving content you find interesting — analysis starts at 5 items",
  "radar.scattered": "Scattered interests",
  "radar.scatteredDesc": "No strong focus this period. Keep saving — patterns may emerge next time.",
  "radar.loadError": "Failed to load insights",
  "radar.analysisError": "Analysis encountered an error",
  "radar.reanalyze": "Re-analyze",
  "radar.updating": "Updating analysis...",
  "radar.section.glance": "At a Glance",
  "radar.section.glanceSub": "Quick Overview",
  "radar.section.diet": "Info Diet",
  "radar.section.dietSub": "Intake Structure",
  "radar.section.subconscious": "Subconscious Insights",
  "radar.section.subconsciousSub": "Hidden Interests",
  "radar.section.graveyard": "Bookmark Graveyard",
  "radar.section.graveyardSub": "Sunk Cost Risk",
  "radar.section.blindSpot": "Knowledge Gaps",
  "radar.section.blindSpotSub": "Overlooked Angles",
  "radar.section.action": "Action Items",
  "radar.section.actionSub": "Actionable",
  "radar.section.heatmap": "Time Heatmap",
  "radar.section.heatmapSub": "Daily Distribution",
  "radar.section.topics": "Topic Distribution",
  "radar.section.verdict": "One-Line Summary",
  "radar.section.verdictSub": "Final Verdict",
  "digest.title": "Weekly Digest",
  "digest.stats": "Digested {{done}} · {{remaining}} remaining",
  "digest.onboardTitle": "👋 Welcome to Digest",
  "digest.onboardDesc": "Review your saved content from this week. Swipe through and decide what to keep.",
  "digest.onboardButton": "Got it, let's go",
  "digest.doneTitle": "All caught up!",
  "digest.doneMessage": "You digested {{count}} items",
  "digest.emptyTitle": "Nothing to digest this week",
  "digest.emptyDesc": "Copy some content and OpenWiki will remember it for you",
  "digest.keep": "✓ Keep",
  "digest.drop": "✕ Drop",
  "digest.important": "★ Important",
  "digest.digestedCount": "🔥 Digested {{count}}",
  "digest.card.imageLabel": "[Image]",
  "digest.card.noContent": "No content",
  "insight.back": "Back to Insights",
  "insight.coreInterest": "Core Interest",
  "insight.emergingInterest": "Emerging Interest",
  "insight.meta": "{{count}} items · {{days}} days",
  "insight.findings": "Key Findings",
  "insight.suggestions": "Suggestions",
  "insight.relatedContent": "Related Content · {{count}}"
}
```

- [ ] **Step 14: Create English datahub.json**

Create `src/locales/en/datahub.json`:

```json
{
  "day.welcomeTitle": "Select a date to view content",
  "day.welcomeDesc": "Browse and manage all your historical data",
  "day.totalItems": "Total Items",
  "day.activeDays": "Active Days",
  "day.itemCount": "{{count}} items",
  "day.exportDay": "Export this day",
  "day.empty": "No content recorded on this day",
  "day.month": "{{month}}",
  "day.dateFormat": "{{day}}, {{weekday}}",
  "sidebar.stats": "{{items}} items · {{days}} days",
  "sidebar.noData": "No data",
  "sidebar.dateFormat": "{{day}} {{weekday}}",
  "sidebar.exportSettings": "Export Settings",
  "sidebar.openFolder": "Open Folder",
  "export.title": "Export Settings",
  "export.directory": "Export Directory",
  "export.notSet": "Not set",
  "export.exportAll": "Export All",
  "export.exporting": "Exporting...",
  "export.done": "Exported {{count}} files",
  "export.openFinder": "Open in Finder",
  "report.title": "Weekly Report",
  "report.history": "History",
  "report.generating": "Generating",
  "report.generate": "Generate",
  "report.loadError": "Failed to load report. Please try again later.",
  "report.generateError": "Failed to generate report. Please configure AI service first.",
  "report.viewSource": "View Source",
  "report.relatedContent": "Related Content",
  "report.relatedCount": "{{count}}",
  "report.interested": "Interested",
  "report.understood": "Got it",
  "report.noted": "Noted",
  "report.noTextContent": "No text content this week",
  "report.textCount": "{{count}} texts",
  "report.noLinkContent": "No links this week",
  "report.linkCount": "{{count}} links",
  "report.unknownLink": "Unknown link",
  "report.noImageContent": "No images this week",
  "report.imageCount": "{{count}} images",
  "report.weekday": "{{day}}"
}
```

- [ ] **Step 15: Create i18n initialization file**

Create `src/lib/i18n.ts`:

```typescript
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import zhCommon from '../locales/zh-CN/common.json';
import zhSettings from '../locales/zh-CN/settings.json';
import zhContent from '../locales/zh-CN/content.json';
import zhWiki from '../locales/zh-CN/wiki.json';
import zhDigest from '../locales/zh-CN/digest.json';
import zhDatahub from '../locales/zh-CN/datahub.json';

import enCommon from '../locales/en/common.json';
import enSettings from '../locales/en/settings.json';
import enContent from '../locales/en/content.json';
import enWiki from '../locales/en/wiki.json';
import enDigest from '../locales/en/digest.json';
import enDatahub from '../locales/en/datahub.json';

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      'zh-CN': {
        common: zhCommon,
        settings: zhSettings,
        content: zhContent,
        wiki: zhWiki,
        digest: zhDigest,
        datahub: zhDatahub,
      },
      en: {
        common: enCommon,
        settings: enSettings,
        content: enContent,
        wiki: enWiki,
        digest: enDigest,
        datahub: enDatahub,
      },
    },
    fallbackLng: 'zh-CN',
    defaultNS: 'common',
    interpolation: {
      escapeValue: false,
    },
    detection: {
      order: ['localStorage', 'navigator'],
      lookupLocalStorage: 'openwiki-language',
      caches: ['localStorage'],
    },
  });

// Keep <html lang> in sync
i18n.on('languageChanged', (lng) => {
  document.documentElement.lang = lng;
});

export default i18n;
```

- [ ] **Step 16: Import i18n in main.tsx**

Modify `src/main.tsx` — add this import as the first line (before all other imports):

```typescript
import './lib/i18n';
```

The full file becomes:

```typescript
import './lib/i18n';
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import SpotlightView from './features/spotlight/SpotlightView.tsx'
import BubbleView from './components/BubbleView.tsx'

// ... rest unchanged
```

- [ ] **Step 17: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

Expected: Build succeeds with no errors. The i18n module is loaded but not yet used by any component.

- [ ] **Step 18: Commit**

```bash
git add src/lib/i18n.ts src/locales/ src/main.tsx package.json package-lock.json
git commit -m "feat(i18n): add react-i18next foundation with zh-CN/en translation files"
```

---

## Task 2: Migrate App.tsx — Nav Tabs, Search, Brand Rename

**Files:**
- Modify: `src/App.tsx`

- [ ] **Step 1: Add useTranslation and replace all hardcoded text**

Modify `src/App.tsx`:

1. Add import at top:
```typescript
import { useTranslation } from 'react-i18next';
```

2. Inside `function App()`, add at the top:
```typescript
const { t } = useTranslation();
```

3. Move `TABS` array inside the component (so it can use `t()`) and replace labels:
```typescript
const TABS: TabItem[] = [
  { id: "content", label: t('nav.content'), icon: ClipboardList },
  { id: "wiki", label: t('nav.wiki'), icon: BookOpen },
  { id: "digest", label: t('nav.digest'), icon: Target },
  { id: "settings", label: t('nav.settings'), icon: Settings },
];
```

4. Replace brand name "小云" → "OpenWiki":
```tsx
<span className="text-base font-bold text-orange-500 flex-shrink-0" data-tauri-drag-region>
  OpenWiki
</span>
```

5. Replace search placeholder:
```tsx
placeholder={t('search.placeholder')}
```

6. Replace search status text:
```tsx
// "搜索中..." →
{t('search.searching')}

// "无结果" →
{t('search.noResults')}

// "知识页面" →
{t('search.wikiPages')}

// "捕获内容" →
{t('search.capturedContent')}

// "无内容" →
{t('search.noContent')}
```

7. Replace search button title:
```tsx
title={t('search.tooltip')}
```

- [ ] **Step 2: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 3: Commit**

```bash
git add src/App.tsx
git commit -m "feat(i18n): migrate App.tsx — nav tabs, search, brand rename to OpenWiki"
```

---

## Task 3: Migrate SettingsView — Language Switcher + All Text

**Files:**
- Modify: `src/features/settings/SettingsView.tsx`
- Modify: `src/stores/settingsStore.ts`

This is the largest single file (~1000 lines). The migration replaces all Chinese text with `t()` calls and adds the language switcher.

- [ ] **Step 1: Migrate settingsStore.ts — move Chinese labels to i18n keys**

In `src/stores/settingsStore.ts`:

1. Change `PROVIDER_LABELS` to use translation keys instead of display strings:
```typescript
export const PROVIDER_LABELS: Record<AIProvider, string> = {
  anthropic: "provider.anthropic",
  openai: "provider.openai",
  openrouter: "provider.openrouter",
  dashscope: "provider.dashscope",
  google: "provider.google",
  minimax: "provider.minimax",
};
```

2. In `MODELS_BY_PROVIDER`, replace Chinese `group` values with i18n keys:
```typescript
// "免费推荐" → "modelGroup.freeRecommended"
// "更多免费" → "modelGroup.moreFree"
// "智谱" → "modelGroup.zhipu"
```

3. Replace Chinese model labels:
```typescript
// { id: "auto", label: "Auto (智能选择)" } → { id: "auto", label: "modelHint.auto" }
// "Gemini 3 Pro (深度推理)" → keep model name, change label to use key for the hint part
```

Note: Model names like "Claude Sonnet 4", "GPT-5.4" are proper names and stay unchanged. Only translate the Chinese parts: group names and hint text like "智能选择", "深度推理".

For models with Chinese hints in the label, store the base name and hint key separately:
```typescript
{ id: "auto", label: "Auto", labelKey: "modelHint.auto" }
{ id: "gemini-3-pro-high", label: "Gemini 3 Pro", labelKey: "modelHint.deepThinking" }
```

Update the `AIModelOption` interface:
```typescript
export interface AIModelOption {
  id: string;
  label: string;
  labelKey?: string; // i18n key for display — if set, use t(labelKey) instead of label
  free?: boolean;
  group?: string;    // now stores i18n key, not display string
}
```

- [ ] **Step 2: Migrate SettingsView.tsx — add language switcher and replace all text**

In `src/features/settings/SettingsView.tsx`:

1. Add import:
```typescript
import { useTranslation } from 'react-i18next';
```

2. Inside `SettingsView()`, add:
```typescript
const { t } = useTranslation('settings');
const { t: tc } = useTranslation('common');
```

3. Replace `BUBBLE_POSITION_OPTIONS` labels:
```typescript
const BUBBLE_POSITION_OPTIONS = [
  { value: "bottom-right" as BubblePosition, label: t('capture.posBottomRight'), icon: "↘" },
  { value: "bottom-center" as BubblePosition, label: t('capture.posBottomCenter'), icon: "↓" },
  // ... etc
];
```

4. Replace `THEME_OPTIONS` labels:
```typescript
const THEME_OPTIONS = [
  { value: "light" as ThemeMode, label: t('theme.light'), icon: "☀️" },
  { value: "dark" as ThemeMode, label: t('theme.dark'), icon: "🌙" },
  { value: "system" as ThemeMode, label: t('theme.system'), icon: "💻" },
];
```

5. Move these arrays inside the component function (after the `useTranslation` calls) since they now depend on `t()`.

6. Add language switcher as the first item in the Appearance section, before theme:
```tsx
{/* Language — always bilingual label */}
<div className="flex items-center justify-between">
  <span className="text-sm text-gray-700 dark:text-gray-300">
    {t('lang.label')}
  </span>
  <div className="flex bg-gray-100/80 dark:bg-white/[0.08] rounded-lg p-0.5">
    {[
      { value: 'zh-CN', label: '中文' },
      { value: 'en', label: 'English' },
    ].map((opt) => (
      <button
        key={opt.value}
        onClick={() => i18n.changeLanguage(opt.value)}
        className={`px-3 py-1 text-xs font-medium rounded-md transition-all ${
          (i18n.language.startsWith('zh') ? 'zh-CN' : 'en') === opt.value
            ? 'bg-white dark:bg-white/[0.15] text-orange-500 shadow-sm'
            : 'text-gray-500 dark:text-gray-400'
        }`}
      >
        {opt.label}
      </button>
    ))}
  </div>
</div>
```

Also add the `i18n` import for `changeLanguage`:
```typescript
import { useTranslation } from 'react-i18next';
// i18n instance is available from useTranslation:
const { t, i18n } = useTranslation('settings');
```

7. Replace all remaining Chinese strings. Every Chinese string in this file gets a `t('key')` call using keys from `settings.json`. Examples:

```tsx
// Category sidebar labels
// "外观" → t('cat.appearance')
// "采集" → t('cat.capture')
// etc.

// Section headings
// "外观" → t('appearance.title')
// "采集" → t('capture.title')

// Setting labels
// "内容捕获" → t('capture.enabled')
// "开启后将自动检测剪贴板和截图变化" → t('capture.enabledDesc')

// Buttons
// "保存" → tc('btn.save')  (from common namespace)
// "隐藏" → t('ai.hide')
// "显示" → t('ai.show')

// Status messages
// "✓ 已保存" → t('ai.saved')
// "测试中..." → t('ai.testing')
// "等待浏览器授权..." → t('ai.waitingAuth')

// Version
// "小云 v0.1.0" → t('version')

// Provider labels (now use t() to resolve the key)
// PROVIDER_LABELS[provider] was "阿里云百炼", now it's "provider.dashscope"
// Render as: t(PROVIDER_LABELS[provider])

// Model groups (similar approach)
// model.group was "免费推荐", now it's "modelGroup.freeRecommended"
// Render as: t(model.group)

// Model labels with labelKey
// Render as: model.labelKey ? t(model.labelKey) : model.label
```

8. For the storage section:
```tsx
// "已保存内容" → t('storage.savedItems')
// "条" — for item count, use interpolation or just append tc('unit.items')
// "磁盘占用" → t('storage.diskUsage')
// "截图目录" → t('storage.screenshotDir')
```

- [ ] **Step 3: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 4: Commit**

```bash
git add src/features/settings/SettingsView.tsx src/stores/settingsStore.ts
git commit -m "feat(i18n): migrate Settings — language switcher + all settings text"
```

---

## Task 4: Migrate Content Module

**Files:**
- Modify: `src/features/content-list/ContentList.tsx`
- Modify: `src/features/content-list/ContentCard.tsx`
- Modify: `src/features/content-list/ImagePreview.tsx`
- Modify: `src/components/FloatingBubble.tsx`
- Modify: `src/components/BubbleView.tsx`
- Modify: `src/features/spotlight/SpotlightView.tsx`

- [ ] **Step 1: Migrate ContentList.tsx**

Add `import { useTranslation } from 'react-i18next';` and `const { t } = useTranslation('content');` inside the component.

Replace all Chinese strings:

```tsx
// Filter buttons
// "全部" → t('filter.all')
// "文本" → t('filter.text')
// "图片" → t('filter.image')
// "链接" → t('filter.link')

// Date filters
// "全部" → t('date.all')
// "今天" → t('date.today')
// "近一周" → t('date.week')
// "半个月" → t('date.halfMonth')

// Export
// "确认导出？" → t('export.confirm')
// "导出中..." → t('export.exporting')
// "✓ 已导出" → t('export.done')
// "↗ 导出" → t('export.button')

// Status
// "捕获中" → t('status.capturing')
// "已暂停" → t('status.paused')

// Empty state
// "还没有保存任何内容" → t('empty.title')
// "复制文本或截图后会自动保存到这里" → t('empty.desc')
// "内容捕获已开启" → t('captureOn')
// "内容捕获已关闭" → t('captureOff')
// "暂无...类型的内容" → t('noTypeContent', { type: filterLabel })
```

- [ ] **Step 2: Migrate ContentCard.tsx**

Add `useTranslation` with both `content` and `common` namespaces.

Replace Chinese strings:

```tsx
// Type labels: use tc('type.text'), tc('type.image'), etc. from common
// Time expressions: use tc('time.justNow'), tc('time.minutesAgo', { count }), etc. from common

// Content-specific:
// "读取中" → t('card.reading')
// "读取失败" → t('card.readFailed')
// "重试中..." → t('card.retrying')
// "重试" → tc('btn.retry')
// "识别了...字" → t('card.ocrChars', { count: charCount })
// "正在识别文字..." → t('card.ocring')
// "无内容" → t('card.noContent')
// "识别失败:" → t('card.ocrFailed')
```

- [ ] **Step 3: Migrate ImagePreview.tsx**

```tsx
// "滚轮缩放 · 拖拽平移 · R 重置 · Esc 关闭" → t('imagePreview.hint')
```

- [ ] **Step 4: Migrate FloatingBubble.tsx**

Add `useTranslation('content')`.

```tsx
// "📷 截图 / 图片" → t('bubble.screenshot')
// "保存中..." → t('bubble.saving')
// "· 点击保存" → t('bubble.clickSave')
```

- [ ] **Step 5: Migrate BubbleView.tsx**

Add `useTranslation('content')`.

```tsx
// "添加备注，回车保存…" → t('bubble.notePlaceholder')
// "等待内容…" → t('bubble.waiting')
```

- [ ] **Step 6: Migrate SpotlightView.tsx**

Add `useTranslation('content')`.

```tsx
// "添加备注，回车保存…" → t('bubble.notePlaceholder')
// "等待内容…" → t('bubble.waiting')
```

- [ ] **Step 7: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 8: Commit**

```bash
git add src/features/content-list/ src/components/ src/features/spotlight/
git commit -m "feat(i18n): migrate content module — list, cards, bubble, spotlight"
```

---

## Task 5: Migrate Wiki Module

**Files:**
- Modify: `src/features/wiki/WikiView.tsx`
- Modify: `src/features/wiki/WikiBrowseView.tsx`
- Modify: `src/features/wiki/WikiPageDetail.tsx`
- Modify: `src/features/wiki/WikiPageCard.tsx`
- Modify: `src/features/wiki/WikiAskSidebar.tsx`
- Modify: `src/features/wiki/WikiGraphView.tsx`
- Modify: `src/features/wiki/WikiLintSection.tsx`

- [ ] **Step 1: Migrate WikiView.tsx**

Add `useTranslation('wiki')`.

```tsx
// "知识库" → t('title')
// "个知识页面 · 个关联 · 条来源" → t('stats', { pages, edges, sources })
// "个待更新" → t('pendingUpdates', { count })
// "提问" → t('btn.ask')
// "浏览" → t('btn.browse')
// "图谱" → t('btn.graph')
```

- [ ] **Step 2: Migrate WikiBrowseView.tsx**

```tsx
// "全部" → t('browse.filterAll')
// "概念" → t('browse.filterConcept')
// "实体" → t('browse.filterEntity')
// "来源" → t('browse.filterSource')
// "对比" → t('browse.filterComparison')
// "总览" → t('browse.filterOverview')
// "知识库还是空的" → t('browse.emptyTitle')
// "捕获的内容会自动编译..." → t('browse.emptyDesc')
```

- [ ] **Step 3: Migrate WikiPageDetail.tsx**

```tsx
// "⚠ 部分来源已失效，正文待更新" → t('detail.staleWarning')
// "确认删除" → t('detail.confirmDelete')
// "取消" → tc('btn.cancel')
// "删除页面" → t('detail.deleteTooltip')
// "基于以下内容编译" → t('detail.sourcesTitle')
// "加载中..." → tc('status.loading')
// "无来源记录" → t('detail.noSources')
// "内容已删除" → t('detail.contentDeleted')
// "未知" → t('detail.unknownApp')
// "置信度" → t('detail.confidence')
// "编译于...· ...个来源" → t('detail.compiledAt', { time, count })
// "未编译" → t('detail.notCompiled')
```

- [ ] **Step 4: Migrate WikiPageCard.tsx**

```tsx
// Type labels: t('card.concept'), t('card.entity'), etc.
// Time expressions: use tc() from common namespace
// "⚠ 待更新" → t('card.stale')
```

- [ ] **Step 5: Migrate WikiAskSidebar.tsx**

```tsx
// "知识问答" → t('ask.title')
// "对话列表" → t('ask.sessions')
// "新对话" → t('ask.newSession')
// "还没有对话记录" → t('ask.emptySessions')
// "开始提问" → t('ask.startAsking')
// "新对话" (default name) → t('ask.defaultSessionName')
// "保存失败:" → t('ask.saveFailed')
// "请求失败:" → t('ask.requestFailed')
```

- [ ] **Step 6: Migrate WikiGraphView.tsx**

```tsx
// "图谱渲染出错" → t('graph.error')
// "重试" → tc('btn.retry')
// "知识库还没有页面，暂无图谱可展示" → t('graph.empty')
```

- [ ] **Step 7: Migrate WikiLintSection.tsx**

```tsx
// "知识库健康" → t('lint.title')
// "运行健康检查" → t('lint.runCheck')
// "知识库状态良好，暂无问题" → t('lint.healthy')
// "保留" → t('lint.keep')
// "重编" → t('lint.recompile')
// "删除" → t('lint.delete')
```

- [ ] **Step 8: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 9: Commit**

```bash
git add src/features/wiki/
git commit -m "feat(i18n): migrate wiki module — browse, detail, ask, graph, lint"
```

---

## Task 6: Migrate Digest Module

**Files:**
- Modify: `src/features/digest/RadarView.tsx`
- Modify: `src/features/digest/DigestView.tsx`
- Modify: `src/features/digest/DigestCard.tsx`
- Modify: `src/features/digest/InsightDetail.tsx`

- [ ] **Step 1: Migrate RadarView.tsx**

Add `useTranslation('digest')`.

Replace all Chinese strings with `t('radar.xxx')` keys from digest.json. This file has the most text — all section titles and subtitles, empty states, error messages, and status text.

```tsx
// "深度洞察" → t('radar.title')
// "刷新分析" → t('radar.refresh')
// "AI 深度分析你的信息收藏行为" → t('radar.subtitle')
// "需要配置 AI 服务" → t('radar.needAI')
// etc. — all keys listed in digest.json under radar.*
```

- [ ] **Step 2: Migrate DigestView.tsx**

```tsx
// "本周消化" → t('digest.title')
// "已消化...条 · 剩余...条" → t('digest.stats', { done, remaining })
// "👋 欢迎来到「消化」" → t('digest.onboardTitle')
// "这里展示你最近一周保存的内容..." → t('digest.onboardDesc')
// "知道了，开始消化" → t('digest.onboardButton')
// "本周内容消化完毕！" → t('digest.doneTitle')
// "你消化了...条内容" → t('digest.doneMessage', { count })
// "最近一周没有需要消化的内容" → t('digest.emptyTitle')
// "复制一些内容，小云会帮你记住" → t('digest.emptyDesc')
// "✓ 还要" → t('digest.keep')
// "✕ 放手" → t('digest.drop')
// "★ 重要" → t('digest.important')
// "🔥 已消化...条" → t('digest.digestedCount', { count })
```

- [ ] **Step 3: Migrate DigestCard.tsx**

```tsx
// Time expressions: use tc() from common
// "[图片]" → t('digest.card.imageLabel')
// "无内容" → t('digest.card.noContent')
```

- [ ] **Step 4: Migrate InsightDetail.tsx**

```tsx
// "返回雷达" → t('insight.back')
// "核心关注" → t('insight.coreInterest')
// "新兴关注" → t('insight.emergingInterest')
// "条内容 · 持续...天" → t('insight.meta', { count, days })
// "核心发现" → t('insight.findings')
// "建议" → t('insight.suggestions')
// "相关内容 · ...条" → t('insight.relatedContent', { count })
```

- [ ] **Step 5: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 6: Commit**

```bash
git add src/features/digest/
git commit -m "feat(i18n): migrate digest module — radar, digest, insight views"
```

---

## Task 7: Migrate DataHub + Weekly Report Module

**Files:**
- Modify: `src/features/data-hub/DayDetail.tsx`
- Modify: `src/features/data-hub/DateSidebar.tsx`
- Modify: `src/features/data-hub/ExportPanel.tsx`
- Modify: `src/features/weekly-report/ReportView.tsx`
- Modify: `src/features/weekly-report/ReportCard.tsx`
- Modify: `src/features/weekly-report/ContentPreviewPanel.tsx`
- Modify: `src/features/weekly-report/FeedbackButtons.tsx`
- Modify: `src/features/weekly-report/TextContentList.tsx`
- Modify: `src/features/weekly-report/CompactLinkList.tsx`
- Modify: `src/features/weekly-report/ActivityStatsCard.tsx`
- Modify: `src/features/weekly-report/ImageFilmstrip.tsx`

- [ ] **Step 1: Migrate DayDetail.tsx**

Add `useTranslation('datahub')`.

```tsx
// Type labels: use tc('type.text'), tc('type.image'), etc. from common
// "月" date format → t('day.month', { month })
// "日 星期" → t('day.dateFormat', { day, weekday })
// "选择左侧日期查看内容" → t('day.welcomeTitle')
// "浏览和管理你的所有历史数据" → t('day.welcomeDesc')
// "总条目" → t('day.totalItems')
// "活跃天数" → t('day.activeDays')
// "条内容" → t('day.itemCount', { count })
// "导出此日" → t('day.exportDay')
// "这一天没有记录的内容" → t('day.empty')
```

- [ ] **Step 2: Migrate DateSidebar.tsx**

```tsx
// "条 · ...天" → t('sidebar.stats', { items, days })
// "暂无数据" → t('sidebar.noData')
// "日 周" → t('sidebar.dateFormat', { day, weekday })
// "导出设置" → t('sidebar.exportSettings')
// "打开文件夹" → t('sidebar.openFolder')
```

- [ ] **Step 3: Migrate ExportPanel.tsx**

```tsx
// "导出设置" → t('export.title')
// "导出目录" → t('export.directory')
// "未设置" → t('export.notSet')
// "导出全部" → t('export.exportAll')
// "导出中..." → t('export.exporting')
// "已导出...个文件" → t('export.done', { count })
// "在 Finder 中打开" → t('export.openFinder')
```

- [ ] **Step 4: Migrate ReportView.tsx**

```tsx
// "周报" → t('report.title')
// "历史" → t('report.history')
// "生成中" → t('report.generating')
// "生成" → t('report.generate')
// "加载周报失败，请稍后重试" → t('report.loadError')
// "生成周报失败，请确认已配置 AI 服务后重试" → t('report.generateError')
```

- [ ] **Step 5: Migrate ReportCard.tsx**

```tsx
// "查看原文" → t('report.viewSource')
```

- [ ] **Step 6: Migrate ContentPreviewPanel.tsx**

```tsx
// "相关内容" → t('report.relatedContent')
// "条" → t('report.relatedCount', { count })
```

- [ ] **Step 7: Migrate FeedbackButtons.tsx**

```tsx
// "感兴趣" → t('report.interested')
// "已了解" → t('report.understood')
// "已记录" → t('report.noted')
```

- [ ] **Step 8: Migrate TextContentList.tsx**

```tsx
// "本周没有文本内容" → t('report.noTextContent')
// "条文本" → t('report.textCount', { count })
// "收起" → tc('btn.collapse')
// "展开" → tc('btn.expand')
```

- [ ] **Step 9: Migrate CompactLinkList.tsx**

```tsx
// "本周没有链接内容" → t('report.noLinkContent')
// "个链接" → t('report.linkCount', { count })
// "未知链接" → t('report.unknownLink')
```

- [ ] **Step 10: Migrate ActivityStatsCard.tsx**

```tsx
// Filter labels: "全部", "文本", "链接", "图片" → use tc('type.all'), tc('type.text'), etc.
// "周" day format → t('report.weekday', { day })
```

- [ ] **Step 11: Migrate ImageFilmstrip.tsx**

```tsx
// "本周没有图片内容" → t('report.noImageContent')
// "张图片" → t('report.imageCount', { count })
```

- [ ] **Step 12: Verify build**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

- [ ] **Step 13: Commit**

```bash
git add src/features/data-hub/ src/features/weekly-report/
git commit -m "feat(i18n): migrate datahub + weekly report modules"
```

---

## Task 8: Final Verification and Polish

**Files:**
- Possibly adjust: any file with missing translations

- [ ] **Step 1: Search for remaining Chinese hardcoded text**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
grep -rn '[\x{4e00}-\x{9fff}]' src/ --include="*.tsx" --include="*.ts" | grep -v 'node_modules' | grep -v 'locales/' | grep -v '\.d\.ts'
```

If any Chinese text is found that should be translated, add appropriate keys to the JSON files and replace the hardcoded text.

Exceptions (OK to keep as Chinese):
- Comments in code
- Console.log/console.error messages (developer-facing)
- AI prompt strings (out of scope)

- [ ] **Step 2: Verify full build succeeds**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run build
```

Expected: Clean build, no errors, no warnings about missing translations.

- [ ] **Step 3: Manual smoke test**

```bash
cd "/Users/pipiwang/Documents/文稿 - Rich ray/xiaoyun"
npm run tauri dev
```

Test checklist:
1. App opens — UI shows Chinese (or English if OS is English)
2. Go to Settings > Appearance — language switcher is visible
3. Switch to English — all UI text updates immediately, no reload needed
4. Switch back to Chinese — text reverts
5. Close and reopen app — language preference persists
6. Navigate all tabs (Content, Wiki, Insights, Settings) — no raw translation keys visible
7. Check search placeholder text changes with language
8. Check bubble window (if visible) — text is translated

- [ ] **Step 4: Fix any issues found**

Address any missing translations, layout overflow with English text, or untranslated strings found during testing.

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "feat(i18n): final polish — fix remaining untranslated text and layout issues"
```

---

## Summary

| Task | Description | Files | Estimated Steps |
|------|-------------|-------|----------------|
| 1 | Foundation — deps, i18n.ts, 12 JSON files | 14 new, 2 modified | 18 |
| 2 | App.tsx — nav, search, brand | 1 modified | 3 |
| 3 | Settings — language switcher + all text | 2 modified | 4 |
| 4 | Content module — list, cards, bubble, spotlight | 6 modified | 8 |
| 5 | Wiki module — all 7 components | 7 modified | 9 |
| 6 | Digest module — radar, digest, insight | 4 modified | 6 |
| 7 | DataHub + Report — 11 components | 11 modified | 13 |
| 8 | Final verification | varies | 5 |
| **Total** | | **~33 files** | **66 steps** |
