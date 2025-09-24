import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface PrimerDesignParams {
  length_min: number;
  length_max: number;
  tm_min: number;
  tm_max: number;
  tm_optimal: number;
  gc_min: number;
  gc_max: number;
  max_self_dimer: number;
  max_hairpin: number;
  max_hetero_dimer: number;
}

interface Primer {
  sequence: string;
  position: number;
  length: number;
  tm: number;
  gc_content: number;
  self_dimer_score: number;
  hairpin_score: number;
  three_prime_stability: number;
  direction: 'Forward' | 'Reverse';
}

interface ValidationResults {
  self_dimer_check: boolean;
  hairpin_check: boolean;
  hetero_dimer_check?: boolean;
  specificity?: number;
  warnings: string[];
}

interface PrimerPair {
  id: string;
  forward: Primer;
  reverse: Primer;
  amplicon_length: number;
  amplicon_sequence: string;
  target_gene?: string;
  target_transcript?: string;
  compatibility_score: number;
  created_by: string;
  created_at: string;
  tags: string[];
  validation_results: ValidationResults;
}

interface PrimerDesignResult {
  pairs: PrimerPair[];
  design_params: PrimerDesignParams;
  target_sequence: string;
  target_start: number;
  target_end: number;
  multiplex_compatibility?: any;
}

interface PrimerDesignProps {
  sequenceId?: string;
}

const defaultParams: PrimerDesignParams = {
  length_min: 18,
  length_max: 25,
  tm_min: 55.0,
  tm_max: 65.0,
  tm_optimal: 60.0,
  gc_min: 40.0,
  gc_max: 60.0,
  max_self_dimer: -5.0,
  max_hairpin: -3.0,
  max_hetero_dimer: -5.0,
};

