import { useState } from "react";
import { SequenceInfo } from "../../../types/sequence";

interface SequenceSelectorProps {
  sequences: SequenceInfo[];
  onSelect: (sequenceIndex: number) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const SequenceSelector = ({
  sequences,
  onSelect,
  onCancel,
  loading = false
}: SequenceSelectorProps) => {
  const [selectedIndex, setSelectedIndex] = useState<number>(0);

  const handleSelect = () => {
    onSelect(selectedIndex);
  };

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h3 className="text-xl font-semibold text-gray-900 mb-4">
        Multiple Sequences Found ({sequences.length})
      </h3>
      <p className="text-gray-600 mb-4">
        Your file contains {sequences.length} sequences. Please select one to analyze:
      </p>

      <div className="space-y-3 max-h-96 overflow-y-auto">
        {sequences.map((seq, index) => (
          <div
            key={index}
            className={`border rounded-lg p-4 cursor-pointer transition-colors ${
              selectedIndex === index
                ? 'border-blue-500 bg-blue-50'
                : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
            }`}
            onClick={() => setSelectedIndex(index)}
          >
            <div className="flex justify-between items-start mb-2">
              <h4 className="font-medium text-gray-900">
                {seq.id || `Sequence ${index + 1}`}
              </h4>
              <span className="text-sm text-gray-500">
                {seq.length.toLocaleString()} bp
              </span>
            </div>

            {seq.name && (
              <p className="text-sm text-gray-600 mb-2">{seq.name}</p>
            )}

            <div className="bg-gray-100 rounded p-2 font-mono text-xs">
              <div className="text-gray-700">
                {seq.preview}
                {seq.length > 50 && <span className="text-gray-400">...</span>}
              </div>
            </div>

            <div className="flex justify-between items-center mt-2 text-xs text-gray-500">
              <span>Preview: {Math.min(seq.preview.length, 50)} chars</span>
              <input
                type="radio"
                name="sequence-selection"
                checked={selectedIndex === index}
                onChange={() => setSelectedIndex(index)}
                className="text-blue-600 focus:ring-blue-500"
              />
            </div>
          </div>
        ))}
      </div>

      <div className="flex gap-3 mt-6">
        <button
          onClick={handleSelect}
          disabled={loading}
          className="flex-1 px-4 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Importing..." : "Import Selected Sequence"}
        </button>
        <button
          onClick={onCancel}
          disabled={loading}
          className="px-4 py-2 bg-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:bg-gray-200 disabled:cursor-not-allowed transition-colors"
        >
          Cancel
        </button>
      </div>
    </div>
  );
};