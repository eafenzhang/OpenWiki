import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { open as openExternal } from "@tauri-apps/plugin-shell";
import { useTranslation } from "react-i18next";
import { Sparkles, X } from "lucide-react";
import {
  dismissUpdateVersion,
  type UpdateInfo,
} from "../../services/updateService";

/**
 * Top-of-main-window banner announcing a newer OpenWiki version.
 *
 * Listens for the `update-available` Tauri event which is emitted by the
 * background check in `src-tauri/src/update/mod.rs`. Also re-renders when
 * `window.dispatchEvent(new CustomEvent("update-available-manual", ...))` is
 * fired by the Settings "Check now" button.
 */
export function UpdateBanner() {
  const { t } = useTranslation("update");
  const [info, setInfo] = useState<UpdateInfo | null>(null);

  useEffect(() => {
    // Tauri backend event (fired by spawn_background_check on startup)
    const unlisten = listen<UpdateInfo>("update-available", (event) => {
      setInfo(event.payload);
    });

    // Manual trigger from the Settings "Check now" button — the Settings
    // view already has the payload, it just asks the banner to show.
    const manualHandler = (e: Event) => {
      const ce = e as CustomEvent<UpdateInfo>;
      if (ce.detail) setInfo(ce.detail);
    };
    window.addEventListener("update-available-manual", manualHandler);

    return () => {
      unlisten.then((fn) => fn());
      window.removeEventListener("update-available-manual", manualHandler);
    };
  }, []);

  if (!info) return null;

  const handleView = async () => {
    try {
      await openExternal(info.url);
    } catch (err) {
      console.error("[update] failed to open release page:", err);
    }
  };

  const handleLater = async () => {
    try {
      await dismissUpdateVersion(info.version);
    } catch (err) {
      console.error("[update] failed to dismiss version:", err);
    }
    setInfo(null);
  };

  return (
    <div
      role="status"
      aria-live="polite"
      className="sticky top-[40px] z-[9] border-b border-orange-200/60 dark:border-orange-500/20
                 bg-orange-50/95 dark:bg-orange-500/[0.08] backdrop-blur-xl
                 animate-in fade-in slide-in-from-top-2 duration-300"
    >
      <div className="flex items-center gap-3 px-4 py-2.5 max-w-full">
        <Sparkles className="w-4 h-4 text-orange-500 flex-shrink-0" />

        <div className="flex-1 min-w-0">
          <div className="text-[13px] font-medium text-gray-900 dark:text-orange-50 truncate">
            {t("banner.title", { version: info.version })}
          </div>
          <div className="text-[11px] text-gray-500 dark:text-orange-200/70 truncate">
            {t("banner.subtitle", { current: info.current_version })}
          </div>
        </div>

        <button
          onClick={handleView}
          className="flex-shrink-0 px-3 py-1 text-[12px] font-medium rounded-md
                     bg-orange-500 text-white hover:bg-orange-600
                     transition-colors shadow-sm"
        >
          {t("banner.view")}
        </button>

        <button
          onClick={handleLater}
          className="flex-shrink-0 px-3 py-1 text-[12px] font-medium rounded-md
                     text-orange-700 dark:text-orange-200 hover:bg-orange-100/60
                     dark:hover:bg-orange-500/[0.15] transition-colors"
        >
          {t("banner.later")}
        </button>

        <button
          onClick={handleLater}
          aria-label={t("banner.close")}
          className="flex-shrink-0 p-1 rounded text-gray-400 hover:text-gray-700
                     dark:text-orange-300/60 dark:hover:text-orange-200
                     hover:bg-orange-100/40 dark:hover:bg-orange-500/10 transition-colors"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
}
