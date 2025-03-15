import React, { useState } from "react";

interface DocExampleProps {
  title: string;
  description: string;
  command: string;
}

export default function DocExample({ title, description, command }: DocExampleProps) {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="bg-gray-900 rounded-lg border border-gray-700 overflow-hidden">
      <div className="p-4">
        <h4 className="text-lg font-medium text-white mb-1">{title}</h4>
        <p className="text-gray-400 text-sm">{description}</p>
      </div>
      <div className="bg-black/30 p-4 font-mono text-sm flex justify-between items-center group">
        <code className="text-emerald-300 break-all">{command}</code>
        <button
          onClick={copyToClipboard}
          className="opacity-0 group-hover:opacity-100 bg-gray-800 hover:bg-gray-700 p-1 rounded transition-all"
          aria-label="Copy to clipboard"
        >
          {copied ? "✓" : "📋"}
        </button>
      </div>
    </div>
  );
}
