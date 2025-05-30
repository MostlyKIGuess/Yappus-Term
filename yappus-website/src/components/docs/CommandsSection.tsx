import React from 'react';
import DocExample from '../ui/DocExample';

const commands = [
  { title: "Direct Query", description: "Ask a question directly from your terminal.", command: 'yappus "How do I find large files in Linux?"' },
  { title: "Change Model", description: "Switch to a different Gemini model.", command: 'yappus model GEMINI_2_5_PRO' },
  { title: "View History", description: "Display your chat history.", command: 'yappus history' },
  { title: "Clear History", description: "Clear all saved chat history.", command: 'yappus clear-history' },
  { title: "Run Setup", description: "Re-run the initial setup for API key and model.", command: 'yappus setup' },
  { title: "Analyze File", description: "Include a file's content for context in your query.", command: 'yappus file script.sh "explain this script"' },
  { title: "Export Chats", description: "Export your chat history to a JSON file.", command: 'yappus export ~/my_chat_export.json' },
];

const interactiveCommands = [
    { title: "/help", description: "Show help message with available interactive commands.", command: "/help" },
    { title: "/model [name]", description: "View or change the Gemini model.", command: "/model GEMINI_FLASH" },
    { title: "/history", description: "View your chat history within the interactive session.", command: "/history" },
    { title: "/clearhistory", description: "Clear your chat history.", command: "/clearhistory" },
    { title: "/file <path> [query]", description: "Include file content in the conversation.", command: "/file src/main.js explain this function" },
    { title: "/ls [path]", description: "List directory contents.", command: "/ls ./src" },
    { title: "/cd <path>", description: "Change current directory.", command: "/cd ../project" },
    { title: "/pwd", description: "Show current working directory.", command: "/pwd" },
    { title: "exit", description: "Exit the Yappus interactive session.", command: "exit" },
];

export default function CommandsSection() {
  return (
    <div>
      <h3 className="text-2xl font-semibold text-slate-100 mb-3 text-center sm:text-left">Command Line Arguments</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">Use Yappus directly from your shell for quick tasks.</p>
      <div className="grid md:grid-cols-2 gap-x-6 gap-y-6">
        {commands.map(cmd => <DocExample key={cmd.title} {...cmd} />)}
      </div>

      <h3 className="text-2xl font-semibold text-slate-100 mb-3 mt-12 md:mt-16 text-center sm:text-left">Interactive Mode Commands</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">Commands available within the Yappus interactive session (start with `yappus`):</p>
      <div className="grid md:grid-cols-2 gap-x-6 gap-y-6">
        {interactiveCommands.map(cmd => <DocExample key={cmd.title} {...cmd} />)}
      </div>
    </div>
  );
}
