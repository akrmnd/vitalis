import { useState } from "react";
import { Layout } from "./components/Layout";
import { SequenceInputForm } from "./features/sequence-input/components/SequenceInputForm";
import { SequenceSelector } from "./features/sequence-input/components/SequenceSelector";
import { AnalysisPanel } from "./features/sequence-analysis/components/AnalysisPanel";
import { EnhancedStatisticsDisplay } from "./features/statistics/components/EnhancedStatisticsDisplay";
import { useSequenceParser } from "./features/sequence-analysis/hooks/useSequenceParser";
import { useStatistics } from "./features/statistics/hooks/useStatistics";
import { SequenceInputData } from "./types/sequence";

// Tab Components
const AnalyzeTab = ({
  parseLoading,
  sequenceId,
  preview,
  currentInputData,
  statsLoading,
  stats,
  onSequenceSubmit,
  onSequenceSelect,
  onFileImport,
  onGetStatistics,
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

    {sequenceId && (
      <AnalysisPanel
        sequenceId={sequenceId}
        onGetStatistics={onGetStatistics}
        loading={statsLoading}
      />
    )}
  </div>
);

const ResultsTab = ({ stats }: any) => (
  <div className="space-y-6">
    {stats ? (
      <EnhancedStatisticsDisplay stats={stats} />
    ) : (
      <div className="text-center py-12">
        <div className="text-6xl mb-4">📊</div>
        <h3 className="text-xl font-semibold text-gray-900 mb-2">No Results Yet</h3>
        <p className="text-gray-600">
          Import and analyze a sequence to see detailed results here.
        </p>
      </div>
    )}
  </div>
);

const GenBankTab = () => (
  <div className="text-center py-12">
    <div className="text-6xl mb-4">📁</div>
    <h3 className="text-xl font-semibold text-gray-900 mb-2">GenBank Import</h3>
    <p className="text-gray-600 mb-4">
      GenBank format support is coming soon!
    </p>
    <div className="inline-block bg-yellow-100 text-yellow-800 px-4 py-2 rounded-lg">
      🚧 Under Development
    </div>
  </div>
);

const RestrictionTab = () => (
  <div className="text-center py-12">
    <div className="text-6xl mb-4">✂️</div>
    <h3 className="text-xl font-semibold text-gray-900 mb-2">Restriction Sites</h3>
    <p className="text-gray-600 mb-4">
      Restriction enzyme analysis will be available soon!
    </p>
    <div className="inline-block bg-yellow-100 text-yellow-800 px-4 py-2 rounded-lg">
      🚧 Under Development
    </div>
  </div>
);

const SettingsTab = () => (
  <div className="space-y-6">
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h3 className="text-lg font-semibold text-gray-900 mb-4">Application Settings</h3>
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Default File Format
          </label>
          <select className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
            <option value="fasta">FASTA</option>
            <option value="fastq">FASTQ</option>
            <option value="genbank">GenBank (Coming Soon)</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Analysis Window Size
          </label>
          <input
            type="number"
            defaultValue={1000}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>
    </div>
  </div>
);

function App() {
  const [activeTab, setActiveTab] = useState("analyze");

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

  const handleSequenceSubmit = (data: SequenceInputData) => {
    parsePreview(data);
  };

  const handleSequenceSelect = (data: SequenceInputData, sequenceIndex: number) => {
    importSequence(data, sequenceIndex);
  };

  const handleFileImport = (filePath: string, format: string) => {
    importFromFile(filePath, format);
  };

  const handleGetStatistics = () => {
    if (sequenceId) {
      getStatistics(sequenceId);
      setActiveTab("results"); // Auto-switch to results tab
    }
  };

  const renderTabContent = () => {
    switch (activeTab) {
      case "analyze":
        return (
          <AnalyzeTab
            parseLoading={parseLoading}
            sequenceId={sequenceId}
            preview={preview}
            currentInputData={currentInputData}
            statsLoading={statsLoading}
            stats={stats}
            onSequenceSubmit={handleSequenceSubmit}
            onSequenceSelect={handleSequenceSelect}
            onFileImport={handleFileImport}
            onGetStatistics={handleGetStatistics}
            onCancelPreview={cancelPreview}
          />
        );
      case "results":
        return <ResultsTab stats={stats} />;
      case "genbank":
        return <GenBankTab />;
      case "restriction":
        return <RestrictionTab />;
      case "settings":
        return <SettingsTab />;
      default:
        return <AnalyzeTab />;
    }
  };

  return (
    <Layout
      activeTab={activeTab}
      onTabChange={setActiveTab}
      hasResults={!!stats}
      sequenceId={sequenceId}
    >
      {renderTabContent()}
    </Layout>
  );
}

export default App;