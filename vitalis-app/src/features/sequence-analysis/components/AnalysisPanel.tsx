interface AnalysisPanelProps {
  sequenceId: string;
  onGetStatistics: () => void;
  loading?: boolean;
}

export const AnalysisPanel = ({ sequenceId, onGetStatistics, loading = false }: AnalysisPanelProps) => {
  return (
    <section className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h2 className="text-2xl font-semibold text-gray-900 mb-4">Analysis</h2>
      <div className="space-y-4">
        <p className="text-gray-700">
          <span className="font-medium text-gray-900">Sequence ID:</span>
          <span className="ml-2 font-mono text-sm bg-gray-100 px-2 py-1 rounded">{sequenceId}</span>
        </p>
        <button
          onClick={onGetStatistics}
          disabled={loading}
          className="px-6 py-3 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Calculating..." : "Get Statistics"}
        </button>
      </div>
    </section>
  );
};