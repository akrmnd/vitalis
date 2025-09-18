import { useState } from "react";
import { SequenceInputData } from "../../../types/sequence";

export const useSequenceInput = () => {
  const [sequence, setSequence] = useState("");
  const [format, setFormat] = useState<"fasta" | "fastq" | "genbank">("fasta");

  const getInputData = (): SequenceInputData => ({
    content: sequence,
    format: format
  });

  const isValid = () => sequence.trim().length > 0;

  const clear = () => {
    setSequence("");
    setFormat("fasta");
  };

  const detectFormat = (content: string): "fasta" | "fastq" | "genbank" => {
    const trimmed = content.trim();
    if (trimmed.startsWith("LOCUS") || trimmed.includes("DEFINITION") || trimmed.includes("FEATURES")) {
      return "genbank";
    } else if (trimmed.startsWith("@")) {
      return "fastq";
    } else {
      return "fasta";
    }
  };

  const setSequenceWithFormatDetection = (content: string) => {
    setSequence(content);
    if (content.trim()) {
      setFormat(detectFormat(content));
    }
  };

  return {
    sequence,
    format,
    setSequence,
    setFormat,
    setSequenceWithFormatDetection,
    getInputData,
    isValid,
    clear
  };
};