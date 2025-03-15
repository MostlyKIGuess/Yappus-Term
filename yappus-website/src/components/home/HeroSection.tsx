import React from "react";
import { motion } from "framer-motion";
import { FaGithub, FaBook, } from "react-icons/fa";
import { FiCpu, FiDatabase } from "react-icons/fi";

export default function HeroSection() {
  return (
    <section className="px-4 pt-32 pb-20 md:pt-40 md:pb-28 max-w-7xl mx-auto">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
        className="text-center"
      >
        <h1 className="text-5xl md:text-7xl font-bold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-emerald-400 to-blue-500">
          Yappus Terminal
        </h1>
        <p className="text-xl md:text-2xl text-gray-300 max-w-3xl mx-auto mb-8">
          A modern terminal interface for your AI assistant, bringing
          intelligence to your command line.
        </p>

        <div className="flex flex-wrap justify-center gap-3 mb-10">
          <motion.div
            className="bg-blue-900/30 border border-blue-700/50 text-blue-400 rounded-full px-4 py-2 flex items-center gap-2"
            whileHover={{ scale: 1.05 }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3, duration: 0.5 }}
          >
            <FiCpu className="text-blue-300" />
            <span className="font-medium">Fully Local Mode Coming Soon</span>
          </motion.div>

          <motion.div
            className="bg-purple-900/30 border border-purple-700/50 text-purple-400 rounded-full px-4 py-2 flex items-center gap-2"
            whileHover={{ scale: 1.05 }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5, duration: 0.5 }}
          >
            <FiDatabase className="text-purple-300" />
            <span className="font-medium">RAG Support Coming Soon</span>
            <span className="inline-flex items-center justify-center bg-purple-700/50 text-purple-200 text-xs px-2 py-0.5 rounded-full">
              SOON
            </span>
          </motion.div>
        </div>

        <div className="flex flex-wrap justify-center gap-4">
          <motion.a
            whileHover={{
              scale: 1.05,
              boxShadow: "0 10px 15px -3px rgba(4, 120, 87, 0.3)",
            }}
            whileTap={{ scale: 0.95 }}
            href="https://github.com/MostlyKIGuess/Yappus-Term"
            className="relative px-8 py-3 bg-gradient-to-br from-emerald-500 to-emerald-700 overflow-hidden rounded-lg font-medium group"
          >
            <span className="relative z-10 flex items-center gap-2">
              <FaGithub className="w-5 h-5" />
              GitHub Repository
            </span>
            <span className="absolute inset-0 bg-gradient-to-br from-emerald-600 to-emerald-800 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
          </motion.a>

          <motion.a
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            href="#docs"
            className="relative px-8 py-3 bg-gray-800 border border-gray-700 overflow-hidden rounded-lg font-medium transition-colors duration-300 hover:bg-gray-700 hover:border-gray-600 flex items-center gap-2"
          >
            <FaBook className="w-5 h-5" />
            Documentation
          </motion.a>
        </div>
      </motion.div>
    </section>
  );
}