export const PrimerDesign: React.FC<PrimerDesignProps> = ({ sequenceId }) => {
  const [params, setParams] = useState<PrimerDesignParams>(defaultParams);
  const [targetStart, setTargetStart] = useState<number>(1);
  const [targetEnd, setTargetEnd] = useState<number>(1000);
  const [designResult, setDesignResult] = useState<PrimerDesignResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDesignPrimers = async () => {
    if (!sequenceId) {
      setError('No sequence loaded. Please import a sequence first.');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const result = await invoke<PrimerDesignResult>('tauri_design_primers', {
        seqId: sequenceId,
        start: targetStart - 1, // Convert to 0-based indexing
        end: targetEnd - 1,
        params: params,
      });

      setDesignResult(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleParamChange = (key: keyof PrimerDesignParams, value: number) => {
    setParams(prev => ({ ...prev, [key]: value }));
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h2 className="text-2xl font-bold text-gray-900 mb-2">PCR Primer Design</h2>
        <p className="text-gray-600">
          Design optimal PCR primer pairs for your target sequence region
        </p>
      </div>

      {/* Target Region Settings */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Target Region</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Start Position
            </label>
            <input
              type="number"
              value={targetStart}
              onChange={(e) => setTargetStart(parseInt(e.target.value) || 1)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              min="1"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              End Position
            </label>
            <input
              type="number"
              value={targetEnd}
              onChange={(e) => setTargetEnd(parseInt(e.target.value) || 1000)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              min="1"
            />
          </div>
        </div>
        <p className="text-sm text-gray-500 mt-2">
          Target region length: {Math.max(0, targetEnd - targetStart + 1)} bp
        </p>
      </div>

      {/* Design Parameters */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Design Parameters</h3>

        <div className="grid grid-cols-2 gap-6">
          {/* Primer Length */}
          <div>
            <h4 className="font-medium text-gray-700 mb-3">Primer Length</h4>
            <div className="space-y-3">
              <div>
                <label className="block text-sm text-gray-600 mb-1">Minimum</label>
                <input
                  type="number"
                  value={params.length_min}
                  onChange={(e) => handleParamChange('length_min', parseInt(e.target.value) || 18)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  min="10"
                  max="50"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">Maximum</label>
                <input
                  type="number"
                  value={params.length_max}
                  onChange={(e) => handleParamChange('length_max', parseInt(e.target.value) || 25)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  min="10"
                  max="50"
                />
              </div>
            </div>
          </div>

          {/* Melting Temperature */}
          <div>
            <h4 className="font-medium text-gray-700 mb-3">Melting Temperature (°C)</h4>
            <div className="space-y-3">
              <div>
                <label className="block text-sm text-gray-600 mb-1">Minimum</label>
                <input
                  type="number"
                  value={params.tm_min}
                  onChange={(e) => handleParamChange('tm_min', parseFloat(e.target.value) || 55)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="0.1"
                  min="40"
                  max="80"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">Maximum</label>
                <input
                  type="number"
                  value={params.tm_max}
                  onChange={(e) => handleParamChange('tm_max', parseFloat(e.target.value) || 65)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="0.1"
                  min="40"
                  max="80"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">Optimal</label>
                <input
                  type="number"
                  value={params.tm_optimal}
                  onChange={(e) => handleParamChange('tm_optimal', parseFloat(e.target.value) || 60)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="0.1"
                  min="40"
                  max="80"
                />
              </div>
            </div>
          </div>

          {/* GC Content */}
          <div>
            <h4 className="font-medium text-gray-700 mb-3">GC Content (%)</h4>
            <div className="space-y-3">
              <div>
                <label className="block text-sm text-gray-600 mb-1">Minimum</label>
                <input
                  type="number"
                  value={params.gc_min}
                  onChange={(e) => handleParamChange('gc_min', parseFloat(e.target.value) || 40)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="1"
                  min="0"
                  max="100"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">Maximum</label>
                <input
                  type="number"
                  value={params.gc_max}
                  onChange={(e) => handleParamChange('gc_max', parseFloat(e.target.value) || 60)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="1"
                  min="0"
                  max="100"
                />
              </div>
            </div>
          </div>

          {/* Secondary Structure */}
          <div>
            <h4 className="font-medium text-gray-700 mb-3">Secondary Structure (ΔG)</h4>
            <div className="space-y-3">
              <div>
                <label className="block text-sm text-gray-600 mb-1">Max Self-Dimer</label>
                <input
                  type="number"
                  value={params.max_self_dimer}
                  onChange={(e) => handleParamChange('max_self_dimer', parseFloat(e.target.value) || -5)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="0.1"
                  max="0"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">Max Hairpin</label>
                <input
                  type="number"
                  value={params.max_hairpin}
                  onChange={(e) => handleParamChange('max_hairpin', parseFloat(e.target.value) || -3)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                  step="0.1"
                  max="0"
                />
              </div>
            </div>
          </div>
        </div>

        <div className="mt-6">
          <button
            onClick={handleDesignPrimers}
            disabled={loading || !sequenceId}
            className="w-full bg-blue-600 text-white py-3 px-4 rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed font-medium"
          >
            {loading ? 'Designing Primers...' : 'Design Primers'}
          </button>
        </div>
      </div>

      {/* Error Display */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <div className="flex">
            <div className="text-red-400">⚠️</div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-red-800">Error</h3>
              <p className="text-sm text-red-700 mt-1">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* Results Display */}
      {designResult && (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            Design Results ({designResult.pairs.length} primer pairs found)
          </h3>

          {designResult.pairs.length === 0 ? (
            <div className="text-center py-8">
              <div className="text-4xl mb-2">🔍</div>
              <h4 className="text-lg font-medium text-gray-900 mb-2">No Primer Pairs Found</h4>
              <p className="text-gray-600">
                Try adjusting the design parameters or target region to find suitable primers.
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              {designResult.pairs.slice(0, 5).map((pair, index) => (
                <div key={pair.id} className="border border-gray-200 rounded-lg p-4">
                  <div className="flex justify-between items-start mb-3">
                    <h4 className="font-medium text-gray-900">Primer Pair {index + 1}</h4>
                    <div className="text-sm text-gray-500">
                      Amplicon: {pair.amplicon_length} bp
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    {/* Forward Primer */}
                    <div className="bg-green-50 rounded-lg p-3">
                      <h5 className="font-medium text-green-800 mb-2">Forward Primer</h5>
                      <div className="space-y-1 text-sm">
                        <div>
                          <span className="font-mono bg-white px-2 py-1 rounded">
                            {pair.forward.sequence}
                          </span>
                        </div>
                        <div className="text-green-700">
                          Position: {pair.forward.position + 1} |
                          Tm: {pair.forward.tm.toFixed(1)}°C |
                          GC: {pair.forward.gc_content.toFixed(1)}%
                        </div>
                      </div>
                    </div>

                    {/* Reverse Primer */}
                    <div className="bg-blue-50 rounded-lg p-3">
                      <h5 className="font-medium text-blue-800 mb-2">Reverse Primer</h5>
                      <div className="space-y-1 text-sm">
                        <div>
                          <span className="font-mono bg-white px-2 py-1 rounded">
                            {pair.reverse.sequence}
                          </span>
                        </div>
                        <div className="text-blue-700">
                          Position: {pair.reverse.position + 1} |
                          Tm: {pair.reverse.tm.toFixed(1)}°C |
                          GC: {pair.reverse.gc_content.toFixed(1)}%
                        </div>
                      </div>
                    </div>
                  </div>

                  {/* Validation Results */}
                  <div className="mt-3 pt-3 border-t border-gray-200">
                    <div className="flex items-center space-x-4 text-sm">
                      <span className={`px-2 py-1 rounded ${
                        pair.validation_results.self_dimer_check
                          ? 'bg-green-100 text-green-800'
                          : 'bg-red-100 text-red-800'
                      }`}>
                        {pair.validation_results.self_dimer_check ? '✓' : '✗'} Self-Dimer
                      </span>
                      <span className={`px-2 py-1 rounded ${
                        pair.validation_results.hairpin_check
                          ? 'bg-green-100 text-green-800'
                          : 'bg-red-100 text-red-800'
                      }`}>
                        {pair.validation_results.hairpin_check ? '✓' : '✗'} Hairpin
                      </span>
                      {pair.validation_results.warnings.length > 0 && (
                        <span className="px-2 py-1 rounded bg-yellow-100 text-yellow-800">
                          ⚠️ {pair.validation_results.warnings.length} warnings
                        </span>
                      )}
                    </div>
                  </div>
                </div>
              ))}

              {designResult.pairs.length > 5 && (
                <div className="text-center py-2 text-gray-500">
                  ... and {designResult.pairs.length - 5} more primer pairs
                </div>
              )}
            </div>
          )}
        </div>
      )}

      {/* No Sequence Warning */}
      {!sequenceId && (
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <div className="flex">
            <div className="text-yellow-400">💡</div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-yellow-800">No Sequence Loaded</h3>
              <p className="text-sm text-yellow-700 mt-1">
                Please import a sequence first using the Import tab to design primers.
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default PrimerDesign;