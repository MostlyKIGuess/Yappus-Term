import React from "react";

interface CommandBadgeProps {
  command: string;
  description: string;
  specialClass?: string;
}

export default function CommandBadge({
  command,
  description,
  specialClass = "",
}: CommandBadgeProps) {
  return (
    <div
      className={`border border-gray-700 ${specialClass} rounded px-3 py-2 bg-black/20 hover:bg-black/40 transition-colors`}
    >
      <code className="text-emerald-300 font-bold text-sm">{command}</code>
      <p className="text-xs text-gray-400 mt-1">{description}</p>
    </div>
  );
}
