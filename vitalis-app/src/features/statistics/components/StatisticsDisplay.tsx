import { SequenceStats } from "../../../types/sequence";
import { StatCard } from "./StatCard";

interface StatisticsDisplayProps {
  stats: SequenceStats;
}

export const StatisticsDisplay = ({ stats }: StatisticsDisplayProps) => {
  return (
    <div className="mt-6 bg-gray-50 rounded-lg p-6">
      <h3 className="text-xl font-semibold text-gray-900 mb-4">Sequence Statistics</h3>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <StatCard
          label="Length"
          value={`${stats.length} bp`}
          color="text-gray-900"
        />
        <StatCard
          label="GC Content"
          value={`${stats.gc_percent.toFixed(2)}%`}
          color="text-blue-600"
        />
        <StatCard
          label="AT Content"
          value={`${stats.at_percent.toFixed(2)}%`}
          color="text-green-600"
        />
        <StatCard
          label="N Content"
          value={`${stats.n_percent.toFixed(2)}%`}
          color="text-gray-500"
        />
      </div>
    </div>
  );
};