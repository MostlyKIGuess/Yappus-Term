// components/Footer.tsx
import React from 'react';
import { FaGithub } from 'react-icons/fa';

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
          
          <div className="flex flex-col md:flex-row md:items-center space-y-4 md:space-y-0 md:space-x-8">
            <div className="flex space-x-4">
              <a href="https://github.com/MostlyKIGuess/Yappus-Term" className="text-gray-400 hover:text-white">
                <FaGithub size={24} />
              </a>
            </div>
            
            <div className="text-gray-400 text-sm">
              © {new Date().getFullYear()} MostlyK. MIT License.
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}
