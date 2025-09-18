export interface GenBankMetadata {
  accession: string;
  version: string;
  definition: string;
  source: string;
  organism: string;
  length: number;
  topology: "Linear" | "Circular";
  features: GenBankFeature[];
}

export interface GenBankFeature {
  feature_type: string;
  location: string;
  qualifiers: Record<string, string>;
}

export interface GenBankFeatureGroup {
  type: string;
  count: number;
  features: GenBankFeature[];
}