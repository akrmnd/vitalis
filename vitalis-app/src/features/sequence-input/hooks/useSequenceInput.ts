import { useState } from "react";
import { SequenceInputData } from "../../../types/sequence";

export const useSequenceInput = () => {
  const [sequence, setSequence] = useState("");

  const getInputData = (): SequenceInputData => ({
    content: sequence,
    format: "fasta" as const
  });

  const isValid = () => sequence.trim().length > 0;

  const clear = () => setSequence("");

  return {
    sequence,
    setSequence,
    getInputData,
    isValid,
    clear
  };
};