import { useState } from "react";
import { tauriApi } from "../../../lib/tauri-api";
import { SequenceInputData } from "../../../types/sequence";

export const useSequenceParser = () => {
  const [loading, setLoading] = useState(false);
  const [sequenceId, setSequenceId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const parseSequence = async (data: SequenceInputData) => {
    setLoading(true);
    setError(null);

    try {
      const result = await tauriApi.parseAndImport(data);
      setSequenceId(result.sequence_id);
    } catch (err) {
      console.error("Error parsing sequence:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const reset = () => {
    setSequenceId(null);
    setError(null);
    setLoading(false);
  };

  return {
    loading,
    sequenceId,
    error,
    parseSequence,
    reset
  };
};