import React, { useState, useMemo } from 'react';
import { GenBankMetadata, GenBankFeature } from '../types/genbank';

interface CircularGenomeVisualizationProps {
  sequence: string;
  metadata?: GenBankMetadata;
  diameter?: number;
}

export const CircularGenomeVisualization: React.FC<CircularGenomeVisualizationProps> = ({
  sequence,
  metadata,
  diameter = 400,
}) => {
  const [selectedFeature, setSelectedFeature] = useState<GenBankFeature | null>(null);
  const [rotation, setRotation] = useState(0);

  const center = diameter / 2;
  const radius = (diameter - 80) / 2;
  const innerRadius = radius - 60;

  // Feature colors
  const FEATURE_COLORS: Record<string, string> = {
    gene: '#22c55e',
    CDS: '#3b82f6',
    tRNA: '#f59e0b',
    rRNA: '#ef4444',
    repeat_region: '#8b5cf6',
    promoter: '#06b6d4',
    terminator: '#84cc16',
    misc_feature: '#6b7280',
    source: '#f97316',
    exon: '#10b981',
    intron: '#64748b',
  };

  // Parse location string
  const parseLocation = (location: string): { start: number; end: number } => {
    const match = location.match(/(\d+)\.\.(\d+)/);
    if (match) {
      return {
        start: parseInt(match[1]) - 1,
        end: parseInt(match[2]) - 1,
      };
    }
    const singleMatch = location.match(/(\d+)/);
    if (singleMatch) {
      const pos = parseInt(singleMatch[1]) - 1;
      return { start: pos, end: pos };
    }
    return { start: 0, end: 0 };
  };

  // Convert position to angle
  const positionToAngle = (position: number): number => {
    return ((position / sequence.length) * 360 + rotation) % 360;
  };

  // Convert angle to SVG coordinates
  const angleToCoords = (angle: number, r: number): { x: number; y: number } => {
    const radians = (angle - 90) * (Math.PI / 180);
    return {
      x: center + r * Math.cos(radians),
      y: center + r * Math.sin(radians),
    };
  };

  // Create arc path
  const createArcPath = (startAngle: number, endAngle: number, innerR: number, outerR: number): string => {
    if (endAngle < startAngle) endAngle += 360;

    const startInner = angleToCoords(startAngle, innerR);
    const endInner = angleToCoords(endAngle, innerR);
    const startOuter = angleToCoords(startAngle, outerR);
    const endOuter = angleToCoords(endAngle, outerR);

    const largeArcFlag = endAngle - startAngle > 180 ? 1 : 0;

    return [
      `M ${startInner.x} ${startInner.y}`,
      `L ${startOuter.x} ${startOuter.y}`,
      `A ${outerR} ${outerR} 0 ${largeArcFlag} 1 ${endOuter.x} ${endOuter.y}`,
      `L ${endInner.x} ${endInner.y}`,
      `A ${innerR} ${innerR} 0 ${largeArcFlag} 0 ${startInner.x} ${startInner.y}`,
      'Z'
    ].join(' ');
  };

  // Group features by type
  const featureGroups = useMemo(() => {
    if (!metadata?.features) return [];

    const groups = new Map<string, GenBankFeature[]>();
    metadata.features.forEach(feature => {
      if (!groups.has(feature.feature_type)) {
        groups.set(feature.feature_type, []);
      }
      groups.get(feature.feature_type)!.push(feature);
    });

    return Array.from(groups.entries()).map(([type, features], index) => ({
      type,
      features,
      color: FEATURE_COLORS[type] || '#9ca3af',
      trackRadius: innerRadius + (index * 15),
    }));
  }, [metadata?.features, innerRadius]);

  // Generate tick marks for positions
  const generateTicks = () => {
    const ticks = [];
    const tickCount = 20;

    for (let i = 0; i < tickCount; i++) {
      const angle = (i * 360) / tickCount;
      const position = Math.round((i * sequence.length) / tickCount);
      const innerCoord = angleToCoords(angle, radius - 10);
      const outerCoord = angleToCoords(angle, radius);
      const labelCoord = angleToCoords(angle, radius + 15);

      ticks.push(
        <g key={i}>
          <line
            x1={innerCoord.x}
            y1={innerCoord.y}
            x2={outerCoord.x}
            y2={outerCoord.y}
            stroke="#6b7280"
            strokeWidth={1}
          />
          <text
            x={labelCoord.x}
            y={labelCoord.y}
            textAnchor="middle"
            dominantBaseline="middle"
            fontSize={10}
            fill="#6b7280"
          >
            {position.toLocaleString()}
          </text>
        </g>
      );
    }
    return ticks;
  };

  return (
    <div className="bg-white rounded-lg border border-gray-200 p-4">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">Circular Genome Map</h3>
        <div className="flex items-center space-x-2">
          <label className="text-sm text-gray-600">Rotation:</label>
          <input
            type="range"
            min="0"
            max="360"
            value={rotation}
            onChange={(e) => setRotation(parseInt(e.target.value))}
            className="w-24"
          />
          <span className="text-sm text-gray-500">{rotation}°</span>
        </div>
      </div>

      <div className="flex justify-center">
        <svg width={diameter} height={diameter} className="border border-gray-200 rounded-full bg-gray-50">
          {/* Background circle */}
          <circle
            cx={center}
            cy={center}
            r={radius}
            fill="none"
            stroke="#e5e7eb"
            strokeWidth={2}
          />

          {/* Sequence length indicator */}
          <text
            x={center}
            y={center - 10}
            textAnchor="middle"
            fontSize={14}
            fontWeight="bold"
            fill="#374151"
          >
            {sequence.length.toLocaleString()} bp
          </text>

          {metadata?.topology && (
            <text
              x={center}
              y={center + 10}
              textAnchor="middle"
              fontSize={12}
              fill="#6b7280"
            >
              {metadata.topology}
            </text>
          )}

          {/* Position ticks */}
          {generateTicks()}

          {/* Feature tracks */}
          {featureGroups.map((group, groupIndex) => (
            <g key={group.type}>
              {group.features.map((feature, featureIndex) => {
                const { start, end } = parseLocation(feature.location);
                let startAngle = positionToAngle(start);
                let endAngle = positionToAngle(end);

                // Handle wrap-around
                if (end < start) {
                  endAngle += 360;
                }

                const trackInnerRadius = group.trackRadius;
                const trackOuterRadius = group.trackRadius + 12;

                const arcPath = createArcPath(startAngle, endAngle, trackInnerRadius, trackOuterRadius);

                return (
                  <path
                    key={`${group.type}-${featureIndex}`}
                    d={arcPath}
                    fill={group.color}
                    stroke={selectedFeature === feature ? "#1f2937" : "none"}
                    strokeWidth={selectedFeature === feature ? 2 : 0}
                    opacity={0.8}
                    style={{ cursor: 'pointer' }}
                    onClick={() => setSelectedFeature(selectedFeature === feature ? null : feature)}
                  >
                    <title>{`${feature.feature_type}: ${feature.location}`}</title>
                  </path>
                );
              })}
            </g>
          ))}

          {/* Track labels */}
          {featureGroups.map((group, index) => (
            <text
              key={`label-${group.type}`}
              x={20}
              y={30 + index * 15}
              fontSize={11}
              fill={group.color}
              fontWeight="500"
            >
              {group.type} ({group.features.length})
            </text>
          ))}
        </svg>
      </div>

      {/* Feature Legend */}
      {featureGroups.length > 0 && (
        <div className="mt-4">
          <h4 className="text-sm font-medium text-gray-700 mb-2">Feature Distribution</h4>
          <div className="grid grid-cols-3 gap-3">
            {featureGroups.map(group => (
              <div key={group.type} className="flex items-center space-x-2">
                <div
                  className="w-4 h-4 rounded"
                  style={{ backgroundColor: group.color }}
                />
                <span className="text-sm text-gray-600 capitalize">
                  {group.type} ({group.features.length})
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Selected Feature Info */}
      {selectedFeature && (
        <div className="mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
          <h4 className="text-sm font-medium text-blue-900 mb-2">
            Selected: {selectedFeature.feature_type}
          </h4>
          <div className="text-sm text-blue-800 space-y-1">
            <div><strong>Location:</strong> {selectedFeature.location}</div>
            {selectedFeature.qualifiers.gene && (
              <div><strong>Gene:</strong> {selectedFeature.qualifiers.gene}</div>
            )}
            {selectedFeature.qualifiers.product && (
              <div><strong>Product:</strong> {selectedFeature.qualifiers.product}</div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};