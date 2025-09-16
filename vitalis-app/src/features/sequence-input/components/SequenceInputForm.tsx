import { useSequenceInput } from "../hooks/useSequenceInput";
import { SequenceInputData } from "../../../types/sequence";

interface SequenceInputFormProps {
  onSubmit: (data: SequenceInputData) => void;
  loading?: boolean;
}

export const SequenceInputForm = ({ onSubmit, loading = false }: SequenceInputFormProps) => {
  const { sequence, setSequence, getInputData, isValid } = useSequenceInput();

  const handleSubmit = () => {
    if (!isValid()) return;
    onSubmit(getInputData());
  };

  return (
    <section className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h2 className="text-2xl font-semibold text-gray-900 mb-4">Sequence Input</h2>
      <div className="space-y-4">
        <textarea
          value={sequence}
          onChange={(e) => setSequence(e.target.value)}
          placeholder="Paste your FASTA sequence here..."
          rows={10}
          className="w-full p-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 font-mono text-sm resize-none"
          disabled={loading}
        />
        <button
          onClick={handleSubmit}
          disabled={loading || !isValid()}
          className="px-6 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Parsing..." : "Parse Sequence"}
        </button>
      </div>
    </section>
  );
};