import React, { useState } from 'react';
import { GenBankMetadata, GenBankFeature, GenBankFeatureGroup } from '../types/genbank';

interface GenBankMetadataViewerProps {
  metadata: GenBankMetadata;
}

export const GenBankMetadataViewer: React.FC<GenBankMetadataViewerProps> = ({ metadata }) => {
  const [activeTab, setActiveTab] = useState<'info' | 'features'>('info');
  const [expandedFeature, setExpandedFeature] = useState<number | null>(null);

  // Group features by type
  const featureGroups: GenBankFeatureGroup[] = React.useMemo(() => {
    const groups = new Map<string, GenBankFeature[]>();

    metadata.features.forEach(feature => {
      if (!groups.has(feature.feature_type)) {
        groups.set(feature.feature_type, []);
      }
      groups.get(feature.feature_type)!.push(feature);
    });

    return Array.from(groups.entries()).map(([type, features]) => ({
      type,
      count: features.length,
      features
    })).sort((a, b) => b.count - a.count);
  }, [metadata.features]);

  const renderQualifierValue = (value: string) => {
    // Handle multi-line values and clean up formatting
    if (value.length > 80) {
      return (
        <div>
          <div className="break-words">{value.substring(0, 80)}...</div>
          <button
            className="text-blue-600 hover:text-blue-800 text-xs mt-1"
            onClick={() => {/* TODO: Show full value in modal */}}
          >
            Show full
          </button>
        </div>
      );
    }
    return <span className="break-words">{value}</span>;
  };

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200">
      <div className="border-b border-gray-200">
        <nav className="flex space-x-8 px-6" aria-label="Tabs">
          <button
            onClick={() => setActiveTab('info')}
            className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'info'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            📄 Sequence Info
          </button>
          <button
            onClick={() => setActiveTab('features')}
            className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'features'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            🧬 Features ({metadata.features.length})
          </button>
        </nav>
      </div>

      <div className="p-6">
        {activeTab === 'info' && (
          <div className="space-y-6">
            {/* Basic Information */}
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Basic Information</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Accession</label>
                    <p className="text-sm font-mono text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200">
                      {metadata.accession}
                    </p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Version</label>
                    <p className="text-sm font-mono text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200">
                      {metadata.version}
                    </p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Length</label>
                    <p className="text-sm text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200">
                      <span className="font-semibold">{metadata.length.toLocaleString()}</span> bp
                    </p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Topology</label>
                    <div className="bg-gray-50 px-3 py-2 rounded-md border border-gray-200">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                        metadata.topology === 'Circular'
                          ? 'bg-green-100 text-green-800'
                          : 'bg-blue-100 text-blue-800'
                      }`}>
                        {metadata.topology === 'Circular' ? '🔄' : '📏'} {metadata.topology}
                      </span>
                    </div>
                  </div>
                </div>
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Definition</label>
                    <p className="text-sm text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200 leading-relaxed">
                      {metadata.definition}
                    </p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Source</label>
                    <p className="text-sm text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200">
                      {metadata.source}
                    </p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Organism</label>
                    <p className="text-sm text-gray-900 bg-gray-50 px-3 py-2 rounded-md border border-gray-200 italic">
                      {metadata.organism}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            {/* Features Summary */}
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Features Summary</h3>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {featureGroups.slice(0, 8).map((group) => (
                  <div key={group.type} className="bg-gray-50 rounded-lg p-3 text-center">
                    <div className="text-lg font-semibold text-gray-900">{group.count}</div>
                    <div className="text-sm text-gray-600 capitalize">{group.type}</div>
                  </div>
                ))}
              </div>
              {featureGroups.length > 8 && (
                <p className="mt-2 text-sm text-gray-500">
                  +{featureGroups.length - 8} more feature types...
                </p>
              )}
            </div>
          </div>
        )}

        {activeTab === 'features' && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold text-gray-900">
                Annotations ({metadata.features.length} features)
              </h3>
              <div className="text-sm text-gray-500">
                {featureGroups.length} different types
              </div>
            </div>

            <div className="space-y-4">
              {featureGroups.map((group, groupIndex) => (
                <div key={group.type} className="border border-gray-200 rounded-lg">
                  <div className="bg-gray-50 px-4 py-3 border-b border-gray-200">
                    <div className="flex items-center justify-between">
                      <h4 className="font-medium text-gray-900 capitalize">
                        {group.type} <span className="text-gray-500">({group.count})</span>
                      </h4>
                    </div>
                  </div>
                  <div className="divide-y divide-gray-200">
                    {group.features.slice(0, 5).map((feature, featureIndex) => {
                      const globalIndex = groupIndex * 1000 + featureIndex; // Simple unique ID
                      const isExpanded = expandedFeature === globalIndex;

                      return (
                        <div key={featureIndex} className="p-4 hover:bg-gray-25 border-b border-gray-100 last:border-b-0">
                          <div className="space-y-3">
                            <div className="flex items-start justify-between">
                              <div className="flex-1 min-w-0">
                                <div className="flex items-center flex-wrap gap-2 mb-3">
                                  <span className="font-mono text-sm text-blue-700 bg-blue-100 px-3 py-1 rounded-md font-medium">
                                    {feature.location}
                                  </span>
                                  {Object.keys(feature.qualifiers).length > 0 && (
                                    <button
                                      onClick={() => setExpandedFeature(isExpanded ? null : globalIndex)}
                                      className="text-xs text-gray-600 hover:text-gray-800 hover:bg-gray-100 px-2 py-1 rounded flex items-center space-x-1 transition-colors"
                                    >
                                      <span>{isExpanded ? '▼' : '▶'}</span>
                                      <span>{Object.keys(feature.qualifiers).length} qualifiers</span>
                                    </button>
                                  )}
                                </div>

                                <div className="space-y-2">
                                  {feature.qualifiers.gene && (
                                    <div className="text-sm text-gray-700">
                                      <span className="inline-block bg-green-100 text-green-800 px-2 py-1 rounded text-xs font-medium mr-2">
                                        Gene
                                      </span>
                                      <span className="font-medium">{feature.qualifiers.gene}</span>
                                    </div>
                                  )}
                                  {feature.qualifiers.product && (
                                    <div className="text-sm text-gray-700">
                                      <span className="inline-block bg-purple-100 text-purple-800 px-2 py-1 rounded text-xs font-medium mr-2">
                                        Product
                                      </span>
                                      <span>{feature.qualifiers.product}</span>
                                    </div>
                                  )}
                                </div>
                              </div>
                            </div>
                          </div>

                          {isExpanded && Object.keys(feature.qualifiers).length > 0 && (
                            <div className="mt-4 bg-gray-50 rounded-lg p-3">
                              <h5 className="text-xs font-semibold text-gray-700 mb-3">Qualifiers</h5>
                              <div className="space-y-2">
                                {Object.entries(feature.qualifiers).map(([key, value]) => (
                                  <div key={key} className="bg-white rounded border border-gray-200 p-2">
                                    <div className="text-xs font-medium text-gray-600 mb-1 capitalize">
                                      {key}
                                    </div>
                                    <div className="text-xs text-gray-800 leading-relaxed break-words">
                                      {renderQualifierValue(value)}
                                    </div>
                                  </div>
                                ))}
                              </div>
                            </div>
                          )}
                        </div>
                      );
                    })}
                    {group.features.length > 5 && (
                      <div className="p-4 text-center text-sm text-gray-500">
                        +{group.features.length - 5} more {group.type} features...
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};