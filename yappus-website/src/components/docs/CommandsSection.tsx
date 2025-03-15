import React from "react";
import CommandBadge from "../ui/CommandBadge";

export default function CommandsSection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Interactive Commands
      </h3>
      <p className="text-gray-300 mb-6">
        Use these commands during an interactive session:
      </p>

      <div className="grid md:grid-cols-3 gap-4">
        <CommandBadge command="/help" description="Show help menu" />
        <CommandBadge command="/model" description="View/change model" />
        <CommandBadge command="/history" description="View chat history" />
        <CommandBadge command="/clearhistory" description="Clear history" />
        <CommandBadge command="/version" description="Show version info" />
        <CommandBadge command="/config" description="Display config" />
        <CommandBadge command="/key reset" description="Reset API key" />
        <CommandBadge command="/export" description="Export history" />
        <CommandBadge command="/clear" description="Clear terminal" />
        <CommandBadge
          command="exit"
          description="Quit application"
          specialClass="border-red-500"
        />
      </div>
    </div>
  );
}
