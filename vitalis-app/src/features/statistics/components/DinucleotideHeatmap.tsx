import { useMemo } from "react";

interface DinucleotideHeatmapProps {
  dinucleotides: Record<string, number>;
}

export const DinucleotideHeatmap = ({ dinucleotides }: DinucleotideHeatmapProps) => {
  const { heatmapData, maxCount, totalCount } = useMemo(() => {
    const bases = ['A', 'T', 'G', 'C'];
    const totalCount = Object.values(dinucleotides).reduce((sum, count) => sum + count, 0);

    // Create a 4x4 matrix for all possible dinucleotides
    const heatmapData = bases.map(first =>
      bases.map(second => {
        const dinuc = `${first}${second}`;
        const count = dinucleotides[dinuc] || 0;
        const frequency = totalCount > 0 ? (count / totalCount) * 100 : 0;
        return {
          dinucleotide: dinuc,
          count,
          frequency,
          first,
          second,
        };
      })
    );

    const maxCount = Math.max(...Object.values(dinucleotides));

    return { heatmapData, maxCount, totalCount };
  }, [dinucleotides]);

  // Sort dinucleotides by frequency for the list view
  const sortedDinucleotides = useMemo(() => {
    return Object.entries(dinucleotides)
      .map(([dinuc, count]) => ({
        dinucleotide: dinuc,
        count,
        frequency: totalCount > 0 ? (count / totalCount) * 100 : 0,
      }))
      .sort((a, b) => b.count - a.count);
  }, [dinucleotides, totalCount]);

  const getHeatmapColor = (count: number): string => {
    if (maxCount === 0) return 'rgb(241, 245, 249)'; // gray-100

    const intensity = count / maxCount;
    // Create a blue gradient from light to dark
    const lightness = 90 - (intensity * 60); // 90% to 30% lightness
    const saturation = 60 + (intensity * 30); // 60% to 90% saturation
    return `hsl(217, ${saturation}%, ${lightness}%)`;
  };

  const getBaseColor = (base: string): string => {
    const colors: Record<string, string> = {
      'A': '#EF4444', // red
      'T': '#3B82F6', // blue
      'G': '#10B981', // green
      'C': '#F59E0B', // yellow
    };
    return colors[base] || '#6B7280';
  };

  return (
    <div className="space-y-6">
      {/* Heatmap Visualization */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Dinucleotide Frequency Heatmap</h4>

        <div className="flex justify-center">
          <div className="inline-block bg-white rounded-lg p-4 shadow-sm">
            {/* Column headers (second base) */}
            <div className="flex mb-2">
              <div className="w-12 h-12"></div> {/* Empty corner */}
              {['A', 'T', 'G', 'C'].map(base => (
                <div
                  key={base}
                  className="w-12 h-12 flex items-center justify-center font-bold text-white rounded"
                  style={{ backgroundColor: getBaseColor(base) }}
                >
                  {base}
                </div>
              ))}
            </div>

            {/* Heatmap rows */}
            {heatmapData.map((row, rowIndex) => (
              <div key={rowIndex} className="flex">
                {/* Row header (first base) */}
                <div
                  className="w-12 h-12 flex items-center justify-center font-bold text-white rounded mr-2"
                  style={{ backgroundColor: getBaseColor(row[0].first) }}
                >
                  {row[0].first}
                </div>

                {/* Heatmap cells */}
                {row.map((cell, colIndex) => (
                  <div
                    key={colIndex}
                    className="w-12 h-12 border border-gray-200 flex items-center justify-center text-xs font-semibold relative group cursor-pointer"
                    style={{ backgroundColor: getHeatmapColor(cell.count) }}
                  >
                    {/* Cell label */}
                    <span className={`${cell.count > maxCount * 0.5 ? 'text-white' : 'text-gray-700'}`}>
                      {cell.dinucleotide}
                    </span>

                    {/* Tooltip */}
                    <div className="absolute bottom-full mb-2 hidden group-hover:block bg-black text-white text-xs rounded px-2 py-1 whitespace-nowrap z-10">
                      {cell.dinucleotide}: {cell.count.toLocaleString()} ({cell.frequency.toFixed(2)}%)
                    </div>
                  </div>
                ))}
              </div>
            ))}
          </div>
        </div>

        {/* Legend */}
        <div className="mt-4 flex justify-center">
          <div className="flex items-center space-x-4 text-sm text-gray-600">
            <span>Low frequency</span>
            <div className="flex space-x-1">
              {[0.2, 0.4, 0.6, 0.8, 1.0].map(intensity => (
                <div
                  key={intensity}
                  className="w-6 h-4 border border-gray-300"
                  style={{ backgroundColor: getHeatmapColor(maxCount * intensity) }}
                />
              ))}
            </div>
            <span>High frequency</span>
          </div>
        </div>
      </div>

      {/* Top Dinucleotides List */}
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Top Dinucleotides</h4>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {sortedDinucleotides.slice(0, 16).map((item, index) => (
            <div key={item.dinucleotide} className="bg-white rounded-lg p-3 shadow-sm border">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-3">
                  <div className="w-8 h-8 bg-gray-100 rounded flex items-center justify-center">
                    <span className="text-sm font-bold text-gray-700">#{index + 1}</span>
                  </div>
                  <div>
                    <div className="font-mono font-bold text-lg">
                      <span style={{ color: getBaseColor(item.dinucleotide[0]) }}>
                        {item.dinucleotide[0]}
                      </span>
                      <span style={{ color: getBaseColor(item.dinucleotide[1]) }}>
                        {item.dinucleotide[1]}
                      </span>
                    </div>
                    <div className="text-xs text-gray-500">
                      {item.frequency.toFixed(2)}%
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-semibold text-gray-900">
                    {item.count.toLocaleString()}
                  </div>
                  <div className="w-16 bg-gray-200 rounded-full h-2 mt-1">
                    <div
                      className="h-2 rounded-full bg-blue-500 transition-all duration-500"
                      style={{
                        width: `${(item.count / maxCount) * 100}%`,
                      }}
                    />
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Summary Statistics */}
      <div className="bg-white rounded-lg p-6 shadow-sm border">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Dinucleotide Summary</h4>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{sortedDinucleotides.length}</div>
            <div className="text-sm text-gray-600">Unique Dinucleotides</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">{totalCount.toLocaleString()}</div>
            <div className="text-sm text-gray-600">Total Dinucleotides</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600">
              {sortedDinucleotides[0]?.dinucleotide || 'N/A'}
            </div>
            <div className="text-sm text-gray-600">Most Frequent</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600">
              {maxCount.toLocaleString()}
            </div>
            <div className="text-sm text-gray-600">Max Frequency</div>
          </div>
        </div>
      </div>
    </div>
  );
};