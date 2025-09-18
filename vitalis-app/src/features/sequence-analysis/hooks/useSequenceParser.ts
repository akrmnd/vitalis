import { useState } from "react";
import { tauriApi } from "../../../lib/tauri-api";
import { SequenceInputData, ParsePreviewResponse } from "../../../types/sequence";

export const useSequenceParser = () => {
  const [loading, setLoading] = useState(false);
  const [sequenceId, setSequenceId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [preview, setPreview] = useState<ParsePreviewResponse | null>(null);
  const [currentInputData, setCurrentInputData] = useState<SequenceInputData | null>(null);

  const parsePreview = async (data: SequenceInputData) => {
    setLoading(true);
    setError(null);
    setCurrentInputData(data);

    try {
      const result = await tauriApi.parsePreview(data);
      setPreview(result);

      // If only one sequence, import it automatically
      if (result.sequences.length === 1) {
        await importSequence(data, 0);
      }
    } catch (err) {
      console.error("Error parsing sequence:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const importSequence = async (data: SequenceInputData, sequenceIndex: number) => {
    setLoading(true);
    setError(null);

    try {
      const result = await tauriApi.importSequence(data, sequenceIndex);
      setSequenceId(result.seq_id);
      setPreview(null); // Clear preview after successful import
    } catch (err) {
      console.error("Error importing sequence:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const parseSequence = async (data: SequenceInputData) => {
    setLoading(true);
    setError(null);

    try {
      const result = await tauriApi.parseAndImport(data);
      setSequenceId(result.seq_id);
    } catch (err) {
      console.error("Error parsing sequence:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const importFromFile = async (filePath: string, format: string, onGenBankMetadata?: (metadata: any) => void) => {
    setLoading(true);
    setError(null);

    try {
      // First, read the file content
      const content = await tauriApi.readFile(filePath);

      // Create input data from file content
      const data: SequenceInputData = {
        content: content,
        format: format as 'fasta' | 'fastq' | 'genbank'
      };

      // If it's GenBank format, parse metadata
      if (format === "genbank" && onGenBankMetadata) {
        try {
          const metadata = await tauriApi.getGenBankMetadata(content);
          onGenBankMetadata(metadata);
        } catch (error) {
          console.error("Failed to parse GenBank metadata:", error);
        }
      }

      // Use the same preview mechanism as text input
      const result = await tauriApi.parsePreview(data);
      setPreview(result);
      setCurrentInputData(data);

      // If only one sequence, import it automatically
      if (result.sequences.length === 1) {
        await importSequence(data, 0);
      }
    } catch (err) {
      console.error("Error importing file:", err);
      setError(err instanceof Error ? err.message : "Unknown error occurred");
    } finally {
      setLoading(false);
    }
  };

  const cancelPreview = () => {
    setPreview(null);
    setError(null);
  };

  const reset = () => {
    setSequenceId(null);
    setPreview(null);
    setCurrentInputData(null);
    setError(null);
    setLoading(false);
  };

  return {
    loading,
    sequenceId,
    error,
    preview,
    currentInputData,
    parsePreview,
    importSequence,
    parseSequence,
    importFromFile,
    cancelPreview,
    reset
  };
};