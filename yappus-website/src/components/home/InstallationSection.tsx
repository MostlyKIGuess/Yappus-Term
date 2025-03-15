import React from "react";
import { motion } from "framer-motion";
import { FaCopy, FaCheck } from "react-icons/fa";

export default function InstallationSection() {
  return (
    <section
      id="installation"
      className="px-4 py-20 bg-gray-900/50 backdrop-blur-sm"
    >
      <div className="max-w-7xl mx-auto">
        <h2 className="text-4xl font-bold text-center mb-12">
          Installation
        </h2>

        <div className="grid md:grid-cols-1 lg:grid-cols-3 gap-8">
          <InstallPlatform
            title="Debian/Ubuntu"
            icon="🐧"
            commands={[
              "curl -O https://raw.githubusercontent.com/MostlyKIGuess/Yappus-Term/main/install-yappus.sh",
              "chmod +x install-yappus.sh",
              "./install-yappus.sh"
            ]}
          />
          
          <InstallPlatform
            title="Arch Linux"
            icon="🏹"
            commands={[
              "# With yay",
              "yay -S yappus",
              "",
              "# Or with PKGBUILD",
              "git clone https://github.com/MostlyKIGuess/Yappus-Term.git",
              "cd Yappus-Term",
              "makepkg -si"
            ]}
          />
          
          <InstallPlatform
            title="From Source"
            icon="🛠️"
            commands={[
              "git clone https://github.com/MostlyKIGuess/Yappus-Term.git",
              "cd yappus-term",
              "cargo build --release",
              "",
              "# The binary will be in:",
              "# target/release/yappus"
            ]}
          />
        </div>
      </div>
    </section>
  );
}

type InstallPlatformProps = {
  title: string;
  icon: string;
  commands: string[];
};

function InstallPlatform({ title, icon, commands }: InstallPlatformProps) {
  return (
    <div className="bg-gray-800 rounded-xl overflow-hidden border border-gray-700 shadow-lg transition-shadow duration-300 hover:shadow-emerald-900/20">
      <div className="p-6 border-b border-gray-700">
        <div className="flex items-center gap-3">
          <span className="text-3xl">{icon}</span>
          <h3 className="text-xl font-bold">{title}</h3>
        </div>
      </div>
      <div className="bg-gray-900 divide-y divide-gray-800/90">
        {commands.map((cmd, idx) => (
          <CommandLine key={idx} command={cmd} />
        ))}
      </div>
    </div>
  );
}

function CommandLine({ command }: { command: string }) {
  const [copied, setCopied] = React.useState(false);
  
  if (!command) return <div className="h-2 bg-gray-900"></div>;
  
  const isComment = command.startsWith('#');

  const copyToClipboard = () => {
    if (isComment) return;
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className={`p-4 flex justify-between items-center group ${isComment ? 'bg-opacity-50' : ''}`}>
      <pre className={`font-mono text-sm whitespace-pre-wrap flex-1 ${isComment ? 'text-gray-500' : 'text-emerald-300'}`}>
        {command}
      </pre>
      {!isComment && (
        <motion.button
          onClick={copyToClipboard}
          className="ml-4 p-2 rounded bg-gray-800 hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity"
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.9 }}
          title="Copy to clipboard"
          aria-label="Copy to clipboard"
        >
          {copied ? (
            <FaCheck className="text-emerald-400 w-4 h-4" />
          ) : (
            <FaCopy className="text-gray-400 w-4 h-4" />
          )}
        </motion.button>
      )}
    </div>
  );
}
