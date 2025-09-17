import { CodonUsage } from "../../../types/sequence";
import { useMemo } from "react";

interface CodonUsageChartProps {
  codonUsage: CodonUsage;
}

export const CodonUsageChart = ({ codonUsage }: CodonUsageChartProps) => {
  const { topCodons, aminoAcidData, rareCodonData } = useMemo(() => {
    // Top 20 most frequent codons
    const topCodons = Object.entries(codonUsage.codon_frequencies)
      .sort(([, a], [, b]) => b - a)
      .slice(0, 20)
      .map(([codon, frequency]) => ({
        codon,
        frequency,
        count: codonUsage.codon_counts[codon] || 0,
        color: getCodonColor(codon),
      }));

    // Amino acid usage
    const aminoAcidData = Object.entries(codonUsage.amino_acid_counts)
      .sort(([, a], [, b]) => b - a)
      .map(([aminoAcid, count]) => ({
        aminoAcid,
        count,
        color: getAminoAcidColor(aminoAcid),
      }));

    // Rare codons analysis
    const rareCodonData = codonUsage.rare_codons.map(codon => ({
      codon,
      frequency: codonUsage.codon_frequencies[codon] || 0,
      count: codonUsage.codon_counts[codon] || 0,
    }));

    return { topCodons, aminoAcidData, rareCodonData };
  }, [codonUsage]);

  const maxCodonFreq = Math.max(...topCodons.map(c => c.frequency));
  const maxAminoAcidCount = Math.max(...aminoAcidData.map(a => a.count));

  return (
    <div className="space-y-8">
      {/* Codon Frequency Chart */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Top 20 Codon Usage</h4>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          {topCodons.map((item) => (
            <div key={item.codon} className="bg-white rounded-lg p-3 shadow-sm">
              <div className="flex items-center justify-between mb-2">
                <span className="font-mono font-bold text-sm" style={{ color: item.color }}>
                  {item.codon}
                </span>
                <span className="text-xs text-gray-600">{item.count}</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className="h-2 rounded-full transition-all duration-500"
                  style={{
                    width: `${(item.frequency / maxCodonFreq) * 100}%`,
                    backgroundColor: item.color,
                  }}
                />
              </div>
              <div className="text-xs text-gray-500 mt-1">
                {(item.frequency * 100).toFixed(2)}%
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Amino Acid Distribution */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Amino Acid Distribution</h4>
        <div className="space-y-2">
          {aminoAcidData.slice(0, 15).map((item) => (
            <div key={item.aminoAcid} className="flex items-center space-x-4">
              <div className="w-8 text-center font-mono font-bold text-sm" style={{ color: item.color }}>
                {item.aminoAcid}
              </div>
              <div className="flex-1">
                <div className="flex items-center justify-between text-sm text-gray-600 mb-1">
                  <span>{item.count.toLocaleString()} residues</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-3">
                  <div
                    className="h-3 rounded-full transition-all duration-500"
                    style={{
                      width: `${(item.count / maxAminoAcidCount) * 100}%`,
                      backgroundColor: item.color,
                    }}
                  />
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Codon Usage Summary */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white rounded-lg p-6 shadow-sm border">
          <h5 className="text-md font-semibold text-gray-900 mb-3">Start/Stop Codons</h5>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Start Codons:</span>
              <span className="font-semibold text-green-600">{codonUsage.start_codons}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Stop Codons:</span>
              <span className="font-semibold text-red-600">{codonUsage.stop_codons}</span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg p-6 shadow-sm border">
          <h5 className="text-md font-semibold text-gray-900 mb-3">Codon Diversity</h5>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Unique Codons:</span>
              <span className="font-semibold text-blue-600">
                {Object.keys(codonUsage.codon_counts).length}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Total Codons:</span>
              <span className="font-semibold text-gray-900">
                {Object.values(codonUsage.codon_counts).reduce((a, b) => a + b, 0).toLocaleString()}
              </span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg p-6 shadow-sm border">
          <h5 className="text-md font-semibold text-gray-900 mb-3">Rare Codons</h5>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Count:</span>
              <span className="font-semibold text-yellow-600">{rareCodonData.length}</span>
            </div>
            {rareCodonData.length > 0 && (
              <div className="mt-2">
                <div className="flex flex-wrap gap-1">
                  {rareCodonData.slice(0, 6).map((rare) => (
                    <span
                      key={rare.codon}
                      className="inline-block px-2 py-1 text-xs font-mono bg-yellow-100 text-yellow-800 rounded"
                    >
                      {rare.codon}
                    </span>
                  ))}
                  {rareCodonData.length > 6 && (
                    <span className="inline-block px-2 py-1 text-xs bg-gray-100 text-gray-600 rounded">
                      +{rareCodonData.length - 6} more
                    </span>
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

// Helper functions for color assignment
function getCodonColor(codon: string): string {
  const colors = [
    '#EF4444', '#3B82F6', '#10B981', '#F59E0B', '#8B5CF6', '#EC4899',
    '#06B6D4', '#84CC16', '#F97316', '#6366F1', '#14B8A6', '#F472B6',
    '#22D3EE', '#A3E635', '#FB923C', '#818CF8', '#34D399', '#FBBF24'
  ];
  let hash = 0;
  for (let i = 0; i < codon.length; i++) {
    hash = codon.charCodeAt(i) + ((hash << 5) - hash);
  }
  return colors[Math.abs(hash) % colors.length];
}

function getAminoAcidColor(aminoAcid: string): string {
  const aaColors: Record<string, string> = {
    'A': '#EF4444', 'R': '#3B82F6', 'N': '#10B981', 'D': '#F59E0B', 'C': '#8B5CF6',
    'Q': '#EC4899', 'E': '#06B6D4', 'G': '#84CC16', 'H': '#F97316', 'I': '#6366F1',
    'L': '#14B8A6', 'K': '#F472B6', 'M': '#22D3EE', 'F': '#A3E635', 'P': '#FB923C',
    'S': '#818CF8', 'T': '#34D399', 'W': '#FBBF24', 'Y': '#F87171', 'V': '#60A5FA',
    '*': '#6B7280' // Stop codon
  };
  return aaColors[aminoAcid] || '#6B7280';
}