import type { WeeklyReport } from "../../types/report";
import { ActivityStatsCard } from "./ActivityStatsCard";
import type { FilterMode } from "./ActivityStatsCard";

interface ReportSummaryProps {
  report: WeeklyReport;
  activeFilter: FilterMode;
  onFilterChange: (filter: FilterMode) => void;
}

/**
 * No container, no card wrapper — just transparent filter tabs
 * floating directly on the page background.
 */
export function ReportSummaryCard({ report, activeFilter, onFilterChange }: ReportSummaryProps) {
  const stats = report.report_json?.stats;
  if (!stats) return null;

  return (
    <ActivityStatsCard
      stats={stats}
      activeFilter={activeFilter}
      onFilterChange={onFilterChange}
    />
  );
}
