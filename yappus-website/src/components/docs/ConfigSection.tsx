import React from "react";
import DocExample from "../ui/DocExample";

export default function ConfigSection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Configuration
      </h3>
      <p className="text-gray-300 mb-6">
        Manage your Yappus setup with these commands:
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        <DocExample
          title="Setup Wizard"
          description="Run the initial configuration process"
          command="yappus setup"
        />
        <DocExample
          title="Check Config"
          description="View your current configuration"
          command="yappus config"
        />
        <DocExample
          title="Check API Key"
          description="Verify your API key status"
          command="yappus key"
        />
        <DocExample
          title="Reset API Key"
          description="Change your Gemini API key"
          command="yappus key --reset"
        />
      </div>
    </div>
  );
}
