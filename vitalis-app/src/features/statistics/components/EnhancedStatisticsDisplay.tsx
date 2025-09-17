import { DetailedStats } from "../../../types/sequence";
import { StatCard } from "./StatCard";
import { BaseCompositionChart } from "./BaseCompositionChart";
import { CodonUsageChart } from "./CodonUsageChart";
import { QualityDistributionChart } from "./QualityDistributionChart";
import { DinucleotideHeatmap } from "./DinucleotideHeatmap";
import { useState } from "react";

interface EnhancedStatisticsDisplayProps {
  stats: DetailedStats;
}

export const EnhancedStatisticsDisplay = ({ stats }: EnhancedStatisticsDisplayProps) => {
  const [activeTab, setActiveTab] = useState<'overview' | 'composition' | 'codon' | 'quality'>('overview');

  const tabs = [
    { id: 'overview', label: 'Overview', icon: '📊' },
    { id: 'composition', label: 'Composition', icon: '🧬' },
    ...(stats.codon_usage ? [{ id: 'codon', label: 'Codon Usage', icon: '🔤' }] : []),
    ...(stats.quality_stats ? [{ id: 'quality', label: 'Quality', icon: '⭐' }] : []),
  ];

  return (
    <div className="mt-6 bg-white rounded-lg shadow-lg">
      <div className="border-b border-gray-200">
        <nav className="flex space-x-8 px-6">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`py-4 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6">
        {activeTab === 'overview' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Basic Statistics</h3>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <StatCard
                  label="Length"
                  value={`${stats.basic.length.toLocaleString()} bp`}
                  color="text-gray-900"
                />
                <StatCard
                  label="GC Content"
                  value={`${stats.basic.gc_percent.toFixed(2)}%`}
                  color="text-blue-600"
                />
                <StatCard
                  label="AT Content"
                  value={`${stats.basic.at_percent.toFixed(2)}%`}
                  color="text-green-600"
                />
                <StatCard
                  label="N Content"
                  value={`${stats.basic.n_percent.toFixed(2)}%`}
                  color="text-gray-500"
                />
              </div>
            </div>

            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Advanced Metrics</h3>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <StatCard
                  label="GC Skew"
                  value={stats.basic.gc_skew.toFixed(3)}
                  color="text-purple-600"
                />
                <StatCard
                  label="AT Skew"
                  value={stats.basic.at_skew.toFixed(3)}
                  color="text-orange-600"
                />
                <StatCard
                  label="Entropy"
                  value={stats.basic.entropy.toFixed(3)}
                  color="text-indigo-600"
                />
                <StatCard
                  label="Complexity"
                  value={stats.basic.complexity.toFixed(3)}
                  color="text-red-600"
                />
              </div>
            </div>

            {stats.codon_usage && (
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Coding Sequence Summary</h3>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <StatCard
                    label="Start Codons"
                    value={stats.codon_usage.start_codons.toString()}
                    color="text-green-600"
                  />
                  <StatCard
                    label="Stop Codons"
                    value={stats.codon_usage.stop_codons.toString()}
                    color="text-red-600"
                  />
                  <StatCard
                    label="Rare Codons"
                    value={stats.codon_usage.rare_codons.length.toString()}
                    color="text-yellow-600"
                  />
                  <StatCard
                    label="Unique Codons"
                    value={Object.keys(stats.codon_usage.codon_counts).length.toString()}
                    color="text-blue-600"
                  />
                </div>
              </div>
            )}

            {stats.quality_stats && (
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Quality Summary</h3>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <StatCard
                    label="Mean Quality"
                    value={stats.quality_stats.mean_quality.toFixed(1)}
                    color="text-green-600"
                  />
                  <StatCard
                    label="Q20 Bases"
                    value={`${((stats.quality_stats.q20_bases / stats.basic.length) * 100).toFixed(1)}%`}
                    color="text-blue-600"
                  />
                  <StatCard
                    label="Q30 Bases"
                    value={`${((stats.quality_stats.q30_bases / stats.basic.length) * 100).toFixed(1)}%`}
                    color="text-purple-600"
                  />
                  <StatCard
                    label="Quality Range"
                    value={`${stats.quality_stats.min_quality}-${stats.quality_stats.max_quality}`}
                    color="text-gray-600"
                  />
                </div>
              </div>
            )}
          </div>
        )}

        {activeTab === 'composition' && (
          <div className="space-y-6">
            <BaseCompositionChart baseCount={stats.base_counts} />
            <DinucleotideHeatmap dinucleotides={stats.dinucleotide_counts} />
          </div>
        )}

        {activeTab === 'codon' && stats.codon_usage && (
          <div className="space-y-6">
            <CodonUsageChart codonUsage={stats.codon_usage} />
          </div>
        )}

        {activeTab === 'quality' && stats.quality_stats && (
          <div className="space-y-6">
            <QualityDistributionChart qualityStats={stats.quality_stats} />
          </div>
        )}
      </div>
    </div>
  );
};