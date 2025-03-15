import React from 'react';
import { FaGithub, FaStar, FaCode, FaHeart } from 'react-icons/fa';
import { motion } from 'framer-motion';

export default function Footer() {
  return (
    <footer className="bg-gray-900 py-12 border-t border-gray-800">
      <div className="max-w-7xl mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <div className="mb-6 md:mb-0">
            <div className="flex items-center space-x-2">
              <span className="text-emerald-400 font-bold text-xl">Yappus</span>
              <span className="text-white font-light text-xl">Terminal</span>
            </div>
            <p className="text-gray-400 mt-2">A terminal interface for your AI assistant</p>
          </div>
          
          <div className="flex flex-col items-center mb-8 md:mb-0">
            <motion.a 
              href="https://github.com/MostlyKIGuess/Yappus-Term/stargazers"
              className="flex items-center space-x-2 bg-gray-800 hover:bg-gray-700 text-amber-400 py-2 px-4 rounded-lg mb-3 transition-colors duration-300"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              <FaStar className="animate-pulse" />
              <span>Star this project on GitHub</span>
            </motion.a>
            <p className="text-gray-500 text-sm text-center">you like it? awww</p>
          </div>
          
          <div className="flex flex-col md:flex-row md:items-center space-y-4 md:space-y-0 md:space-x-8">
            <div className="flex space-x-4 items-center">
              <a 
                href="https://github.com/MostlyKIGuess" 
                className="text-gray-400 hover:text-white transition-colors duration-300 flex items-center space-x-2"
                title="Visit MostlyK's GitHub"
              >
                <FaGithub size={20} />
                <span>MostlyK</span>
              </a>
              <a 
                href="https://github.com/MostlyKIGuess/Yappus-Term" 
                className="text-gray-400 hover:text-white transition-colors duration-300 flex items-center space-x-2"
                title="View project repository"
              >
                <FaCode size={18} />
                <span>Repository</span>
              </a>
            </div>
            
            <div className="text-gray-400 text-sm flex items-center">
              <span className="mr-2">© {new Date().getFullYear()}</span>
              <span className="mr-1">Made with</span>
              <FaHeart className="text-red-500 mx-1" size={14} />
              <span>by MostlyK</span>
            </div>
          </div>
        </div>
        
        <div className="mt-8 pt-6 border-t border-gray-800 text-center text-gray-500 text-sm">
          <p>MIT License. Feel free to use and modify.</p>
        </div>
      </div>
    </footer>
  );
}
