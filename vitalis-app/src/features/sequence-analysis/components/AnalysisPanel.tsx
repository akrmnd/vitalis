import { useState, useEffect } from "react";
import { tauriApi } from "../../../lib/tauri-api";
import { SequenceViewer } from "./SequenceViewer";

interface SequenceMetadata {
  id: string;
  name: string;
  length: number;
  file_path?: string;
}

interface AnalysisPanelProps {
  sequenceId: string;
  onGetStatistics: () => void;
  loading?: boolean;
}

export const AnalysisPanel = ({ sequenceId, onGetStatistics, loading = false }: AnalysisPanelProps) => {
  const [metadata, setMetadata] = useState<SequenceMetadata | null>(null);
  const [metaLoading, setMetaLoading] = useState(false);
  const [showViewer, setShowViewer] = useState(false);

  useEffect(() => {
    const loadMetadata = async () => {
      setMetaLoading(true);
      try {
        const meta = await tauriApi.getMeta(sequenceId);
        setMetadata(meta);
      } catch (error) {
        console.error("Error loading metadata:", error);
      } finally {
        setMetaLoading(false);
      }
    };

    loadMetadata();
  }, [sequenceId]);

  return (
    <div className="space-y-6">
      <section className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h2 className="text-2xl font-semibold text-gray-900 mb-4">Analysis</h2>
        <div className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <span className="font-medium text-gray-900">Sequence ID:</span>
              <span className="ml-2 font-mono text-sm bg-gray-100 px-2 py-1 rounded">{sequenceId}</span>
            </div>

            {metaLoading ? (
              <div className="text-gray-500">Loading metadata...</div>
            ) : metadata ? (
              <>
                <div>
                  <span className="font-medium text-gray-900">Name:</span>
                  <span className="ml-2 text-gray-700">{metadata.name}</span>
                </div>
                <div>
                  <span className="font-medium text-gray-900">Length:</span>
                  <span className="ml-2 text-gray-700">{metadata.length.toLocaleString()} bases</span>
                </div>
                {metadata.file_path && (
                  <div>
                    <span className="font-medium text-gray-900">Source:</span>
                    <span className="ml-2 text-gray-700 text-sm">{metadata.file_path}</span>
                  </div>
                )}
              </>
            ) : null}
          </div>

          <div className="flex gap-3">
            <button
              onClick={onGetStatistics}
              disabled={loading}
              className="px-6 py-3 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              {loading ? "Calculating..." : "Get Statistics"}
            </button>

            <button
              onClick={() => setShowViewer(!showViewer)}
              disabled={!metadata}
              className="px-6 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              {showViewer ? "Hide Sequence" : "View Sequence"}
            </button>
          </div>
        </div>
      </section>

      {showViewer && metadata && (
        <SequenceViewer
          sequenceId={sequenceId}
          sequenceLength={metadata.length}
        />
      )}
    </div>
  );
};