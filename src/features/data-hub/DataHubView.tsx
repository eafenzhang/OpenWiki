import { useEffect, useState } from "react";
import { useDataHubStore } from "../../stores/dataHubStore";
import { DateSidebar } from "./DateSidebar";
import { DayDetail } from "./DayDetail";
import { ExportPanel } from "./ExportPanel";

export function DataHubView() {
  const totalItems = useDataHubStore((s) => s.totalItems);
  const totalDates = useDataHubStore((s) => s.totalDates);
  const loadDateList = useDataHubStore((s) => s.loadDateList);
  const [showExportPanel, setShowExportPanel] = useState(false);

  useEffect(() => {
    loadDateList();
  }, [loadDateList]);

  return (
    <>
      <div className="flex" style={{ height: "calc(100vh - 44px)" }}>
        <DateSidebar
          totalItems={totalItems}
          totalDates={totalDates}
          onOpenExportPanel={() => setShowExportPanel(true)}
        />
        <div className="flex-1 min-w-0">
          <DayDetail />
        </div>
      </div>

      {/* Export panel modal */}
      {showExportPanel && (
        <ExportPanel onClose={() => setShowExportPanel(false)} />
      )}
    </>
  );
}
