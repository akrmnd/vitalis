import { useState } from "react";
import { tauriApi } from "../../../lib/tauri-api";
import { SequenceStats } from "../../../types/sequence";

export const useStatistics = () => {
  const [loading, setLoading] = useState(false);
  const [stats, setStats] = useState<SequenceStats | null>(null);
  const [error, setError] = useState<string | null>(null);

  const getStatistics = async (sequenceId: string) => {
    setLoading(true);
    setError(null);

    try {
      const result = await tauriApi.getStatistics(sequenceId);
      setStats(result);
    } catch (err) {
      console.error("Error getting statistics:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const reset = () => {
    setStats(null);
    setError(null);
    setLoading(false);
  };

  return {
    loading,
    stats,
    error,
    getStatistics,
    reset
  };
};