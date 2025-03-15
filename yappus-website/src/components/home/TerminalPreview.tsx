import React from "react";

export default function TerminalPreview() {
  return (
    <section className="px-4 py-2 max-w-5xl mx-auto">
      <div className="bg-gray-900 rounded-lg border border-gray-700 shadow-xl overflow-hidden">
        <div className="bg-gray-800 px-4 py-2 flex items-center">
          <div className="flex space-x-2">
            <div className="w-3 h-3 bg-red-500 rounded-full"></div>
            <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
          </div>
          <p className="ml-4 text-gray-400 text-sm">yappus-terminal</p>
        </div>
        <div className="p-4 font-mono text-sm">
          <p className="text-gray-400">$ yappus</p>
          <p className="text-green-500 mt-2">
            Welcome to Yappus Terminal! Type &apos;exit&apos; to quit or
            &apos;/help&apos; for commands.
          </p>
          <p className="mt-4 text-gray-400">
            &gt; How do I find large files in Linux?
          </p>
          <p className="mt-2 text-blue-400">
            To find large files in Linux, you can use the &apos;find&apos;
            command with the &apos;-size&apos; parameter:
          </p>
          <p className="mt-1 text-yellow-400">
            find / -type f -size +100M -exec ls -lh {} \; | sort -rh
          </p>
          <p className="text-gray-400 mt-4">&gt; _</p>
        </div>
      </div>
    </section>
  );
}
