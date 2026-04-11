-- Add locale column to content and AI-generated tables
-- Default 'zh-CN' for all existing data
ALTER TABLE captured_content ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';
ALTER TABLE weekly_reports ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';
ALTER TABLE attention_insights ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';
ALTER TABLE wiki_pages ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';
