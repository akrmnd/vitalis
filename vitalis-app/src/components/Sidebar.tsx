import { useState } from "react";

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

interface NavItem {
  id: string;
  label: string;
  icon: string;
  description: string;
}

const navItems: NavItem[] = [
  {
    id: "import",
    label: "Import",
    icon: "📁",
    description: "Import sequences (FASTA/FASTQ/GenBank)"
  },
  {
    id: "visualization",
    label: "Sequence View",
    icon: "🧬",
    description: "Interactive sequence visualization"
  },
  {
    id: "analysis",
    label: "Analysis",
    icon: "📊",
    description: "Statistics and analysis tools"
  }
];

export const Sidebar = ({ activeTab, onTabChange }: SidebarProps) => {
  const [isCollapsed, setIsCollapsed] = useState(false);

  return (
    <div className={`bg-gray-900 text-white flex flex-col transition-all duration-300 ${
      isCollapsed ? 'w-16' : 'w-64'
    }`}>
      {/* Header */}
      <div className="p-4 border-b border-gray-700">
        <div className="flex items-center justify-between">
          {!isCollapsed && (
            <div>
              <h1 className="text-xl font-bold">Vitalis Studio</h1>
              <p className="text-xs text-gray-400">DNA/RNA Analysis Tool</p>
            </div>
          )}
          <button
            onClick={() => setIsCollapsed(!isCollapsed)}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors"
          >
            {isCollapsed ? "→" : "←"}
          </button>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4">
        <ul className="space-y-2">
          {navItems.map((item) => (
            <li key={item.id}>
              <button
                onClick={() => onTabChange(item.id)}
                className={`w-full flex items-center gap-3 p-3 rounded-lg transition-all duration-200 ${
                  activeTab === item.id
                    ? 'bg-blue-600 text-white shadow-lg'
                    : 'hover:bg-gray-800 text-gray-300 hover:text-white'
                }`}
              >
                <span className="text-xl flex-shrink-0">{item.icon}</span>
                {!isCollapsed && (
                  <div className="text-left">
                    <div className="font-medium">{item.label}</div>
                    <div className="text-xs text-gray-400">{item.description}</div>
                  </div>
                )}
              </button>
            </li>
          ))}
        </ul>
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-gray-700">
        {!isCollapsed && (
          <div className="text-xs text-gray-400 text-center">
            Phase 1 - Basic Analysis
          </div>
        )}
      </div>
    </div>
  );
};