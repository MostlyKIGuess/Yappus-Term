import React from 'react';
import { motion } from 'framer-motion';

const examples = [
  {
    title: "Code Generation",
    description: "Ask Yappus to write a Python script to fetch weather data.",
    conversation: [
      { user: "> Generate a Python script to get weather from an API" },
      { ai: "Sure, here's a Python script using the requests library and OpenWeatherMap API:\n```python\nimport requests\nimport json\n\nAPI_KEY = 'YOUR_API_KEY'\nCITY = 'London'\nURL = f'http://api.openweathermap.org/data/2.5/weather?q={CITY}&appid={API_KEY}&units=metric'\n\nresponse = requests.get(URL)\nif response.status_code == 200:\n  data = response.json()\n  main = data['main']\n  print(f\"Weather in {CITY}: {data['weather'][0]['description']}, Temp: {main['temp']}°C\")\nelse:\n  print('Error fetching weather data')\n```" }
    ]
  },
  {
    title: "File Context Analysis",
    description: "Analyze a configuration file and ask for improvements.",
    conversation: [
      { user: "> /file Dockerfile" },
      { ai: "File content from Dockerfile added to context." },
      { user: "> What could be improved in this Dockerfile for a Node.js app?" },
      { ai: "Based on your Dockerfile, consider these improvements:\n1. Use a more specific base image (e.g., `node:18-alpine`).\n2. Implement multi-stage builds to reduce final image size.\n3. Ensure you're copying only necessary files (`.dockerignore`).\n4. Run as a non-root user for better security." }
    ]
  },
  {
    title: "Command Piping & Explanation",
    description: "Pipe a shell command and ask Yappus to explain it.",
    conversation: [
      { user: "> /ls | what is this project?" },
      { ai: "Looks like your downloads directory to me." }
    ]
  }
];

export default function ExamplesSection() {
  return (
    <div>
      <h3 className="text-2xl font-semibold text-slate-100 mb-3 text-center sm:text-left">Usage Examples</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">See Yappus in action with these common use cases.</p>
      <div className="space-y-8">
        {examples.map((example, index) => (
          <motion.div 
            key={example.title}
            className="bg-neutral-800/70 rounded-xl border border-neutral-700/80 shadow-lg overflow-hidden backdrop-blur-sm"
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true, amount: 0.2 }}
            transition={{ duration: 0.5, delay: index * 0.1, ease: "easeOut" }}
          >
            <div className="p-5 border-b border-neutral-700/80">
              <h4 className="text-lg font-semibold text-slate-100">{example.title}</h4>
              <p className="text-sm text-slate-400 mt-1">{example.description}</p>
            </div>
            <div className="p-5 font-mono text-xs sm:text-sm leading-relaxed">
              {example.conversation.map((line, i) => line.user ? (
                  <div key={i} className="mb-2">
                    <span className="text-slate-500 select-none">{line.user.startsWith('>') ? '' : '$ '}</span>
                    <span className="text-slate-200">{line.user}</span>
                  </div>
                ) : (
                  <div key={i} className="mb-2 pl-2 border-l-2 border-sky-500/40">
                    <pre className="whitespace-pre-wrap text-slate-300">{line.ai}</pre>
                  </div>
                )
              )}
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
}
