import { invoke } from "@tauri-apps/api/core";
import type { WeeklyReport, ReportSummary, FeedbackType } from "../types/report";

export async function generateReport(): Promise<WeeklyReport> {
  return invoke("generate_report");
}

export async function getReport(weekStart: string): Promise<WeeklyReport> {
  return invoke("get_report", { weekStart });
}

export async function getAllReports(): Promise<ReportSummary[]> {
  return invoke("get_all_reports");
}

export async function submitFeedback(
  contentId: string | null,
  sectionId: string | null,
  feedbackType: FeedbackType
): Promise<void> {
  return invoke("submit_feedback", { contentId, sectionId, feedbackType });
}
