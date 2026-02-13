import { create } from "zustand";
import type { WeeklyReport, ReportSummary } from "../types/report";

interface ReportState {
  currentReport: WeeklyReport | null;
  reportList: ReportSummary[];
  isGenerating: boolean;
  error: string | null;
  setCurrentReport: (report: WeeklyReport | null) => void;
  setReportList: (reports: ReportSummary[]) => void;
  setIsGenerating: (generating: boolean) => void;
  setError: (error: string | null) => void;
}

export const useReportStore = create<ReportState>((set) => ({
  currentReport: null,
  reportList: [],
  isGenerating: false,
  error: null,
  setCurrentReport: (report) => set({ currentReport: report }),
  setReportList: (reports) => set({ reportList: reports }),
  setIsGenerating: (generating) => set({ isGenerating: generating }),
  setError: (error) => set({ error }),
}));
