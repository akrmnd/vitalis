import { useState, useEffect, useRef } from "react";
import { tauriApi } from "../../../lib/tauri-api";

interface SequenceViewerProps {
  sequenceId: string;
  sequenceLength: number;
}

const WINDOW_SIZE = 1000; // Characters to load at once
const VISIBLE_LINES = 20; // Lines visible in viewport
const CHARS_PER_LINE = 80; // Characters per line

export const SequenceViewer = ({ sequenceId, sequenceLength }: SequenceViewerProps) => {
  const [windowStart, setWindowStart] = useState(0);
  const [sequenceData, setSequenceData] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  const loadSequenceWindow = async (start: number) => {
    setLoading(true);
    setError(null);

    try {
      const end = Math.min(start + WINDOW_SIZE, sequenceLength);
      const result = await tauriApi.getWindow(sequenceId, start, end);
      setSequenceData(result.bases);
      setWindowStart(start);
    } catch (err) {
      console.error("Error loading sequence window:", err);
      setError(err instanceof Error ? err.message : "Failed to load sequence");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSequenceWindow(0);
  }, [sequenceId, sequenceLength]);

  const formatSequence = (sequence: string): string[] => {
    const lines: string[] = [];
    for (let i = 0; i < sequence.length; i += CHARS_PER_LINE) {
      const lineStart = windowStart + i;
      const lineData = sequence.slice(i, i + CHARS_PER_LINE);
      const lineNumber = Math.floor(lineStart / CHARS_PER_LINE) + 1;
      lines.push(`${lineNumber.toString().padStart(6, ' ')}: ${lineData}`);
    }
    return lines;
  };

  const handlePrevious = () => {
    const newStart = Math.max(0, windowStart - WINDOW_SIZE);
    loadSequenceWindow(newStart);
  };

  const handleNext = () => {
    const newStart = Math.min(windowStart + WINDOW_SIZE, sequenceLength - WINDOW_SIZE);
    loadSequenceWindow(newStart);
  };

  const handleJumpTo = (position: number) => {
    const alignedStart = Math.floor(position / WINDOW_SIZE) * WINDOW_SIZE;
    const clampedStart = Math.max(0, Math.min(alignedStart, sequenceLength - WINDOW_SIZE));
    loadSequenceWindow(clampedStart);
  };

  const currentProgress = ((windowStart + WINDOW_SIZE) / sequenceLength) * 100;
  const formattedLines = formatSequence(sequenceData);

  if (error) {
    return (
      <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
        <p className="text-red-800">Error loading sequence: {error}</p>
        <button
          onClick={() => loadSequenceWindow(windowStart)}
          className="mt-2 px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg border border-gray-200 p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-xl font-semibold text-gray-900">Sequence Viewer</h3>
        <div className="text-sm text-gray-600">
          Showing {windowStart + 1}-{Math.min(windowStart + WINDOW_SIZE, sequenceLength)} of {sequenceLength} bases
        </div>
      </div>

      {/* Progress bar */}
      <div className="mb-4">
        <div className="flex justify-between text-xs text-gray-500 mb-1">
          <span>Position: {windowStart + 1}</span>
          <span>{currentProgress.toFixed(1)}%</span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-2">
          <div
            className="bg-blue-600 h-2 rounded-full transition-all duration-300"
            style={{ width: `${currentProgress}%` }}
          />
        </div>
      </div>

      {/* Navigation controls */}
      <div className="flex gap-2 mb-4">
        <button
          onClick={handlePrevious}
          disabled={windowStart === 0 || loading}
          className="px-3 py-1 bg-gray-600 text-white rounded hover:bg-gray-700 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          ← Previous
        </button>
        <button
          onClick={handleNext}
          disabled={windowStart + WINDOW_SIZE >= sequenceLength || loading}
          className="px-3 py-1 bg-gray-600 text-white rounded hover:bg-gray-700 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          Next →
        </button>
        <div className="flex items-center gap-2 ml-4">
          <label className="text-sm font-medium text-gray-700">Jump to position:</label>
          <input
            type="number"
            min="1"
            max={sequenceLength}
            placeholder="Position"
            className="w-24 px-2 py-1 border border-gray-300 rounded text-sm"
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                const position = parseInt((e.target as HTMLInputElement).value) - 1;
                if (!isNaN(position) && position >= 0 && position < sequenceLength) {
                  handleJumpTo(position);
                }
              }
            }}
          />
        </div>
      </div>

      {/* Sequence display */}
      <div
        ref={containerRef}
        className="relative bg-gray-50 border border-gray-200 rounded p-4 font-mono text-sm overflow-auto"
        style={{ height: `${VISIBLE_LINES * 1.5}rem` }}
      >
        {loading && (
          <div className="absolute inset-0 bg-white bg-opacity-75 flex items-center justify-center">
            <div className="text-gray-600">Loading sequence data...</div>
          </div>
        )}

        <div className="whitespace-pre-wrap">
          {formattedLines.map((line, index) => (
            <div key={index} className="leading-6">
              <span className="text-gray-500 mr-2">{line.split(': ')[0]}:</span>
              <span className="text-gray-900">{line.split(': ')[1]}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Sequence info */}
      <div className="mt-4 text-xs text-gray-500 flex justify-between">
        <span>Window size: {WINDOW_SIZE} bases</span>
        <span>Total sequence length: {sequenceLength.toLocaleString()} bases</span>
      </div>
    </div>
  );
};