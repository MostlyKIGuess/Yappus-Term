import React from "react";
import BasicUsageSection from "./BasicUsageSection";
import ModelSection from "./ModelSection";
import HistorySection from "./HistorySection";
import ConfigSection from "./ConfigSection";
import CommandsSection from "./CommandsSection";
import ExamplesSection from "./ExamplesSection";
import ContextAwarenessSection from "./ContextAwarenessSection";
export default function DocumentationSection() {
  return (
    <section id="docs" className="px-4 py-20 max-w-7xl mx-auto">
      <h2 className="text-4xl font-bold text-center mb-16">
        Documentation
      </h2>

      <div className="space-y-12">
        <BasicUsageSection />
        <ModelSection />
        <HistorySection />
        <ConfigSection />
        <CommandsSection />
        <ContextAwarenessSection/>
        <ExamplesSection />
      </div>
    </section>
  );
}
