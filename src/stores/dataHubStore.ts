import { create } from "zustand";
import {
  getDatesWithContent,
  getContentForDate,
  getExportDir,
} from "../services/dataHubService";
import type { CapturedContent } from "../types/content";

interface DateEntry {
  date: string;
  count: number;
}

interface MonthGroup {
  month: string; // "2026-03"
  label: string; // "2026年3月"
  dates: DateEntry[];
  expanded: boolean;
  totalCount: number;
}

interface DataHubState {
  selectedDate: string | null;
  monthGroups: MonthGroup[];
  dayContents: CapturedContent[];
  isLoading: boolean;
  exportDir: string;
  totalDates: number;
  totalItems: number;

  selectDate: (date: string) => Promise<void>;
  toggleMonth: (month: string) => void;
  loadDateList: () => Promise<void>;
  loadExportDir: () => Promise<void>;
  removeContent: (id: string) => void;
}

function groupByMonth(dates: DateEntry[]): MonthGroup[] {
  const map = new Map<string, DateEntry[]>();

  for (const entry of dates) {
    const month = entry.date.slice(0, 7); // "YYYY-MM"
    if (!map.has(month)) {
      map.set(month, []);
    }
    map.get(month)!.push(entry);
  }

  const groups: MonthGroup[] = [];
  for (const [month, entries] of map) {
    const [year, m] = month.split("-");
    const monthNum = parseInt(m, 10);
    groups.push({
      month,
      label: `${year}年${monthNum}月`,
      dates: entries.sort((a, b) => b.date.localeCompare(a.date)),
      expanded: false,
      totalCount: entries.reduce((sum, e) => sum + e.count, 0),
    });
  }

  // Sort descending by month
  groups.sort((a, b) => b.month.localeCompare(a.month));

  // Auto-expand the most recent month
  if (groups.length > 0) {
    groups[0].expanded = true;
  }

  return groups;
}

export const useDataHubStore = create<DataHubState>((set) => ({
  selectedDate: null,
  monthGroups: [],
  dayContents: [],
  isLoading: false,
  exportDir: "",
  totalDates: 0,
  totalItems: 0,

  selectDate: async (date: string) => {
    set({ selectedDate: date, isLoading: true });
    try {
      const contents = await getContentForDate(date);
      set({ dayContents: contents, isLoading: false });
    } catch (e) {
      console.error("Failed to load content for date:", e);
      set({ dayContents: [], isLoading: false });
    }
  },

  toggleMonth: (month: string) => {
    set((state) => ({
      monthGroups: state.monthGroups.map((g) =>
        g.month === month ? { ...g, expanded: !g.expanded } : g
      ),
    }));
  },

  loadDateList: async () => {
    try {
      const dates = await getDatesWithContent();
      const groups = groupByMonth(dates);
      const totalDates = dates.length;
      const totalItems = dates.reduce((sum, d) => sum + d.count, 0);
      set({ monthGroups: groups, totalDates, totalItems });
    } catch (e) {
      console.error("Failed to load date list:", e);
    }
  },

  loadExportDir: async () => {
    try {
      const dir = await getExportDir();
      set({ exportDir: dir });
    } catch (e) {
      console.error("Failed to load export dir:", e);
    }
  },

  removeContent: (id: string) => {
    set((state) => ({
      dayContents: state.dayContents.filter((c) => c.id !== id),
      totalItems: Math.max(0, state.totalItems - 1),
    }));
  },
}));
