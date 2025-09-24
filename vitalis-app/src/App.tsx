import { useState, useEffect } from "react";
import { Layout } from "./components/Layout";
import { SequenceInputForm } from "./features/sequence-input/components/SequenceInputForm";
import { SequenceSelector } from "./features/sequence-input/components/SequenceSelector";
import { AnalysisPanel } from "./features/sequence-analysis/components/AnalysisPanel";
import { EnhancedStatisticsDisplay } from "./features/statistics/components/EnhancedStatisticsDisplay";
import { GenBankMetadataViewer } from "./components/GenBankMetadataViewer";
import { CircularGenomeVisualization } from "./components/CircularGenomeVisualization";
import { SnapGeneStyleVisualization } from "./components/SnapGeneStyleVisualization";
import { useSequenceParser } from "./features/sequence-analysis/hooks/useSequenceParser";
import { useStatistics } from "./features/statistics/hooks/useStatistics";
import { SequenceInputData } from "./types/sequence";
import { GenBankMetadata } from "./types/genbank";
import { tauriApi } from "./lib/tauri-api";

// Tab Components
const ImportTab = ({
  parseLoading,
  preview,
  currentInputData,
  onSequenceSubmit,
  onSequenceSelect,
  onFileImport,
  onCancelPreview
}: any) => (
  <div className="space-y-6">
    <SequenceInputForm
      onSubmit={onSequenceSubmit}
      onFileImport={onFileImport}
      loading={parseLoading}
    />

    {preview && preview.sequences.length > 1 && currentInputData && (
      <SequenceSelector
        sequences={preview.sequences}
        onSelect={(sequenceIndex) => {
          onSequenceSelect(currentInputData, sequenceIndex);
        }}
        onCancel={onCancelPreview}
        loading={parseLoading}
      />
    )}
  </div>
);

const AnalysisTab = ({ sequenceId, stats, genbankMetadata, onGetStatistics, statsLoading }: any) => (
  <div className="space-y-6">
    {sequenceId && (
      <AnalysisPanel
        sequenceId={sequenceId}
        onGetStatistics={onGetStatistics}
        loading={statsLoading}
      />
    )}

    {stats && (
      <EnhancedStatisticsDisplay stats={stats} />
    )}

    {genbankMetadata && (
      <GenBankMetadataViewer metadata={genbankMetadata} />
    )}

    {!sequenceId && (
      <div className="text-center py-12">
        <div className="text-6xl mb-4">📊</div>
        <h3 className="text-xl font-semibold text-gray-900 mb-2">No Sequence Loaded</h3>
        <p className="text-gray-600">
          Import a sequence to see analysis results here.
        </p>
      </div>
    )}
  </div>
);


const VisualizationTab = ({ sequenceId, genbankMetadata }: any) => {
  const [sequenceData, setSequenceData] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [viewMode, setViewMode] = useState<'linear' | 'circular'>('linear');

  const loadFullSequence = async () => {
    if (!sequenceId) return;

    setLoading(true);
    try {
      // Get sequence metadata first to know the length
      const meta = await tauriApi.getMeta(sequenceId);
      // For visualization, we'll load a reasonable chunk (up to 10kb for now)
      const end = Math.min(meta.length, 10000);
      const result = await tauriApi.getWindow(sequenceId, 0, end);
      setSequenceData(result.bases);
    } catch (error) {
      console.error("Error loading sequence for visualization:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (sequenceId) {
      loadFullSequence();
    }
  }, [sequenceId]);

  if (!sequenceId) {
    return (
      <div className="text-center py-12">
        <div className="text-6xl mb-4">🧬</div>
        <h3 className="text-xl font-semibold text-gray-900 mb-2">Sequence Visualization</h3>
        <p className="text-gray-600">
          Import a sequence to see its visual representation with annotations.
        </p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="text-center py-12">
        <div className="text-4xl mb-4">⏳</div>
        <h3 className="text-xl font-semibold text-gray-900 mb-2">Loading Sequence...</h3>
        <p className="text-gray-600">
          Preparing visualization data...
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* View Mode Selector */}
      <div className="flex items-center justify-center space-x-4">
        <div className="bg-gray-100 rounded-lg p-1 flex">
          <button
            onClick={() => setViewMode('linear')}
            className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
              viewMode === 'linear'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            🧬 Linear View
          </button>
          <button
            onClick={() => setViewMode('circular')}
            className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
              viewMode === 'circular'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            🔄 Circular View
          </button>
        </div>
      </div>

      {/* Visualization Component */}
      {viewMode === 'linear' ? (
        <SnapGeneStyleVisualization
          sequence={sequenceData}
          metadata={genbankMetadata}
          width={1000}
        />
      ) : (
        <CircularGenomeVisualization
          sequence={sequenceData}
          metadata={genbankMetadata}
          diameter={500}
        />
      )}
    </div>
  );
};


function App() {
  const [activeTab, setActiveTab] = useState("import");
  const [genbankMetadata, setGenbankMetadata] = useState<GenBankMetadata | null>(null);

  const {
    loading: parseLoading,
    sequenceId,
    preview,
    currentInputData,
    parsePreview,
    importSequence,
    importFromFile,
    cancelPreview
  } = useSequenceParser();
  const { loading: statsLoading, stats, getStatistics } = useStatistics();

  const handleSequenceSubmit = async (data: SequenceInputData) => {
    // Clear previous GenBank metadata
    setGenbankMetadata(null);

    // Parse preview for sequence selection
    await parsePreview(data);

    // If it's GenBank format, parse metadata
    if (data.format === "genbank") {
      try {
        const metadata = await tauriApi.getGenBankMetadata(data.content);
        setGenbankMetadata(metadata);
      } catch (error) {
        console.error("Failed to parse GenBank metadata:", error);
      }
    }
  };

  const handleSequenceSelect = (data: SequenceInputData, sequenceIndex: number) => {
    importSequence(data, sequenceIndex);
    // 配列選択後、自動的に可視化タブへ遷移
    setActiveTab("visualization");
  };

  const handleFileImport = (filePath: string, format: string) => {
    importFromFile(filePath, format, (metadata) => {
      if (format === "genbank") {
        setGenbankMetadata(metadata);
      }
      // ファイル読み込み成功後、自動的に可視化タブへ遷移
      setActiveTab("visualization");
    });
  };

  const handleGetStatistics = () => {
    if (sequenceId) {
      getStatistics(sequenceId);
      setActiveTab("analysis"); // Auto-switch to analysis tab
    }
  };

  const renderTabContent = () => {
    switch (activeTab) {
      case "import":
        return (
          <ImportTab
            parseLoading={parseLoading}
            preview={preview}
            currentInputData={currentInputData}
            onSequenceSubmit={handleSequenceSubmit}
            onSequenceSelect={handleSequenceSelect}
            onFileImport={handleFileImport}
            onCancelPreview={cancelPreview}
          />
        );
      case "visualization":
        return <VisualizationTab sequenceId={sequenceId} genbankMetadata={genbankMetadata} />;
      case "analysis":
        return (
          <AnalysisTab
            sequenceId={sequenceId}
            stats={stats}
            genbankMetadata={genbankMetadata}
            onGetStatistics={handleGetStatistics}
            statsLoading={statsLoading}
          />
        );
      default:
        return <ImportTab />;
    }
  };

  return (
    <Layout
      activeTab={activeTab}
      onTabChange={setActiveTab}
      hasResults={!!stats}
      sequenceId={sequenceId || undefined}
    >
      {renderTabContent()}
    </Layout>
  );
}

export default App;