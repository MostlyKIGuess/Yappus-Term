import React from 'react';
import SectionContainer from '../SectionContainer';
import CommandsSection from './CommandsSection';
import ExamplesSection from './ExamplesSection';
import HistorySection from './HistorySection';
import ContextAwarenessSection from './ContextAwarenessSection'; 
import ModelsSection from './ModelSection';

export default function DocumentationSection() {
  return (
    <SectionContainer id="documentation" title="Documentation" subtitle="Learn how to use Yappus Terminal effectively">
      <div className="space-y-16 md:space-y-20">
        <CommandsSection />
        <ExamplesSection />
        <ModelsSection />
        <HistorySection />
        <ContextAwarenessSection />
      </div>
    </SectionContainer>
  );
}
