import React from 'react';
import { FiMessageCircle, FiFileText, FiGitBranch } from 'react-icons/fi';
import { motion } from 'framer-motion';

const contextFeatures = [
  {
    icon: <FiMessageCircle className="w-6 h-6 text-purple-400" />,
    title: "Conversation Flow",
    description: "Yappus remembers previous parts of your current conversation, allowing for natural follow-up questions and more relevant AI responses without needing to repeat context."
  },
  {
    icon: <FiFileText className="w-6 h-6 text-orange-400" />,
    title: "File Content Integration",
    description: "Use the `/file <path>` command to load a file's content directly into the conversation context. The AI can then answer questions or perform tasks based on that specific file."
  },
  {
    icon: <FiGitBranch className="w-6 h-6 text-teal-400" />,
    title: "Directory & Git Awareness",
    description: "Yappus automatically includes information about your current working directory and (if applicable) your current Git branch and repository status as part of the context provided to the AI."
  }
];

export default function ContextAwarenessSection() {
  return (
    <div>
      <h3 className="text-2xl font-semibold text-slate-100 mb-3 text-center sm:text-left">Context Awareness</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">
        Yappus understands the context of your work for smarter interactions.
      </p>
      <div className="grid md:grid-cols-3 gap-6 lg:gap-8">
        {contextFeatures.map((feature, index) => (
          <motion.div
            key={feature.title}
            className="bg-neutral-800/70 rounded-xl border border-neutral-700/80 p-6 shadow-lg backdrop-blur-sm flex flex-col items-start"
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true, amount: 0.3 }}
            transition={{ duration: 0.5, delay: index * 0.1, ease: "easeOut" }}
          >
            <div className="p-2.5 rounded-lg bg-neutral-700/60 mb-4">
              {feature.icon}
            </div>
            <h4 className="text-lg font-semibold text-slate-100 mb-2">{feature.title}</h4>
            <p className="text-sm text-slate-400 flex-grow">{feature.description}</p>
          </motion.div>
        ))}
      </div>
    </div>
  );
}
