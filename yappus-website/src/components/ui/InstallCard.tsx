import React from "react";

interface InstallCardProps {
  title: string;
  icon: string;
  commands: string[];
}

export default function InstallCard({ title, icon, commands }: InstallCardProps) {
  return (
    <div className="bg-gray-800 rounded-xl overflow-hidden border border-gray-700 shadow-lg hover:shadow-emerald-900/20 hover:border-emerald-800/50 transition duration-300">
      <div className="p-6 border-b border-gray-700">
        <div className="flex items-center gap-3">
          <span className="text-3xl">{icon}</span>
          <h3 className="text-xl font-bold">{title}</h3>
        </div>
      </div>
      <div className="bg-gray-900 p-4 font-mono text-sm text-emerald-300 overflow-x-auto">
        {commands.map((cmd, idx) => (
          <div key={idx} className={cmd.startsWith("#") ? "text-gray-500" : ""}>
            {cmd ? cmd : <br />}
          </div>
        ))}
      </div>
    </div>
  );
}
