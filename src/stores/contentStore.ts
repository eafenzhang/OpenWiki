import { create } from "zustand";
import type { CapturedContent } from "../types/content";

interface ContentState {
  contents: CapturedContent[];
  isLoading: boolean;
  highlightedIds: string[];
  scrollToId: string | null;
  setContents: (contents: CapturedContent[]) => void;
  setIsLoading: (loading: boolean) => void;
  addContent: (content: CapturedContent) => void;
  removeContent: (id: string) => void;
  updateContent: (updated: CapturedContent) => void;
  setHighlightedIds: (ids: string[]) => void;
  setScrollToId: (id: string | null) => void;
  clearHighlights: () => void;
}

export const useContentStore = create<ContentState>((set) => ({
  contents: [],
  isLoading: false,
  highlightedIds: [],
  scrollToId: null,
  setContents: (contents) => set({ contents }),
  setIsLoading: (loading) => set({ isLoading: loading }),
  addContent: (content) =>
    set((state) => ({ contents: [content, ...state.contents] })),
  removeContent: (id) =>
    set((state) => ({
      contents: state.contents.filter((c) => c.id !== id),
    })),
  updateContent: (updated) =>
    set((state) => ({
      contents: state.contents.map((c) => (c.id === updated.id ? updated : c)),
    })),
  setHighlightedIds: (ids) => set({ highlightedIds: ids, scrollToId: ids[0] ?? null }),
  setScrollToId: (id) => set({ scrollToId: id }),
  clearHighlights: () => set({ highlightedIds: [], scrollToId: null }),
}));
