import { ReactNode } from "react";
import { Sidebar } from "./Sidebar";

interface LayoutProps {
  children: ReactNode;
  activeTab?: string;
  onTabChange?: (tab: string) => void;
  hasResults?: boolean;
  sequenceId?: string;
}

export const Layout = ({ children, activeTab = "analyze", onTabChange, hasResults, sequenceId }: LayoutProps) => {
  const handleTabChange = (tab: string) => {
    if (onTabChange) {
      onTabChange(tab);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex">
      {/* Sidebar */}
      <Sidebar activeTab={activeTab} onTabChange={handleTabChange} />

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Top bar */}
        <header className="bg-white shadow-sm border-b border-gray-200 px-6 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-semibold text-gray-900">
                {getTabTitle(activeTab)}
              </h2>
              <p className="text-sm text-gray-600">{getTabDescription(activeTab)}</p>
            </div>
            <div className="flex items-center space-x-4">
              <div className="text-sm text-gray-500">
                {sequenceId ? (
                  <div className="flex items-center space-x-2">
                    <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                    <span>Sequence loaded: {sequenceId}</span>
                  </div>
                ) : (
                  "Ready for analysis"
                )}
              </div>
              {hasResults && (
                <div className="text-xs bg-green-100 text-green-800 px-2 py-1 rounded-full">
                  Results available
                </div>
              )}
            </div>
          </div>
        </header>

        {/* Main content area */}
        <main className="flex-1 p-6 overflow-auto">
          <div className="max-w-6xl mx-auto">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
};

function getTabTitle(tab: string): string {
  const titles: Record<string, string> = {
    import: "Import Sequences",
    visualization: "Sequence View",
    "primer-design": "Primer Design",
    analysis: "Analysis Tools"
  };
  return titles[tab] || "Vitalis Studio";
}

function getTabDescription(tab: string): string {
  const descriptions: Record<string, string> = {
    import: "Import DNA/RNA sequences from FASTA, FASTQ, or GenBank formats",
    visualization: "Interactive sequence visualization with feature annotations",
    "primer-design": "Design PCR primers with Tm calculation and validation",
    analysis: "Statistical analysis and sequence processing tools"
  };
  return descriptions[tab] || "DNA/RNA Sequence Analysis Tool";
}