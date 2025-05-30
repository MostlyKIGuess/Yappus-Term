import React, { useState } from 'react';
import { motion } from 'framer-motion';
import SectionTitle from '../SectionTitle';
import { FaLinux, FaWindows, FaRust } from 'react-icons/fa';
import { FiClipboard, FiCheck } from 'react-icons/fi';

const installStepsData = [
  {
    os: 'Debian/Ubuntu',
    icon: <FaLinux className="w-7 h-7 text-orange-400" />,
    command: `curl -O https://raw.githubusercontent.com/MostlyKIGuess/Yappus-Term/main/install-yappus.sh
chmod +x install-yappus.sh
./install-yappus.sh`,
    bgClass: 'from-orange-500/10 to-neutral-800/0',
  },
  {
    os: 'Arch Linux',
    icon: <FaLinux className="w-7 h-7 text-sky-400" />,
    command: `# Install from AUR (e.g., with yay)
yay -S yappus

# Or build manually
git clone https://github.com/MostlyKIGuess/Yappus-Term.git
cd Yappus-Term
makepkg -si`,
    bgClass: 'from-sky-500/10 to-neutral-800/0',
  },
  {
    os: 'Windows',
    icon: <FaWindows className="w-7 h-7 text-blue-400" />,
    command: `# Download script using PowerShell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/MostlyKIGuess/Yappus-Term/main/install-yappus.ps1 -OutFile install-yappus.ps1

# Run installer (Admin privileges may be required for system-wide install)
powershell -ExecutionPolicy Bypass -File install-yappus.ps1`,
    bgClass: 'from-blue-500/10 to-neutral-800/0',
  },
  {
    os: 'Crates.io (Cargo)',
    icon: <FaRust className="w-7 h-7 text-red-400" />, // Using FaRust, FaDownload could also work
    command: `# Ensure you have Rust and Cargo installed
# https://www.rust-lang.org/tools/install

cargo install yappus-term`,
    bgClass: 'from-red-500/10 to-neutral-800/0',
  },
];

const CopyToClipboardButton = ({ textToCopy }: { textToCopy: string }) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(textToCopy);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <button
      onClick={handleCopy}
      className="absolute top-2 right-2 p-1.5 bg-neutral-700/50 hover:bg-neutral-600/70 rounded-md text-slate-400 hover:text-slate-200 transition-all duration-200 opacity-50 group-hover:opacity-100"
      aria-label="Copy to clipboard"
    >
      {copied ? <FiCheck className="w-4 h-4 text-green-400" /> : <FiClipboard className="w-4 h-4" />}
    </button>
  );
};


export default function InstallationSection() {
  return (
    <section id="installation" className="py-16 md:py-24 bg-neutral-900">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <SectionTitle title="Installation" subtitle="Get Yappus running on your system" />
        
        <div className="mt-12 grid md:grid-cols-2 gap-6 lg:gap-8">
          {installStepsData.map((step, index) => (
            <motion.div
              key={step.os}
              className={`bg-neutral-800/70 rounded-xl border border-neutral-700/80 p-6 shadow-lg backdrop-blur-sm overflow-hidden relative bg-gradient-to-br ${step.bgClass}`}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true, amount: 0.2 }}
              transition={{ duration: 0.5, delay: index * 0.1, ease: "easeOut" }}
            >
              <div className="flex items-center mb-5">
                {step.icon}
                <h3 className="ml-3 text-xl font-semibold text-slate-100">{step.os}</h3>
              </div>
              <div className="relative group">
                <pre className="bg-neutral-900/80 p-4 rounded-md text-xs text-slate-300 overflow-x-auto whitespace-pre-wrap border border-neutral-700/90 custom-scrollbar" style={{ minHeight: '120px' }}>
                  <code>{step.command}</code>
                </pre>
                <CopyToClipboardButton textToCopy={step.command} />
              </div>
            </motion.div>
          ))}
        </div>
         <p className="text-center mt-12 text-slate-400 text-sm">
          For other systems or manual builds, check the <a href="https://github.com/MostlyKIGuess/Yappus-Term#building" target="_blank" rel="noopener noreferrer" className="text-sky-400 hover:text-sky-300 underline font-medium">GitHub repository</a>.
        </p>
      </div>
    </section>
  );
}
