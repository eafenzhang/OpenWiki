import { useState, forwardRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { CapturedContent } from "../../types/content";
import { deleteContent } from "../../services/storageService";
import { useContentStore } from "../../stores/contentStore";
import { ImagePreview } from "./ImagePreview";

interface ContentCardProps {
  content: CapturedContent;
  isHighlighted?: boolean;
}

function formatRelativeTime(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return "刚刚";
  if (diffMin < 60) return `${diffMin} 分钟前`;
  if (diffHour < 24) return `${diffHour} 小时前`;
  if (diffDay < 7) return `${diffDay} 天前`;
  return date.toLocaleDateString("zh-CN", { month: "short", day: "numeric" });
}

export const ContentCard = forwardRef<HTMLDivElement, ContentCardProps>(
  function ContentCard({ content, isHighlighted = false }, ref) {
  const removeContent = useContentStore((s) => s.removeContent);
  const [previewOpen, setPreviewOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  const [deleteState, setDeleteState] = useState<"idle" | "confirm" | "deleting">("idle");

  const handleDelete = async () => {
    if (deleteState === "idle") {
      setDeleteState("confirm");
      return;
    }
    if (deleteState === "confirm") {
      setDeleteState("deleting");
      try {
        await deleteContent(content.id);
        removeContent(content.id);
      } catch (e) {
        console.error("Failed to delete:", e);
        setDeleteState("idle");
      }
    }
  };

  const cancelDelete = () => {
    setDeleteState("idle");
  };

  const handleCopy = async () => {
    if (!content.raw_text) return;
    try {
      await navigator.clipboard.writeText(content.raw_text);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch (e) {
      console.error("Failed to copy:", e);
    }
  };

  const typeConfig = {
    image: { icon: "🖼️", label: "图片" },
    url: { icon: "🔗", label: "链接" },
    text: { icon: "📝", label: "文本" },
    mixed: { icon: "📎", label: "混合" },
  };

  const { icon: typeIcon, label: typeLabel } = typeConfig[content.content_type] || typeConfig.text;
  const timeStr = formatRelativeTime(content.captured_at);

  // URL content states
  const isUrlContent = content.content_type === "url";
  const hasSourceUrl = isUrlContent && !!content.source_url;
  const isFetchedUrl = hasSourceUrl && content.raw_text !== content.source_url;
  const isLoadingUrl = hasSourceUrl && !isFetchedUrl;

  const imageSrc =
    content.content_type === "image"
      ? content.thumbnail_path
        ? convertFileSrc(content.thumbnail_path)
        : content.image_path
          ? convertFileSrc(content.image_path)
          : null
      : null;

  const fullImageSrc =
    content.content_type === "image" && content.image_path
      ? convertFileSrc(content.image_path)
      : null;

  return (
    <>
      <div
        ref={ref}
        className={`
        group bg-white dark:bg-slate-800 rounded-xl border transition-all duration-500
        ${isHighlighted
          ? "border-blue-300 dark:border-blue-500/50 shadow-lg shadow-blue-100/50 dark:shadow-blue-500/10 ring-2 ring-blue-200/60 dark:ring-blue-500/30 animate-highlight-fade"
          : deleteState !== "idle"
            ? "border-red-200 dark:border-red-500/30 shadow-md"
            : "border-gray-100 dark:border-slate-700 hover:border-gray-200 dark:hover:border-slate-600 hover:shadow-md dark:hover:shadow-slate-800/50"
        }
      `}>
        {/* Main content area */}
        <div className="p-4">
          <div className="flex items-start gap-3">
            {/* Type icon */}
            <div className={`w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 transition-colors duration-500 ${
              isHighlighted ? "bg-blue-50 dark:bg-blue-500/20" : "bg-gray-50 dark:bg-slate-700"
            }`}>
              <span className="text-base">{typeIcon}</span>
            </div>

            {/* Content body */}
            <div className="min-w-0 flex-1">
              {/* Image thumbnail */}
              {imageSrc && (
                <div
                  className="mb-2.5 cursor-pointer group/img inline-block"
                  onClick={() => setPreviewOpen(true)}
                >
                  <img
                    src={imageSrc}
                    alt="Captured"
                    className="max-w-full max-h-44 rounded-lg border border-gray-200 dark:border-slate-600
                               group-hover/img:border-blue-300 dark:group-hover/img:border-blue-500
                               group-hover/img:shadow transition-all object-cover"
                    loading="lazy"
                  />
                  <span className="text-[11px] text-gray-400 dark:text-slate-500
                                   group-hover/img:text-blue-500 dark:group-hover/img:text-blue-400 transition-colors mt-1 block">
                    点击查看大图
                  </span>
                </div>
              )}

              {/* URL content: three states */}
              {isUrlContent && isFetchedUrl && (
                <div>
                  <p className="text-sm text-gray-700 dark:text-gray-200 leading-relaxed line-clamp-4">
                    {content.raw_text}
                  </p>
                  <a
                    href={content.source_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="inline-flex items-center gap-1 mt-1.5 text-xs text-blue-500 dark:text-blue-400 hover:underline"
                  >
                    <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
                    </svg>
                    打开原文
                  </a>
                </div>
              )}

              {isUrlContent && isLoadingUrl && (
                <div className="flex items-center gap-2">
                  <p className="text-sm text-blue-500 dark:text-blue-400 truncate flex-1">
                    {content.source_url}
                  </p>
                  <span className="flex items-center gap-1.5 text-xs text-gray-400 dark:text-slate-500 flex-shrink-0">
                    <svg className="w-3.5 h-3.5 animate-spin" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                    </svg>
                    读取中
                  </span>
                </div>
              )}

              {/* Non-URL text content */}
              {!isUrlContent && content.raw_text && (
                <p className="text-sm text-gray-700 dark:text-gray-200 leading-relaxed line-clamp-4">
                  {content.raw_text}
                </p>
              )}

              {/* No content fallback */}
              {!imageSrc && !content.raw_text && !isUrlContent && (
                <p className="text-sm text-gray-400 dark:text-slate-500 italic">无内容</p>
              )}

              {/* Footer: meta + actions */}
              <div className="flex items-center justify-between mt-2.5">
                <div className="flex items-center gap-2 text-[11px] text-gray-400 dark:text-slate-500">
                  <span>{timeStr}</span>
                  <span className="text-gray-300 dark:text-slate-600">·</span>
                  <span>{content.source_app}</span>
                  <span className="text-gray-300 dark:text-slate-600">·</span>
                  <span>{typeLabel}</span>
                </div>

                {/* Action buttons */}
                <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
                  {hasSourceUrl && (
                    <a
                      href={content.source_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium
                                 text-gray-500 dark:text-slate-400 hover:text-blue-600 dark:hover:text-blue-400
                                 hover:bg-blue-50 dark:hover:bg-blue-500/10 transition-all"
                    >
                      <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
                      </svg>
                      打开链接
                    </a>
                  )}
                  {content.raw_text && (
                    <button
                      onClick={handleCopy}
                      className={`
                        flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium transition-all
                        ${copied
                          ? "bg-green-50 dark:bg-green-500/10 text-green-600 dark:text-green-400"
                          : "text-gray-500 dark:text-slate-400 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-500/10"
                        }
                      `}
                    >
                      {copied ? (
                        <>
                          <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                            <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                          </svg>
                          已复制
                        </>
                      ) : (
                        <>
                          <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                            <path strokeLinecap="round" strokeLinejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9.75a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
                          </svg>
                          复制
                        </>
                      )}
                    </button>
                  )}
                  <button
                    onClick={handleDelete}
                    className="flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium
                               text-gray-500 dark:text-slate-400 hover:text-red-600 dark:hover:text-red-400
                               hover:bg-red-50 dark:hover:bg-red-500/10 transition-all"
                  >
                    <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                    </svg>
                    删除
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Delete confirmation bar */}
        {deleteState !== "idle" && (
          <div className="px-4 py-3 bg-red-50 dark:bg-red-500/5 border-t border-red-100 dark:border-red-500/20 rounded-b-xl
                          flex items-center justify-between">
            <span className="text-sm text-red-600 dark:text-red-400 font-medium">
              {deleteState === "deleting" ? "正在删除..." : "确定要删除这条内容吗？"}
            </span>
            <div className="flex items-center gap-2">
              <button
                onClick={cancelDelete}
                disabled={deleteState === "deleting"}
                className="px-3 py-1.5 text-xs font-medium rounded-lg
                           text-gray-600 dark:text-slate-300 bg-white dark:bg-slate-700
                           border border-gray-200 dark:border-slate-600
                           hover:bg-gray-50 dark:hover:bg-slate-600 transition-colors
                           disabled:opacity-50"
              >
                取消
              </button>
              <button
                onClick={handleDelete}
                disabled={deleteState === "deleting"}
                className="px-3 py-1.5 text-xs font-medium rounded-lg
                           text-white bg-red-500 hover:bg-red-600
                           disabled:opacity-50 transition-colors"
              >
                {deleteState === "deleting" ? "删除中..." : "确认删除"}
              </button>
            </div>
          </div>
        )}
      </div>

      {previewOpen && fullImageSrc && (
        <ImagePreview
          src={fullImageSrc}
          onClose={() => setPreviewOpen(false)}
        />
      )}
    </>
  );
});
