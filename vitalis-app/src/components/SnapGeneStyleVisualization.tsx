import React, { useState, useRef, useMemo, useCallback } from 'react';
import { GenBankMetadata, GenBankFeature } from '../types/genbank';
import { useSequenceSelection } from '../hooks/useSequenceSelection';

interface SnapGeneStyleVisualizationProps {
  sequence: string;
  metadata?: GenBankMetadata;
  width?: number;
}

interface FeatureTrack {
  features: GenBankFeature[];
  type: string;
  color: string;
  yOffset: number;
  strand: 'forward' | 'reverse';
}

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

const BASES_PER_LINE = 100;
const BASE_WIDTH = 8;
const LINE_HEIGHT = 150;
const FEATURE_HEIGHT = 20;
const FEATURE_SPACING = 25;
const RULER_HEIGHT = 30;
const SEQUENCE_HEIGHT = 25;

export const SnapGeneStyleVisualization: React.FC<SnapGeneStyleVisualizationProps> = ({
  sequence,
  metadata,
  width = 1000,
}) => {
  const [selectedFeature, setSelectedFeature] = useState<GenBankFeature | null>(null);
  const [showSequence, setShowSequence] = useState(true);
  const [showTranslation, setShowTranslation] = useState(false);
  const [viewStart, setViewStart] = useState(0);
  const [viewEnd, setViewEnd] = useState(Math.min(sequence.length, 1000));

  // Use custom hook for selection
  const {
    selectionStart,
    selectionEnd,
    isSelecting,
    hasSelection,
    tooltipPosition,
    getSelectionInfo,
    clearSelection,
    startSelection,
    updateSelection,
    finishSelection
  } = useSequenceSelection();

  const svgRef = useRef<SVGSVGElement>(null);

  // Calculate how many lines we need
  const totalLines = Math.ceil((viewEnd - viewStart) / BASES_PER_LINE);
  const actualWidth = Math.min(width, BASES_PER_LINE * BASE_WIDTH + 100);

  // Group features by type and strand
  const featureTracks = useMemo(() => {
    if (!metadata?.features) return [];

    const forwardTracks = new Map<string, GenBankFeature[]>();
    const reverseTracks = new Map<string, GenBankFeature[]>();

    metadata.features.forEach(feature => {
      // Skip source features
      if (feature.feature_type === 'source') return;

      const isReverse = feature.location.includes('complement');
      const tracks = isReverse ? reverseTracks : forwardTracks;

      if (!tracks.has(feature.feature_type)) {
        tracks.set(feature.feature_type, []);
      }
      tracks.get(feature.feature_type)!.push(feature);
    });

    const tracks: FeatureTrack[] = [];
    let offset = 0;

    // Forward tracks
    Array.from(forwardTracks.entries()).forEach(([type, features]) => {
      tracks.push({
        type,
        features,
        color: FEATURE_COLORS[type] || '#9ca3af',
        yOffset: offset,
        strand: 'forward',
      });
      offset += FEATURE_SPACING;
    });

    // Reverse tracks
    Array.from(reverseTracks.entries()).forEach(([type, features]) => {
      tracks.push({
        type,
        features,
        color: FEATURE_COLORS[type] || '#9ca3af',
        yOffset: offset,
        strand: 'reverse',
      });
      offset += FEATURE_SPACING;
    });

    return tracks;
  }, [metadata?.features]);

  // Parse location string
  const parseLocation = (location: string): { start: number; end: number; isReverse: boolean } => {
    const isReverse = location.includes('complement');
    const cleanLocation = location.replace(/complement\(|\)/g, '');

    const match = cleanLocation.match(/(\d+)\.\.(\d+)/);
    if (match) {
      return {
        start: parseInt(match[1]) - 1,
        end: parseInt(match[2]) - 1,
        isReverse,
      };
    }

    const singleMatch = cleanLocation.match(/(\d+)/);
    if (singleMatch) {
      const pos = parseInt(singleMatch[1]) - 1;
      return { start: pos, end: pos, isReverse };
    }

    return { start: 0, end: 0, isReverse };
  };

  // Convert SVG coordinates to sequence position
  const coordsToPosition = (x: number, line: number): number => {
    // Calculate base index, ensuring we're within bounds
    const baseIndex = Math.max(0, Math.floor((x - 50 + BASE_WIDTH/2) / BASE_WIDTH));
    const lineStart = viewStart + line * BASES_PER_LINE;
    const lineEnd = Math.min(lineStart + BASES_PER_LINE - 1, viewEnd - 1);
    const position = lineStart + baseIndex;

    return Math.min(Math.max(position, lineStart), lineEnd);
  };

  // Convert SVG coordinates to sequence position (more flexible for dragging)
  const coordsToPositionFlexible = (x: number, y: number): number => {
    // Calculate which line based on Y coordinate
    let line = Math.floor((y - RULER_HEIGHT) / LINE_HEIGHT);
    line = Math.max(0, Math.min(line, totalLines - 1));

    // Calculate base index, allowing for positions outside the exact bounds
    let baseIndex = Math.floor((x - 50 + BASE_WIDTH/2) / BASE_WIDTH);

    // Allow extending beyond line boundaries during drag
    if (baseIndex < 0) baseIndex = 0;
    if (baseIndex >= BASES_PER_LINE) baseIndex = BASES_PER_LINE - 1;

    const lineStart = viewStart + line * BASES_PER_LINE;
    const position = lineStart + baseIndex;

    return Math.min(Math.max(position, viewStart), viewEnd - 1);
  };

  // Handle mouse events for sequence selection

  const handleMouseDown = (event: React.MouseEvent<SVGElement>) => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;


    // Calculate which line was clicked
    const line = Math.floor((y - RULER_HEIGHT) / LINE_HEIGHT);

    // Check if click is within any of the yellow highlighted areas for this line
    const lineStart = viewStart + line * BASES_PER_LINE;
    const lineEnd = Math.min(lineStart + BASES_PER_LINE, viewEnd);
    const yellowAreaTop = RULER_HEIGHT + line * LINE_HEIGHT + SEQUENCE_HEIGHT - 9 - 20;
    const yellowAreaBottom = yellowAreaTop + 52;

    // Check if x coordinate is within any base area on this line
    let isInSelectableArea = false;
    if (showSequence && line >= 0 && line < totalLines && y >= yellowAreaTop && y <= yellowAreaBottom) {
      for (let i = 0; i < (lineEnd - lineStart); i++) {
        const baseX = 50 + i * BASE_WIDTH;
        const leftBound = baseX - 1;
        const rightBound = baseX + BASE_WIDTH + 1;
        if (x >= leftBound && x <= rightBound) {
          isInSelectableArea = true;
          break;
        }
      }
    }

    // Only start selection if clicking within the yellow area
    if (!isInSelectableArea) {
      clearSelection();
      return;
    }

    // Use simple position calculation based on x coordinate
    const position = coordsToPosition(x, line);

    // Check if clicking on existing selection
    const selectionInfo = getSelectionInfo();
    if (hasSelection && selectionInfo && position >= selectionInfo.start && position <= selectionInfo.end) {
      // Clicking on selection keeps it but updates tooltip position
      updateSelection(position, { x: event.clientX, y: event.clientY });
      return;
    }

    // Start new selection
    startSelection(position, { x: event.clientX, y: event.clientY });
    setSelectedFeature(null); // Clear feature selection

    event.preventDefault();
    event.stopPropagation();
  };

  const handleMouseMove = (event: React.MouseEvent<SVGGElement>) => {
    if (!isSelecting || selectionStart === null) return;

    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Use flexible positioning for drag operations
    const position = coordsToPositionFlexible(x, y);
    updateSelection(position, { x: event.clientX, y: event.clientY });

    event.preventDefault();
  };

  const handleMouseUp = () => {
    if (isSelecting && selectionStart !== null && selectionEnd !== null) {
      finishSelection();
    }
  };



  // Render sequence bases for a line
  const renderSequenceLine = (lineIndex: number) => {
    if (!showSequence) return null;

    const lineStart = viewStart + lineIndex * BASES_PER_LINE;
    const lineEnd = Math.min(lineStart + BASES_PER_LINE, viewEnd);
    const lineBases = sequence.slice(lineStart, lineEnd);
    const selectionInfo = getSelectionInfo();

    const baseElements = [];
    for (let i = 0; i < lineBases.length; i++) {
      const base = lineBases[i].toUpperCase();
      const position = lineStart + i;
      const x = 50 + i * BASE_WIDTH;
      const y = RULER_HEIGHT + lineIndex * LINE_HEIGHT + SEQUENCE_HEIGHT;

      const color = {
        'A': '#ff6b6b',
        'T': '#4ecdc4',
        'G': '#45b7d1',
        'C': '#f9ca24'
      }[base] || '#666';

      // Check if this base is selected
      const isSelected = selectionInfo &&
        position >= selectionInfo.start &&
        position <= selectionInfo.end;

      // Debug: log selection info for troubleshooting
      if (isSelected && position === selectionInfo.start) {
        console.log('Selection debug:', {
          selectionStart: selectionInfo.start,
          selectionEnd: selectionInfo.end,
          selectionLength: selectionInfo.length,
          firstSelectedPosition: position,
          baseWidth: BASE_WIDTH,
          x: x
        });
      }

      // Add selectable area background (for debugging) - show actual clickable zone
      baseElements.push(
        <rect
          key={`clickable-zone-${lineIndex}-${i}`}
          x={x - 1}
          y={RULER_HEIGHT + lineIndex * LINE_HEIGHT + SEQUENCE_HEIGHT - 9 - 20}  // 文字上端-9px + 余白20px
          width={BASE_WIDTH + 2}
          height={12 + 40}  // フォントサイズ12px + 上下余白20pxずつ
          fill="rgba(255, 255, 0, 0.15)"  // Yellow background for actual clickable area
          stroke="rgba(255, 255, 0, 0.3)"
          strokeWidth={0.5}
          style={{ pointerEvents: 'none' }}
        />
      );

      // Add selection background
      if (isSelected) {
        baseElements.push(
          <rect
            key={`selection-${lineIndex}-${i}`}
            x={x - 1}  // Slightly extend left
            y={y - 12}  // Align with text vertical center
            width={BASE_WIDTH + 2}  // Slightly wider for better coverage
            height={14}  // Match font size
            fill="rgba(59, 130, 246, 0.8)"
            stroke="rgba(37, 99, 235, 1.0)"
            strokeWidth={1}
          />
        );
      }

      baseElements.push(
        <text
          key={`${lineIndex}-${i}`}
          x={x + BASE_WIDTH/2}
          y={y}
          textAnchor="middle"
          fontSize={12}
          fill={isSelected ? '#1e40af' : color}
          fontFamily="monospace"
          fontWeight={isSelected ? "bold" : "bold"}
          style={{ userSelect: 'none', pointerEvents: 'none' }}
        >
          {base}
        </text>
      );
    }

    return baseElements;
  };

  // Render ruler for a line
  const renderRulerLine = (lineIndex: number) => {
    const lineStart = viewStart + lineIndex * BASES_PER_LINE;
    const lineY = RULER_HEIGHT + lineIndex * LINE_HEIGHT;
    const ticks = [];

    // Major ticks every 10 bases
    for (let i = 0; i <= BASES_PER_LINE; i += 10) {
      if (lineStart + i > viewEnd) break;

      const x = 50 + i * BASE_WIDTH;
      const position = lineStart + i;

      ticks.push(
        <g key={`tick-${lineIndex}-${i}`}>
          <line
            x1={x}
            y1={lineY - 10}
            x2={x}
            y2={lineY}
            stroke="#666"
            strokeWidth={1}
          />
          {i % 50 === 0 && (
            <text
              x={x}
              y={lineY - 15}
              textAnchor="middle"
              fontSize={9}
              fill="#666"
              style={{ userSelect: 'none', pointerEvents: 'none' }}
            >
              {position}
            </text>
          )}
        </g>
      );
    }

    // Baseline
    ticks.push(
      <line
        key={`baseline-${lineIndex}`}
        x1={50}
        y1={lineY}
        x2={50 + Math.min(BASES_PER_LINE, viewEnd - lineStart) * BASE_WIDTH}
        y2={lineY}
        stroke="#ccc"
        strokeWidth={1}
      />
    );

    return ticks;
  };

  // Render feature arrow
  const renderFeatureArrow = (
    feature: GenBankFeature,
    track: FeatureTrack,
    lineIndex: number
  ) => {
    const { start, end, isReverse } = parseLocation(feature.location);


    if (end < viewStart || start > viewEnd) return null;

    const lineStart = viewStart + lineIndex * BASES_PER_LINE;
    const lineEnd = Math.min(lineStart + BASES_PER_LINE, viewEnd);

    // Check if feature should be rendered on this line
    if (end < lineStart || start > lineEnd) return null;

    const featureStart = Math.max(start, lineStart);
    const featureEnd = Math.min(end, lineEnd);

    // Only render if this feature appears on the current line
    // Feature should be rendered if it starts, ends, or spans through this line
    const featureStartLine = Math.floor((start - viewStart) / BASES_PER_LINE);
    const featureEndLine = Math.floor((end - viewStart) / BASES_PER_LINE);

    if (lineIndex < featureStartLine || lineIndex > featureEndLine) return null;

    // Calculate x coordinates directly for the current line
    const startPosInLine = featureStart - lineStart;
    const endPosInLine = featureEnd - lineStart;

    const x1 = 50 + startPosInLine * BASE_WIDTH;
    const x2 = 50 + endPosInLine * BASE_WIDTH;
    const y = RULER_HEIGHT + lineIndex * LINE_HEIGHT + SEQUENCE_HEIGHT + 30 + track.yOffset;
    const arrowSize = 8;

    // Create arrow path
    let path;
    if (isReverse) {
      // Reverse arrow (pointing left)
      path = [
        `M ${x1 + arrowSize} ${y}`,
        `L ${x2} ${y}`,
        `L ${x2} ${y - FEATURE_HEIGHT/2}`,
        `L ${x2 + arrowSize} ${y}`,
        `L ${x2} ${y + FEATURE_HEIGHT/2}`,
        `L ${x2} ${y}`,
        `L ${x1 + arrowSize} ${y}`,
        `L ${x1 + arrowSize} ${y + FEATURE_HEIGHT/2}`,
        `L ${x1} ${y}`,
        `L ${x1 + arrowSize} ${y - FEATURE_HEIGHT/2}`,
        'Z'
      ].join(' ');
    } else {
      // Forward arrow (pointing right)
      path = [
        `M ${x1} ${y - FEATURE_HEIGHT/2}`,
        `L ${x2 - arrowSize} ${y - FEATURE_HEIGHT/2}`,
        `L ${x2 - arrowSize} ${y - FEATURE_HEIGHT/2 - arrowSize/2}`,
        `L ${x2} ${y}`,
        `L ${x2 - arrowSize} ${y + FEATURE_HEIGHT/2 + arrowSize/2}`,
        `L ${x2 - arrowSize} ${y + FEATURE_HEIGHT/2}`,
        `L ${x1} ${y + FEATURE_HEIGHT/2}`,
        'Z'
      ].join(' ');
    }

    const isSelected = selectedFeature === feature;

    return (
      <g key={`${feature.feature_type}-${lineIndex}-${start}`}>
        <path
          d={path}
          fill={track.color}
          stroke={isSelected ? "#000" : track.color}
          strokeWidth={isSelected ? 2 : 1}
          opacity={0.9}
          onClick={() => setSelectedFeature(isSelected ? null : feature)}
        />

        {/* Feature label */}
        {x2 - x1 > 30 && (
          <text
            x={(x1 + x2) / 2}
            y={y + 4}
            textAnchor="middle"
            fontSize={10}
            fill="white"
            fontWeight="600"
            style={{ pointerEvents: 'none', userSelect: 'none' }}
          >
            {feature.qualifiers.gene || feature.qualifiers.label || feature.qualifiers.product || feature.feature_type}
          </text>
        )}
      </g>
    );
  };

  // Calculate total height
  const totalHeight = RULER_HEIGHT + totalLines * LINE_HEIGHT + featureTracks.length * FEATURE_SPACING + 50;

  return (
    <div className="bg-white rounded-lg border border-gray-200 p-4 relative">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">SnapGene Style Visualization</h3>
        <div className="flex items-center space-x-2 text-sm text-gray-600">
          <span>Showing: {viewStart.toLocaleString()} - {viewEnd.toLocaleString()}</span>
          <span>({sequence.length.toLocaleString()} bp total)</span>
        </div>
      </div>

      {/* Selection Tooltip - Show when has selection or is selecting */}
      {(hasSelection || isSelecting) && tooltipPosition && getSelectionInfo() && (
        <div
          className="fixed z-50 bg-gray-900 text-white px-3 py-2 rounded-lg shadow-xl text-sm pointer-events-none border border-gray-700"
          style={{
            left: tooltipPosition.x + 10,
            top: tooltipPosition.y - 60,
            transform: 'translateY(-100%)'
          }}
        >
          <div className="flex items-center space-x-3">
            <span className="text-gray-300">Start: <strong className="text-white">{(getSelectionInfo()!.start + 1).toLocaleString()}</strong></span>
            <span className="text-gray-400">|</span>
            <span className="text-gray-300">End: <strong className="text-white">{(getSelectionInfo()!.end + 1).toLocaleString()}</strong></span>
            <span className="text-gray-400">|</span>
            <span className="text-gray-300">Length: <strong className="text-green-400">{getSelectionInfo()!.length.toLocaleString()} nt</strong></span>
          </div>
        </div>
      )}

      {/* Controls */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center space-x-4">
          <div className="flex items-center space-x-2">
            <label className="text-sm">View range:</label>
            <input
              type="number"
              value={viewStart}
              onChange={(e) => setViewStart(Math.max(0, parseInt(e.target.value) || 0))}
              className="w-20 px-2 py-1 border rounded text-sm"
            />
            <span className="text-sm">to</span>
            <input
              type="number"
              value={viewEnd}
              onChange={(e) => setViewEnd(Math.min(sequence.length, parseInt(e.target.value) || viewEnd))}
              className="w-20 px-2 py-1 border rounded text-sm"
            />
          </div>
        </div>

        <div className="flex items-center space-x-4">
          <label className="flex items-center space-x-1">
            <input
              type="checkbox"
              checked={showSequence}
              onChange={(e) => setShowSequence(e.target.checked)}
            />
            <span className="text-sm">Show Sequence</span>
          </label>
          <label className="flex items-center space-x-1">
            <input
              type="checkbox"
              checked={showTranslation}
              onChange={(e) => setShowTranslation(e.target.checked)}
            />
            <span className="text-sm">Show Translation</span>
          </label>
        </div>
      </div>

      {/* SVG Visualization */}
      <div className="border border-gray-300 rounded-lg overflow-auto bg-white">
        <svg
          ref={svgRef}
          width={actualWidth}
          height={totalHeight}
          className="bg-white"
        >
          {/* Background rect for capturing all mouse events */}
          <rect
            x={0}
            y={0}
            width={actualWidth}
            height={totalHeight}
            fill="transparent"
            onMouseDown={handleMouseDown}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
          />
          {/* Content overlay */}
          <g>
            {/* Render each line */}
            {Array.from({ length: totalLines }, (_, lineIndex) => (
              <g key={`line-${lineIndex}`}>
                {/* Ruler for this line */}
                {renderRulerLine(lineIndex)}

                {/* Sequence for this line */}
                {renderSequenceLine(lineIndex)}

                {/* Features for this line */}
                {featureTracks.map((track) =>
                  track.features.map((feature) =>
                    renderFeatureArrow(feature, track, lineIndex)
                  )
                )}
              </g>
            ))}
          </g>

        </svg>
      </div>

      {/* Feature Legend */}
      {featureTracks.length > 0 && (
        <div className="mt-4">
          <h4 className="text-sm font-medium text-gray-700 mb-2">Feature Types</h4>
          <div className="grid grid-cols-4 gap-3">
            {featureTracks.map(track => (
              <div key={`${track.type}-${track.strand}`} className="flex items-center space-x-2">
                <div
                  className="w-4 h-4 rounded"
                  style={{ backgroundColor: track.color }}
                />
                <span className="text-sm text-gray-600 capitalize">
                  {track.type} {track.strand === 'reverse' ? '(−)' : '(+)'}
                  <span className="text-gray-400"> ({track.features.length})</span>
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Sequence Selection Info (detailed panel) */}
      {(() => {
        const selectionInfo = getSelectionInfo();

        if (!selectionInfo || !hasSelection) return null; // Show only when has selection

        const selectedSequence = sequence.slice(selectionInfo.start, selectionInfo.end + 1).toUpperCase();

        return (
          <div className="mt-4 p-4 bg-green-50 border border-green-200 rounded-lg">
            <h4 className="text-sm font-semibold text-green-900 mb-2">
              Selected Sequence Details
            </h4>
            <div className="grid grid-cols-3 gap-4 text-sm text-green-800 mb-3">
              <div>
                <div><strong>Start:</strong> {(selectionInfo.start + 1).toLocaleString()}</div>
              </div>
              <div>
                <div><strong>End:</strong> {(selectionInfo.end + 1).toLocaleString()}</div>
              </div>
              <div>
                <div><strong>Length:</strong> {selectionInfo.length.toLocaleString()} nt</div>
              </div>
            </div>
            <div>
              <div className="text-sm text-green-700 mb-1">
                <strong>Sequence:</strong>
              </div>
              <div className="font-mono bg-green-100 px-3 py-2 rounded break-all text-xs max-h-32 overflow-y-auto">
                {selectedSequence || 'No sequence data'}
              </div>
            </div>
          </div>
        );
      })()}

      {/* Selected Feature Info */}
      {selectedFeature && (
        <div className="mt-4 p-4 bg-blue-50 border border-blue-200 rounded-lg">
          <h4 className="text-sm font-semibold text-blue-900 mb-2">
            Selected Feature: {selectedFeature.feature_type.toUpperCase()}
          </h4>
          <div className="grid grid-cols-2 gap-4 text-sm text-blue-800">
            <div>
              <div><strong>Location:</strong> {selectedFeature.location}</div>
              {selectedFeature.qualifiers.gene && (
                <div><strong>Gene:</strong> {selectedFeature.qualifiers.gene}</div>
              )}
              {selectedFeature.qualifiers.label && (
                <div><strong>Label:</strong> {selectedFeature.qualifiers.label}</div>
              )}
              {selectedFeature.qualifiers.product && (
                <div><strong>Product:</strong> {selectedFeature.qualifiers.product}</div>
              )}
            </div>
            <div>
              {selectedFeature.qualifiers.locus_tag && (
                <div><strong>Locus Tag:</strong> {selectedFeature.qualifiers.locus_tag}</div>
              )}
              {selectedFeature.qualifiers.note && (
                <div><strong>Note:</strong> {selectedFeature.qualifiers.note}</div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};