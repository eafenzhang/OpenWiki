export type SectionType =
  | "key_insight"
  | "highlight"
  | "trend"
  | "routine"
  | "recommendation"
  | "topic"; // legacy fallback

export interface ReportStats {
  total_items: number;
  topics_count: number;
  top_sources: Array<{ app: string; count: number }>;
  daily_counts: number[]; // length 7, Mon-Sun
  type_counts?: {
    text: number;
    url: number;
    image: number;
  };
}

export interface WeeklyReport {
  id: string;
  week_start: string;
  week_end: string;
  summary_text: string;
  report_json: {
    stats?: ReportStats;
    raw_response?: string;
  };
  content_count: number;
  model_used: string;
  tokens_used?: number;
  generated_at: string;
  sections: ReportSection[];
}

export interface ReportSection {
  id: string;
  report_id: string;
  section_type: SectionType;
  title: string;
  body: string;
  relevance_score?: number;
  sort_order: number;
  content_ids: string[];
}

export interface ReportSummary {
  id: string;
  week_start: string;
  week_end: string;
  summary_text: string;
  content_count: number;
  generated_at: string;
}

export type FeedbackType = "interested" | "dismissed" | "bookmarked";

export interface UserFeedback {
  id: string;
  content_id?: string;
  section_id?: string;
  feedback_type: FeedbackType;
  created_at: string;
}
