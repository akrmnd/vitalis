import { useState, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useSequenceInput } from "../hooks/useSequenceInput";
import { SequenceInputData, ImportFromFileRequest } from "../../../types/sequence";
import { tauriApi } from "../../../lib/tauri-api";

interface SequenceInputFormProps {
  onSubmit: (data: SequenceInputData) => void;
  onFileImport?: (filePath: string, format: string) => void;
  loading?: boolean;
}

export const SequenceInputForm = ({ onSubmit, onFileImport, loading = false }: SequenceInputFormProps) => {
  const { sequence, setSequence, getInputData, isValid } = useSequenceInput();
  const [fileLoading, setFileLoading] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);

  const handleSubmit = () => {
    if (!isValid()) return;
    onSubmit(getInputData());
  };

  const handleFileSelect = async () => {
    try {
      setFileLoading(true);
      const file = await open({
        title: "Select sequence file",
        filters: [
          {
            name: "Sequence Files",
            extensions: ["fasta", "fa", "fas", "fastq", "fq"]
          },
          {
            name: "FASTA Files",
            extensions: ["fasta", "fa", "fas"]
          },
          {
            name: "FASTQ Files",
            extensions: ["fastq", "fq"]
          },
          {
            name: "All Files",
            extensions: ["*"]
          }
        ]
      });

      if (file) {
        // Auto-detect format from file extension
        const extension = file.split('.').pop()?.toLowerCase();
        let format = "fasta"; // default

        if (extension && ["fastq", "fq"].includes(extension)) {
          format = "fastq";
        }

        if (onFileImport) {
          onFileImport(file, format);
        }
      }
    } catch (error) {
      console.error("Error selecting file:", error);
    } finally {
      setFileLoading(false);
    }
  };

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  }, []);

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    if (loading || fileLoading) return;

    const files = Array.from(e.dataTransfer.files);
    if (files.length === 0) return;

    const file = files[0]; // Take the first file only

    // Check if it's a valid sequence file
    const extension = file.name.split('.').pop()?.toLowerCase();
    const validExtensions = ["fasta", "fa", "fas", "fastq", "fq"];

    if (!extension || !validExtensions.includes(extension)) {
      alert("Please drop a valid sequence file (.fasta, .fa, .fas, .fastq, .fq)");
      return;
    }

    try {
      setFileLoading(true);

      // Auto-detect format from file extension
      let format = "fasta"; // default
      if (extension && ["fastq", "fq"].includes(extension)) {
        format = "fastq";
      }

      if (onFileImport) {
        onFileImport(file.path, format);
      }
    } catch (error) {
      console.error("Error processing dropped file:", error);
    } finally {
      setFileLoading(false);
    }
  }, [loading, fileLoading, onFileImport]);

  return (
    <section className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h2 className="text-2xl font-semibold text-gray-900 mb-4">Sequence Input</h2>

      {/* File import section */}
      <div
        className={`mb-6 p-4 rounded-lg border-2 border-dashed transition-colors ${
          isDragOver
            ? "bg-green-50 border-green-400 border-solid"
            : "bg-gray-50 border-gray-300"
        }`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        <div className="text-center">
          <div className="mb-2">
            <span className="text-2xl">{isDragOver ? "⬇️" : "📁"}</span>
          </div>
          <p className="text-sm text-gray-600 mb-3">
            {isDragOver
              ? "Drop your sequence file here"
              : "Drag & drop or import sequence file (FASTA, FASTQ)"
            }
          </p>
          <button
            onClick={handleFileSelect}
            disabled={loading || fileLoading}
            className="px-4 py-2 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
          >
            {fileLoading ? "Loading..." : "Select File"}
          </button>
        </div>
      </div>

      {/* Text input section */}
      <div className="space-y-4">
        <div className="flex items-center space-x-2 text-sm text-gray-500">
          <hr className="flex-1" />
          <span>or paste sequence directly</span>
          <hr className="flex-1" />
        </div>

        <textarea
          value={sequence}
          onChange={(e) => setSequence(e.target.value)}
          placeholder="Paste your FASTA sequence here..."
          rows={10}
          className="w-full p-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 font-mono text-sm resize-none"
          disabled={loading || fileLoading}
        />
        <button
          onClick={handleSubmit}
          disabled={loading || fileLoading || !isValid()}
          className="px-6 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Parsing..." : "Parse Sequence"}
        </button>
      </div>
    </section>
  );
};