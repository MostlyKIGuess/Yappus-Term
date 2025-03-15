import React from "react";
import DocExample from "../ui/DocExample";

export default function HistorySection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Chat History
      </h3>
      <p className="text-gray-300 mb-6">
        Manage your conversation history with these commands:
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        <DocExample
          title="View History"
          description="Review your past conversations"
          command="yappus history"
        />
        <DocExample
          title="Clear History"
          description="Remove all saved conversations"
          command="yappus clear-history"
        />
        <DocExample
          title="Export History"
          description="Save your history to a file"
          command="yappus export ~/my_chat_export.json"
        />
        <DocExample
          title="Interactive History"
          description="View history during a chat session"
          command="/history"
        />
      </div>
    </div>
  );
}
