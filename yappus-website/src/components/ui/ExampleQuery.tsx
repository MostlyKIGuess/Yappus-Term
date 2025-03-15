import React from "react";

interface ExampleQueryProps {
  query: string;
  response: string;
}

export default function ExampleQuery({ query, response }: ExampleQueryProps) {
  return (
    <div className="rounded-lg overflow-hidden bg-black/20 border border-gray-700">
      <div className="bg-gray-800/80 p-4">
        <div className="flex items-start gap-3">
          <div className="w-6 h-6 bg-gray-700 rounded-full flex items-center justify-center mt-0.5">
            <span className="text-sm">👤</span>
          </div>
          <p className="text-gray-200">{query}</p>
        </div>
      </div>
      <div className="p-4">
        <div className="flex items-start gap-3">
          <div className="w-6 h-6 bg-emerald-900/50 rounded-full flex items-center justify-center mt-0.5">
            <span className="text-sm">🤖</span>
          </div>
          <pre className="text-emerald-300 whitespace-pre-wrap font-mono text-sm">
            {response}
          </pre>
        </div>
      </div>
    </div>
  );
}
