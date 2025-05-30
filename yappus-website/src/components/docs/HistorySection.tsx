import React from 'react';
import { FiArchive, FiTrash2, FiUploadCloud } from 'react-icons/fi';
import { motion } from 'framer-motion';

const historyFeatures = [
  {
    icon: <FiArchive className="w-6 h-6 text-sky-400" />,
    title: "Persistent Chat History",
    description: "Yappus automatically saves your conversations, allowing you to pick up where you left off. History is stored locally in `~/.config/yappus-term/chat_history.json`."
  },
  {
    icon: <FiUploadCloud className="w-6 h-6 text-green-400" />,
    title: "Export Your Chats",
    description: "Easily export your entire chat history to a JSON file for backup or analysis using the `yappus export [path]` command or `/export [path]` in interactive mode."
  },
  {
    icon: <FiTrash2 className="w-6 h-6 text-red-400" />,
    title: "Manage History",
    description: "View your history with `yappus history` or `/history`, and clear it completely with `yappus clear-history` or `/clearhistory` if needed."
  }
];

export default function HistorySection() {
  return (
    <div>
      <h3 className="text-2xl font-semibold text-slate-100 mb-3 text-center sm:text-left">Chat History Management</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">
        Keep track of your conversations and manage your data with ease.
      </p>
      <div className="grid md:grid-cols-3 gap-6 lg:gap-8">
        {historyFeatures.map((feature, index) => (
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
