import React from "react";
import { FaTerminal, FaHistory, FaCode } from "react-icons/fa";
import FeatureCard from "../ui/FeatureCard";

export default function FeaturesSection() {
  return (
    <section id="features" className="px-4 py-16 max-w-7xl mx-auto">
      <h2 className="text-4xl font-bold text-center mb-16">Key Features</h2>

      <div className="grid md:grid-cols-3 gap-8">
        <FeatureCard
          icon={<FaTerminal className="text-emerald-400" size={24} />}
          title="AI-Powered Terminal"
          description="Interact with Google Gemini AI models directly from your command line."
        />
        <FeatureCard
          icon={<FaHistory className="text-blue-400" size={24} />}
          title="Persistent History"
          description="Keep track of your conversations with chat history that persists across sessions."
        />
        <FeatureCard
          icon={<FaCode className="text-purple-400" size={24} />}
          title="Syntax Highlighting"
          description="Clear syntax highlighting for code blocks in your AI responses."
        />
      </div>
    </section>
  );
}
