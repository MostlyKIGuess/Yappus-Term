import React from "react";
import { motion } from "framer-motion";
import { FaGithub, FaBook } from "react-icons/fa";
import { FiCpu, FiDatabase } from "react-icons/fi";

export default function HeroSection() {
  return (
    <section className="px-4 pt-32 pb-20 md:pt-40 md:pb-28 max-w-7xl mx-auto">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, ease: "easeOut" }}
        className="text-center"
      >
        <h1 className="text-5xl md:text-7xl font-bold mb-6 bg-clip-text text-transparent bg-gradient-to-r from-slate-200 to-slate-400">
          Yappus Terminal
        </h1>
        <p className="text-xl md:text-2xl text-slate-400 max-w-3xl mx-auto mb-12">
          A modern terminal interface for your AI assistant, bringing
          intelligence to your command line.
        </p>

        <div className="flex flex-wrap justify-center gap-4 mb-12">
          <motion.div
            className="bg-neutral-800/70 border border-neutral-700 text-slate-300 rounded-full px-5 py-2.5 flex items-center gap-2.5 shadow-sm"
            whileHover={{ scale: 1.03, borderColor: 'rgb(100 116 139)', backgroundColor: 'rgba(51, 65, 85, 0.5)' }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2, duration: 0.5, ease: "easeOut" }}
          >
            <FiCpu className="text-slate-500 w-5 h-5" />
            <span className="font-medium text-sm">Fully Local Mode Coming Soon</span>
          </motion.div>

          <motion.div
            className="bg-neutral-800/70 border border-neutral-700 text-slate-300 rounded-full px-5 py-2.5 flex items-center gap-2.5 shadow-sm"
            whileHover={{ scale: 1.03, borderColor: 'rgb(100 116 139)', backgroundColor: 'rgba(51, 65, 85, 0.5)' }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.35, duration: 0.5, ease: "easeOut" }}
          >
            <FiDatabase className="text-slate-500 w-5 h-5" />
            <span className="font-medium text-sm">RAG Support</span>
            <span className="inline-flex items-center justify-center bg-slate-700 text-slate-200 text-xs font-semibold px-2 py-0.5 rounded-full ml-1">
              SOON
            </span>
          </motion.div>
        </div>

        <div className="flex flex-wrap justify-center gap-x-4 gap-y-3">
          <motion.a
            whileHover={{
              scale: 1.03,
              backgroundColor: 'rgb(51 65 85)', 
              boxShadow: "0px 5px 15px rgba(51, 65, 85, 0.4)"
            }}
            whileTap={{ scale: 0.97 }}
            href="https://github.com/MostlyKIGuess/Yappus-Term"
            className="px-8 py-3 bg-slate-700 border border-slate-600 text-slate-100 rounded-lg font-semibold group transition-all duration-300 ease-out flex items-center gap-2.5 shadow-md"
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.5, duration: 0.4, ease: "easeOut" }}
          >
            <FaGithub className="w-5 h-5" />
            GitHub Repository
          </motion.a>

          <motion.a
            whileHover={{ 
              scale: 1.03, 
              backgroundColor: 'rgb(30 41 59)', 
              borderColor: 'rgb(71 85 105)',
              boxShadow: "0px 5px 15px rgba(30, 41, 59, 0.3)"
            }}
            whileTap={{ scale: 0.97 }}
            href="#documentation"
            className="px-8 py-3 bg-neutral-800 border border-neutral-700 text-slate-300 rounded-lg font-semibold transition-all duration-300 ease-out hover:text-slate-100 flex items-center gap-2.5 shadow-md"
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.6, duration: 0.4, ease: "easeOut" }}
          >
            <FaBook className="w-5 h-5" />
            Documentation
          </motion.a>
        </div>
      </motion.div>
    </section>
  );
}