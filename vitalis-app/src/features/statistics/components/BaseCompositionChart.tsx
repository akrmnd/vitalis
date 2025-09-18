import { BaseCount } from "../../../types/sequence";
import { useMemo } from "react";

interface BaseCompositionChartProps {
  baseCount: BaseCount;
}

export const BaseCompositionChart = ({ baseCount }: BaseCompositionChartProps) => {
  const data = useMemo(() => {
    const total = baseCount.a + baseCount.t + baseCount.g + baseCount.c + baseCount.n + baseCount.other;
    return [
      { base: 'A', count: baseCount.a, percentage: (baseCount.a / total) * 100, color: '#EF4444' },
      { base: 'T', count: baseCount.t, percentage: (baseCount.t / total) * 100, color: '#3B82F6' },
      { base: 'G', count: baseCount.g, percentage: (baseCount.g / total) * 100, color: '#10B981' },
      { base: 'C', count: baseCount.c, percentage: (baseCount.c / total) * 100, color: '#F59E0B' },
      { base: 'N', count: baseCount.n, percentage: (baseCount.n / total) * 100, color: '#6B7280' },
      { base: 'Other', count: baseCount.other, percentage: (baseCount.other / total) * 100, color: '#8B5CF6' },
    ].filter(item => item.count > 0);
  }, [baseCount]);

  const maxCount = Math.max(...data.map(d => d.count));

  return (
    <div className="bg-gray-50 rounded-lg p-6">
      <h4 className="text-lg font-semibold text-gray-900 mb-4">Base Composition</h4>

      {/* Bar Chart */}
      <div className="space-y-3 mb-6">
        {data.map((item) => (
          <div key={item.base} className="flex items-center space-x-4">
            <div className="w-8 text-center font-mono font-bold" style={{ color: item.color }}>
              {item.base}
            </div>
            <div className="flex-1">
              <div className="flex items-center justify-between text-sm text-gray-600 mb-1">
                <span>{item.count.toLocaleString()} bases</span>
                <span>{item.percentage.toFixed(2)}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-4">
                <div
                  className="h-4 rounded-full transition-all duration-500"
                  style={{
                    width: `${(item.count / maxCount) * 100}%`,
                    backgroundColor: item.color,
                  }}
                />
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Pie Chart Representation */}
      <div className="flex justify-center">
        <div className="relative w-32 h-32">
          <svg width="128" height="128" className="transform -rotate-90">
            <circle
              cx="64"
              cy="64"
              r="60"
              fill="none"
              stroke="#E5E7EB"
              strokeWidth="8"
            />
            {data.reduce((acc, item) => {
              const angle = (item.percentage / 100) * 360;
              const startAngle = acc.currentAngle;
              const endAngle = startAngle + angle;

              const startX = 64 + 60 * Math.cos((startAngle * Math.PI) / 180);
              const startY = 64 + 60 * Math.sin((startAngle * Math.PI) / 180);
              const endX = 64 + 60 * Math.cos((endAngle * Math.PI) / 180);
              const endY = 64 + 60 * Math.sin((endAngle * Math.PI) / 180);

              const largeArcFlag = angle > 180 ? 1 : 0;

              const pathData = [
                `M 64 64`,
                `L ${startX} ${startY}`,
                `A 60 60 0 ${largeArcFlag} 1 ${endX} ${endY}`,
                'Z'
              ].join(' ');

              acc.elements.push(
                <path
                  key={item.base}
                  d={pathData}
                  fill={item.color}
                  opacity="0.8"
                />
              );

              acc.currentAngle = endAngle;
              return acc;
            }, { elements: [] as React.JSX.Element[], currentAngle: 0 }).elements}
          </svg>
        </div>
      </div>
    </div>
  );
};