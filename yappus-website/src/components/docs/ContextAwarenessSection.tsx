import React from 'react';

export default function ContextAwarenessSection() {
  return (
    <div className="py-16 px-4 md:px-8 max-w-6xl mx-auto" id="context-awareness">
      <h3 className="text-4xl font-semibold mb-4 text-emerald-400">
        Context Awareness
      </h3>
      <div className="mt-10 space-y-8 text-gray-300">
        <div>
          <h3 className="text-2xl font-semibold text-white mb-4">Persistent Conversation Context</h3>
          <p className="mb-4">
            Yappus Terminal maintains context between your queries, enabling more coherent and relevant AI responses.
            The terminal preserves conversation history, allowing the AI to reference previous exchanges and provide
            more accurate answers based on the full context of your conversation.
          </p>
          
          <div className="bg-gray-800 rounded-lg p-4 font-mono text-sm mt-4">
            <p className="text-green-400">$ yappus What is JavaScript?</p>
            <p className="text-gray-400 mt-2">JavaScript is a programming language commonly used for web development...</p>
            <p className="text-green-400 mt-3">$ yappus What about TypeScript?</p>
            <p className="text-gray-400 mt-2">TypeScript is a superset of JavaScript that adds static type definitions...</p>
          </div>
        </div>
        
        <div>
          <h3 className="text-2xl font-semibold text-white mb-4">File Context Integration</h3>
          <p className="mb-4">
            Yappus seamlessly incorporates file content into your AI conversations. Reference files using
            the <code className="bg-gray-700 px-1 rounded">/file</code> command to make them part of the
            conversation context.
          </p>
          
          <div className="bg-gray-800 rounded-lg p-4 font-mono text-sm mt-4">
            <p className="text-green-400">$ yappus /file src/utils.js</p>
            <p className="text-gray-400 mt-2">File content added to conversation context.</p>
            <p className="text-green-400 mt-3">$ yappus What does the parseConfig function do?</p>
            <p className="text-gray-400 mt-2">The parseConfig function in your utils.js file parses the JSON configuration...</p>
          </div>
        </div>
      </div>
    </div>
  );
}
