import { useEffect, useState, useRef, useCallback } from "react";
import { createPortal } from "react-dom";
import { useTranslation } from "react-i18next";

interface ImagePreviewProps {
  src: string;
  onClose: () => void;
}

export function ImagePreview({ src, onClose }: ImagePreviewProps) {
  const { t } = useTranslation("content");
  const [scale, setScale] = useState(1);
  const [translate, setTranslate] = useState({ x: 0, y: 0 });
  const [dragging, setDragging] = useState(false);
  const dragStart = useRef({ x: 0, y: 0 });
  const translateStart = useRef({ x: 0, y: 0 });
  const containerRef = useRef<HTMLDivElement>(null);

  // Close on Escape, reset on R
  useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
      if (e.key === "r" || e.key === "R") {
        setScale(1);
        setTranslate({ x: 0, y: 0 });
      }
    };
    window.addEventListener("keydown", handleKey);
    document.body.style.overflow = "hidden";
    return () => {
      window.removeEventListener("keydown", handleKey);
      document.body.style.overflow = "";
    };
  }, [onClose]);

  // Scroll wheel zoom
  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -0.1 : 0.1;
    setScale((prev) => Math.max(0.1, Math.min(10, prev + delta * prev)));
  }, []);

  // Drag start
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    if (e.button !== 0) return;
    e.preventDefault();
    setDragging(true);
    dragStart.current = { x: e.clientX, y: e.clientY };
    translateStart.current = { ...translate };
  }, [translate]);

  // Drag move
  useEffect(() => {
    if (!dragging) return;
    const handleMove = (e: MouseEvent) => {
      setTranslate({
        x: translateStart.current.x + e.clientX - dragStart.current.x,
        y: translateStart.current.y + e.clientY - dragStart.current.y,
      });
    };
    const handleUp = () => setDragging(false);
    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);
    return () => {
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
  }, [dragging]);

  // Click backdrop to close (only if not dragged)
  const handleBackdropClick = useCallback((e: React.MouseEvent) => {
    if (e.target === containerRef.current) {
      onClose();
    }
  }, [onClose]);

  const zoomIn = () => setScale((s) => Math.min(10, s * 1.3));
  const zoomOut = () => setScale((s) => Math.max(0.1, s / 1.3));
  const resetView = () => { setScale(1); setTranslate({ x: 0, y: 0 }); };

  return createPortal(
    <div
      ref={containerRef}
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
      onClick={handleBackdropClick}
      onWheel={handleWheel}
      style={{ cursor: dragging ? "grabbing" : "grab" }}
    >
      {/* Top controls */}
      <div className="absolute top-4 right-4 flex items-center gap-2 z-10">
        <span className="text-white/50 text-xs mr-1">
          {Math.round(scale * 100)}%
        </span>
        <button
          onClick={zoomOut}
          className="text-white/80 hover:text-white text-lg w-8 h-8 flex items-center justify-center rounded-full bg-black/30 hover:bg-black/50 transition-colors"
        >
          −
        </button>
        <button
          onClick={resetView}
          className="text-white/80 hover:text-white text-xs w-8 h-8 flex items-center justify-center rounded-full bg-black/30 hover:bg-black/50 transition-colors"
        >
          1:1
        </button>
        <button
          onClick={zoomIn}
          className="text-white/80 hover:text-white text-lg w-8 h-8 flex items-center justify-center rounded-full bg-black/30 hover:bg-black/50 transition-colors"
        >
          +
        </button>
        <button
          onClick={onClose}
          className="text-white/80 hover:text-white text-xl w-8 h-8 flex items-center justify-center rounded-full bg-black/30 hover:bg-black/50 transition-colors ml-1"
        >
          ✕
        </button>
      </div>

      {/* Image */}
      <img
        src={src}
        alt="Preview"
        draggable={false}
        onMouseDown={handleMouseDown}
        className="select-none rounded-lg shadow-2xl"
        style={{
          maxWidth: scale <= 1 ? "90vw" : "none",
          maxHeight: scale <= 1 ? "90vh" : "none",
          transform: `translate(${translate.x}px, ${translate.y}px) scale(${scale})`,
          transformOrigin: "center center",
          transition: dragging ? "none" : "transform 0.15s ease-out",
        }}
      />

      {/* Bottom hint */}
      <div className="absolute bottom-4 left-1/2 -translate-x-1/2 text-white/30 text-xs">
        {t("imagePreview.hint")}
      </div>
    </div>,
    document.body
  );
}
