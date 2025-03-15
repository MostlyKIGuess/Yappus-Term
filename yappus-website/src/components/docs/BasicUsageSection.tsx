import React from "react";
import DocExample from "../ui/DocExample";

export default function BasicUsageSection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Basic Usage
      </h3>
      <p className="text-gray-300 mb-6">
        Start using Yappus with these simple commands:
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        <DocExample
          title="Direct Query"
          description="Ask questions directly from your command line"
          command='yappus "How do I find large files in Linux?"'
        />
        <DocExample
          title="Interactive Mode"
          description="Start a chat session with the AI"
          command="yappus"
        />
      </div>
    </div>
  );
}
