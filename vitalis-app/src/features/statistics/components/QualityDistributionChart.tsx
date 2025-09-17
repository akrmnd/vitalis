import { QualityStats } from "../../../types/sequence";
import { useMemo } from "react";

interface QualityDistributionChartProps {
  qualityStats: QualityStats;
}

export const QualityDistributionChart = ({ qualityStats }: QualityDistributionChartProps) => {
  const { distributionData, qualityMetrics } = useMemo(() => {
    // Prepare quality distribution data
    const distributionData = Object.entries(qualityStats.quality_distribution)
      .map(([quality, count]) => ({
        quality: parseInt(quality),
        count,
        percentage: (count / Object.values(qualityStats.quality_distribution).reduce((a, b) => a + b, 0)) * 100,
      }))
      .sort((a, b) => a.quality - b.quality);

    // Calculate quality metrics
    const totalBases = Object.values(qualityStats.quality_distribution).reduce((a, b) => a + b, 0);
    const q20Percentage = (qualityStats.q20_bases / totalBases) * 100;
    const q30Percentage = (qualityStats.q30_bases / totalBases) * 100;

    const qualityMetrics = {
      totalBases,
      q20Percentage,
      q30Percentage,
      qualityRange: qualityStats.max_quality - qualityStats.min_quality,
    };

    return { distributionData, qualityMetrics };
  }, [qualityStats]);

  const maxCount = Math.max(...distributionData.map(d => d.count));

  const getQualityColor = (quality: number): string => {
    if (quality >= 30) return '#10B981'; // Green for high quality
    if (quality >= 20) return '#F59E0B'; // Yellow for medium quality
    return '#EF4444'; // Red for low quality
  };

  return (
    <div className="space-y-6">
      {/* Quality Summary Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div className="bg-white rounded-lg p-4 shadow-sm border">
          <div className="text-2xl font-bold text-green-600">{qualityStats.mean_quality.toFixed(1)}</div>
          <div className="text-sm text-gray-600">Mean Quality</div>
        </div>
        <div className="bg-white rounded-lg p-4 shadow-sm border">
          <div className="text-2xl font-bold text-blue-600">{qualityStats.median_quality}</div>
          <div className="text-sm text-gray-600">Median Quality</div>
        </div>
        <div className="bg-white rounded-lg p-4 shadow-sm border">
          <div className="text-2xl font-bold text-green-600">{qualityMetrics.q20Percentage.toFixed(1)}%</div>
          <div className="text-sm text-gray-600">Q20+ Bases</div>
        </div>
        <div className="bg-white rounded-lg p-4 shadow-sm border">
          <div className="text-2xl font-bold text-purple-600">{qualityMetrics.q30Percentage.toFixed(1)}%</div>
          <div className="text-sm text-gray-600">Q30+ Bases</div>
        </div>
      </div>

      {/* Quality Distribution Histogram */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Quality Score Distribution</h4>

        {/* Chart */}
        <div className="relative h-64 border-l-2 border-b-2 border-gray-300">
          {/* Y-axis labels */}
          <div className="absolute -left-12 top-0 h-full flex flex-col justify-between text-xs text-gray-600">
            <span>{maxCount.toLocaleString()}</span>
            <span>{Math.round(maxCount * 0.75).toLocaleString()}</span>
            <span>{Math.round(maxCount * 0.5).toLocaleString()}</span>
            <span>{Math.round(maxCount * 0.25).toLocaleString()}</span>
            <span>0</span>
          </div>

          {/* Bars */}
          <div className="h-full flex items-end justify-start space-x-1 ml-2">
            {distributionData.map((item) => (
              <div key={item.quality} className="flex flex-col items-center group relative">
                <div
                  className="w-4 transition-all duration-300 hover:opacity-80"
                  style={{
                    height: `${(item.count / maxCount) * 100}%`,
                    backgroundColor: getQualityColor(item.quality),
                    minHeight: item.count > 0 ? '2px' : '0px',
                  }}
                />

                {/* Tooltip */}
                <div className="absolute bottom-full mb-2 hidden group-hover:block bg-black text-white text-xs rounded px-2 py-1 whitespace-nowrap z-10">
                  Q{item.quality}: {item.count.toLocaleString()} ({item.percentage.toFixed(1)}%)
                </div>
              </div>
            ))}
          </div>

          {/* X-axis labels */}
          <div className="absolute -bottom-6 left-2 right-0 flex justify-between text-xs text-gray-600">
            <span>0</span>
            <span>10</span>
            <span>20</span>
            <span>30</span>
            <span>40+</span>
          </div>
        </div>

        {/* Legend */}
        <div className="mt-6 flex justify-center space-x-6 text-sm">
          <div className="flex items-center space-x-2">
            <div className="w-4 h-4 bg-red-500 rounded"></div>
            <span>Low Quality (&lt;20)</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-4 h-4 bg-yellow-500 rounded"></div>
            <span>Medium Quality (20-29)</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-4 h-4 bg-green-500 rounded"></div>
            <span>High Quality (≥30)</span>
          </div>
        </div>
      </div>

      {/* Detailed Quality Statistics */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Quality Statistics Details</h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">

          {/* Quality Range */}
          <div className="bg-white rounded-lg p-4 shadow-sm">
            <h5 className="text-md font-semibold text-gray-900 mb-3">Quality Range</h5>
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-600">Minimum:</span>
                <span className="font-semibold" style={{ color: getQualityColor(qualityStats.min_quality) }}>
                  Q{qualityStats.min_quality}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Maximum:</span>
                <span className="font-semibold" style={{ color: getQualityColor(qualityStats.max_quality) }}>
                  Q{qualityStats.max_quality}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Range:</span>
                <span className="font-semibold text-gray-900">
                  {qualityMetrics.qualityRange} points
                </span>
              </div>
            </div>
          </div>

          {/* Quality Thresholds */}
          <div className="bg-white rounded-lg p-4 shadow-sm">
            <h5 className="text-md font-semibold text-gray-900 mb-3">Quality Thresholds</h5>
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-600">Q20+ Bases:</span>
                <span className="font-semibold text-green-600">
                  {qualityStats.q20_bases.toLocaleString()} ({qualityMetrics.q20Percentage.toFixed(1)}%)
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Q30+ Bases:</span>
                <span className="font-semibold text-purple-600">
                  {qualityStats.q30_bases.toLocaleString()} ({qualityMetrics.q30Percentage.toFixed(1)}%)
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Total Bases:</span>
                <span className="font-semibold text-gray-900">
                  {qualityMetrics.totalBases.toLocaleString()}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};