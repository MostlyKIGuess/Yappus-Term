import React, { useState } from 'react';
import { FiClipboard, FiCheck } from 'react-icons/fi';
import { motion } from 'framer-motion';

interface DocExampleProps {
  title: string;
  description: string;
  command: string;
}

export default function DocExample({ title, description, command }: DocExampleProps) {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <motion.div 
      className="bg-neutral-800/70 rounded-xl border border-neutral-700/80 shadow-lg overflow-hidden backdrop-blur-sm"
      initial={{ opacity: 0, y: 15 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, amount: 0.3 }}
      transition={{ duration: 0.5, ease: "easeOut" }}
    >
      <div className="p-5">
        <h4 className="text-lg font-semibold text-slate-100 mb-1.5">{title}</h4>
        <p className="text-slate-400 text-sm mb-4">{description}</p>
      </div>
      <div className="bg-neutral-900/80 p-4 font-mono text-sm flex justify-between items-center group border-t border-neutral-700/80">
        <code className="text-sky-400 break-all select-all">{command}</code>
        <motion.button
          onClick={copyToClipboard}
          className="p-2 rounded-md bg-neutral-700/50 hover:bg-neutral-600/70 text-slate-400 hover:text-slate-200 transition-all duration-200 opacity-60 group-hover:opacity-100"
          aria-label="Copy to clipboard"
          whileTap={{ scale: 0.9 }}
        >
          {copied ? <FiCheck className="w-4 h-4 text-green-400" /> : <FiClipboard className="w-4 h-4" />}
        </motion.button>
      </div>
    </motion.div>
  );
}
