import React from 'react';

interface ModelBadgeProps {
  name: string;
  description: string;
  isCurrent?: boolean;
}

export default function ModelBadge({ name, description, isCurrent }: ModelBadgeProps) {
  return (
    <div className={`p-4 rounded-lg border transition-all duration-200 ease-out
                    ${isCurrent ? 'bg-slate-700/60 border-slate-600 shadow-md' : 'bg-neutral-800/70 border-neutral-700 hover:border-neutral-600'}`}>
      <div className="flex items-center justify-between mb-1">
        <span className={`text-sm font-semibold ${isCurrent ? 'text-sky-300' : 'text-slate-200'}`}>
          {name}
        </span>
        {isCurrent && (
          <span className="text-xs bg-sky-500/30 text-sky-300 px-2 py-0.5 rounded-full font-medium">
            Current
          </span>
        )}
      </div>
      <p className="text-xs text-slate-400">{description}</p>
    </div>
  );
}
