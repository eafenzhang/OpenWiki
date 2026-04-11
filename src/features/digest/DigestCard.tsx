import { useTranslation } from "react-i18next";
import type { CapturedContent } from "../../types/content";

interface DigestCardProps {
  content: CapturedContent;
}

function useTimeAgo() {
  const { t } = useTranslation("digest");

  return (dateStr: string): string => {
    const now = new Date();
    const then = new Date(dateStr);
    const diffMs = now.getTime() - then.getTime();
    const days = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    if (days === 0) return t("timeAgo.today");
    if (days === 1) return t("timeAgo.yesterday");
    if (days < 30) return t("timeAgo.daysAgo", { count: days });
    const months = Math.floor(days / 30);
    if (months < 12) return t("timeAgo.monthsAgo", { count: months });
    return t("timeAgo.yearsAgo", { count: Math.floor(months / 12) });
  };
}

function typeIcon(type: string): string {
  switch (type) {
    case "image": return "📷";
    case "url": return "🔗";
    default: return "📝";
  }
}

export function DigestCard({ content }: DigestCardProps) {
  const { t } = useTranslation("digest");
  const timeAgo = useTimeAgo();

  const fullText = content.raw_text || content.source_url || (content.image_path ? t("card.imagePlaceholder") : t("card.noContentText"));

  return (
    <div className="glass rounded-xl overflow-hidden flex flex-col h-full">
      {/* Meta — fixed top */}
      <div className="flex items-center justify-between px-4 pt-3 pb-2 flex-shrink-0">
        <div className="flex items-center gap-2">
          <span className="text-xs">{typeIcon(content.content_type)}</span>
          <span className="text-[11px] text-gray-500 dark:text-slate-400 bg-gray-100/50 dark:bg-white/[0.06] px-2 py-0.5 rounded-full">
            {content.source_app}
          </span>
        </div>
        <span className="text-[11px] text-amber-500 dark:text-amber-400 italic">
          {timeAgo(content.captured_at)}{t("card.saved")}
        </span>
      </div>

      {/* Content — scrollable */}
      <div className="flex-1 overflow-y-auto px-4 pb-3">
        <p className="text-sm text-gray-700 dark:text-gray-200 leading-relaxed whitespace-pre-wrap break-words">
          {fullText}
        </p>
        {content.content_type === "image" && content.image_path && (
          <div className="mt-3 rounded-lg overflow-hidden">
            <img
              src={`asset://localhost/${content.image_path}`}
              alt=""
              className="w-full max-h-48 object-cover rounded-lg"
            />
          </div>
        )}
      </div>
    </div>
  );
}
