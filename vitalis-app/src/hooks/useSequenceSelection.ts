import { useState, useRef, useCallback, useEffect } from 'react';

interface SelectionInfo {
  start: number;
  end: number;
  length: number;
}

interface TooltipPosition {
  x: number;
  y: number;
}

export const useSequenceSelection = () => {
  const [selectionStart, setSelectionStart] = useState<number | null>(null);
  const [selectionEnd, setSelectionEnd] = useState<number | null>(null);
  const [isSelecting, setIsSelecting] = useState(false);
  const [tooltipPosition, setTooltipPosition] = useState<TooltipPosition | null>(null);
  const [cursorStyle, setCursorStyle] = useState<'default' | 'pointer' | 'text'>('default');

  // Refs for stable event handlers
  const isSelectingRef = useRef(false);
  const selectionStartRef = useRef<number | null>(null);

  const hasSelection = selectionStart !== null && selectionEnd !== null;

  const getSelectionInfo = useCallback((): SelectionInfo | null => {
    if (selectionStart === null || selectionEnd === null) return null;

    const start = Math.min(selectionStart, selectionEnd);
    const end = Math.max(selectionStart, selectionEnd);
    return {
      start,
      end,
      length: end - start + 1
    };
  }, [selectionStart, selectionEnd]);

  const clearSelection = useCallback(() => {
    setSelectionStart(null);
    setSelectionEnd(null);
    setIsSelecting(false);
    setTooltipPosition(null);
    setCursorStyle('default');
    isSelectingRef.current = false;
    selectionStartRef.current = null;
  }, []);

  const startSelection = useCallback((position: number, mousePosition: TooltipPosition) => {
    setSelectionStart(position);
    setSelectionEnd(position);
    setIsSelecting(true);
    setTooltipPosition(mousePosition);
    setCursorStyle('text');
    isSelectingRef.current = true;
    selectionStartRef.current = position;
  }, []);

  const updateSelection = useCallback((position: number, mousePosition: TooltipPosition) => {
    if (isSelectingRef.current && selectionStartRef.current !== null) {
      setSelectionEnd(position);
      setTooltipPosition(mousePosition);
    }
  }, []);

  const finishSelection = useCallback(() => {
    setIsSelecting(false);
    isSelectingRef.current = false;
    // Keep tooltip visible if we have a selection
    if (selectionStart !== null && selectionEnd !== null) {
      setCursorStyle('default');
    } else {
      clearSelection();
    }
  }, [selectionStart, selectionEnd, clearSelection]);

  const updateCursor = useCallback((isInSelectableArea: boolean) => {
    if (!isSelectingRef.current) {
      setCursorStyle(isInSelectableArea ? 'text' : 'default');
    }
  }, []);

  // Global mouse events for drag outside SVG
  useEffect(() => {
    const handleGlobalMouseMove = (e: MouseEvent) => {
      if (isSelectingRef.current) {
        const mousePosition = { x: e.clientX, y: e.clientY };
        setTooltipPosition(mousePosition);
      }
    };

    const handleGlobalMouseUp = () => {
      if (isSelectingRef.current) {
        finishSelection();
      }
    };

    if (isSelecting) {
      document.addEventListener('mousemove', handleGlobalMouseMove);
      document.addEventListener('mouseup', handleGlobalMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleGlobalMouseMove);
      document.removeEventListener('mouseup', handleGlobalMouseUp);
    };
  }, [isSelecting, finishSelection]);

  return {
    // State
    selectionStart,
    selectionEnd,
    isSelecting,
    hasSelection,
    tooltipPosition,
    cursorStyle,

    // Computed values
    getSelectionInfo,

    // Actions
    clearSelection,
    startSelection,
    updateSelection,
    finishSelection,
    updateCursor
  };
};