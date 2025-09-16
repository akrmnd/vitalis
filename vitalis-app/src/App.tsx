import { Layout } from "./components/Layout";
import { SequenceInputForm } from "./features/sequence-input/components/SequenceInputForm";
import { AnalysisPanel } from "./features/sequence-analysis/components/AnalysisPanel";
import { StatisticsDisplay } from "./features/statistics/components/StatisticsDisplay";
import { useSequenceParser } from "./features/sequence-analysis/hooks/useSequenceParser";
import { useStatistics } from "./features/statistics/hooks/useStatistics";
import { SequenceInputData } from "./types/sequence";

function App() {
  const { loading: parseLoading, sequenceId, parseSequence } = useSequenceParser();
  const { loading: statsLoading, stats, getStatistics } = useStatistics();

  const handleSequenceSubmit = (data: SequenceInputData) => {
    parseSequence(data);
  };

  const handleGetStatistics = () => {
    if (sequenceId) {
      getStatistics(sequenceId);
    }
  };

  return (
    <Layout>
      <SequenceInputForm
        onSubmit={handleSequenceSubmit}
        loading={parseLoading}
      />

      {sequenceId && (
        <AnalysisPanel
          sequenceId={sequenceId}
          onGetStatistics={handleGetStatistics}
          loading={statsLoading}
        />
      )}

      {stats && <StatisticsDisplay stats={stats} />}
    </Layout>
  );
}

export default App;