import React from "react";

interface ModelBadgeProps {
  name: string;
  description: string;
  colorClass?: string;
}

export default function ModelBadge({
  name,
  description,
  colorClass = "from-green-500 to-emerald-600",
}: ModelBadgeProps) {
  return (
    <div className="flex flex-col">
      <div
        className={`text-xs font-semibold bg-gradient-to-r ${colorClass} rounded px-2 py-1 text-white inline-block w-fit`}
      >
        {name}
      </div>
      <span className="text-xs text-gray-400 mt-1">{description}</span>
    </div>
  );
}
