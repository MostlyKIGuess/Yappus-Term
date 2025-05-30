import React from 'react';
import { motion } from 'framer-motion';
import SectionTitle from '../SectionTitle';
import { FiTerminal, FiZap, FiMessageSquare, FiBox, FiCpu, FiFileText, FiGitBranch, FiSettings } from 'react-icons/fi';

const features = [
  { title: "Interactive CLI", description: "Chat with Gemini AI directly in your terminal.", icon: <FiTerminal /> },
  { title: "Multiple Models", description: "Supports various Gemini models, including Gemini 2.0 Flash.", icon: <FiBox /> },
  { title: "Persistent History", description: "Chat history is saved across sessions.", icon: <FiMessageSquare /> },
  { title: "File Integration", description: "Use /file for context-aware discussions about your code.", icon: <FiFileText /> },
  { title: "Context Awareness", description: "Maintains conversation flow and uses directory/git context.", icon: <FiGitBranch /> },
  { title: "Command Piping", description: "Combine shell commands with AI queries for powerful workflows.", icon: <FiZap /> },
  { title: "Configurable", description: "Manage API keys and model preferences easily.", icon: <FiSettings /> },
  { title: "Local Mode (Soon)", description: "Upcoming support for fully local AI via Ollama.", icon: <FiCpu /> },
];

export default function FeaturesSection() {
  return (
    <section id="features" className="py-16 md:py-24">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <SectionTitle title="Features" subtitle="Powerful capabilities at your fingertips" />

        <div className="mt-12 grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 lg:gap-8">
          {features.map((feature, index) => (
            <motion.div
              key={feature.title}
              className="bg-neutral-800/60 rounded-xl border border-neutral-700/70 p-6 shadow-lg backdrop-blur-sm flex flex-col items-start hover:border-neutral-600/90 transition-colors duration-200"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true, amount: 0.2 }}
              transition={{ duration: 0.5, delay: index * 0.05, ease: "easeOut" }}
            >
              <div className="p-3 rounded-lg bg-slate-700/50 text-slate-300 mb-4">
                {React.cloneElement(feature.icon, { className: 'w-6 h-6' })}
              </div>
              <h3 className="text-lg font-semibold text-slate-100 mb-2">{feature.title}</h3>
              <p className="text-sm text-slate-400 flex-grow">{feature.description}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
