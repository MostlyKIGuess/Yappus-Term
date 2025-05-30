import React from 'react';
import { FaGithub, FaHeart } from 'react-icons/fa';

export default function Footer() {
  return (
    <footer className="bg-neutral-900 border-t border-neutral-800/70 text-slate-400 py-12">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <div className="flex justify-center items-center space-x-6 mb-6">
          <a 
            href="https://github.com/MostlyKIGuess/Yappus-Term" 
            target="_blank" 
            rel="noopener noreferrer"
            className="text-slate-500 hover:text-slate-300 transition-colors duration-200"
            aria-label="GitHub Repository"
          >
            <FaGithub size={24} />
          </a>
        </div>
        <p className="text-sm mb-2">
          Yappus Terminal &copy; {new Date().getFullYear()}
        </p>
        <p className="text-xs">
          Made with <FaHeart className="inline text-red-500/70 mx-0.5" /> by MostlyKIGuess
        </p>
      </div>
    </footer>
  );
}
