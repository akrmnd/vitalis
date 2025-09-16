interface StatCardProps {
  label: string;
  value: string;
  color: string;
}

export const StatCard = ({ label, value, color }: StatCardProps) => {
  return (
    <div className="bg-white p-4 rounded-lg border border-gray-200">
      <div className="text-sm text-gray-600 mb-1">{label}</div>
      <div className={`text-2xl font-bold ${color}`}>{value}</div>
    </div>
  );
};