import React from 'react';
import { motion } from 'framer-motion';

const terminalConversation = [
  { type: 'prompt', text: 'yappus' },
  { type: 'ai', text: "Welcome to Yappus Terminal! Type 'exit' to quit or '/help' for commands." },
  { type: 'prompt', text: 'How do I find large files in Linux?' },
  { type: 'ai', text: "To find large files in Linux, you can use the 'find' command:" },
  { type: 'code', text: 'find / -type f -size +100M -print0 | xargs -0 du -h | sort -rh' },
  { type: 'prompt', text: '/file package.json' },
  { type: 'ai', text: 'File content from package.json added to context.' },
  { type: 'prompt', text: 'What are the main dependencies?' },
  { type: 'ai', text: 'Based on package.json, the main dependencies are: react, next, tailwindcss.' },
  { type: 'cursor', text: ''}
];

export default function TerminalPreview() {
  return (
    <section className="py-16 md:py-24 px-4">
      <motion.div 
        className="max-w-3xl mx-auto bg-neutral-800/60 rounded-xl border border-neutral-700/80 shadow-2xl overflow-hidden backdrop-blur-sm"
        initial={{ opacity: 0, y: 20 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true, amount: 0.3 }}
        transition={{ duration: 0.7, ease: "easeOut" }}
      >
        <div className="bg-neutral-700/50 px-4 py-2.5 flex items-center border-b border-neutral-700/80">
          <div className="flex space-x-1.5">
            <div className="w-3 h-3 bg-red-500/70 rounded-full opacity-80"></div>
            <div className="w-3 h-3 bg-yellow-500/70 rounded-full opacity-80"></div>
            <div className="w-3 h-3 bg-green-500/70 rounded-full opacity-80"></div>
          </div>
          <p className="ml-auto text-neutral-400 text-xs font-medium">yappus@terminal</p>
        </div>
        <div className="p-5 sm:p-6 font-mono text-sm leading-relaxed h-80 overflow-y-auto">
          {terminalConversation.map((line, index) => (
            <div key={index} className={`mt-${index === 0 ? '0' : '2'}`}>
              {line.type === 'prompt' && (
                <span className="text-slate-500 mr-1.5 select-none">$</span>
              )}
              {line.type === 'cursor' ? (
                 <span className="text-slate-500 mr-1.5 select-none">$ <span className="bg-slate-200 w-2 h-4 inline-block animate-pulse align-middle"></span></span>
              ) : (
                <span className={
                  line.type === 'ai' ? 'text-slate-300' :
                  line.type === 'code' ? 'text-sky-400' :
                  'text-slate-200' 
                }>
                  {line.text}
                </span>
              )}
            </div>
          ))}
        </div>
      </motion.div>
    </section>
  );
}
