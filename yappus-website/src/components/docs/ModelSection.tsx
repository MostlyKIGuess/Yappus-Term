import React from 'react';
import ModelBadge from '../ui/ModelBadge';

const models = [
  { name: "GEMINI_FLASH", description: "Default model, latest and greatest from Gemini 2.0 series.", isCurrent: true },
  { name: "GEMINI_2_5_PRO", description: "Most capable model for complex tasks and reasoning." },
  { name: "GEMINI_2_5_FLASH", description: "High performance with excellent reasoning capabilities." },
  { name: "GEMINI_1_5_PRO", description: "Powerful legacy model, good for general purpose tasks." },
  { name: "GEMINI_1_5_FLASH", description: "Fast and efficient legacy model." },
];

export default function ModelsSection() {
  return (
    <div>
      <h3 className="text-2xl font-semibold text-slate-100 mb-3 text-center sm:text-left">Available Models</h3>
      <p className="text-slate-400 mb-8 text-center sm:text-left">
        Yappus supports various Gemini models. You can switch between them using 
        <code className="bg-neutral-700/80 text-sky-300 px-1.5 py-0.5 rounded-md text-xs mx-1">yappus model [MODEL_NAME]</code> or 
        <code className="bg-neutral-700/80 text-sky-300 px-1.5 py-0.5 rounded-md text-xs mx-1">/model [MODEL_NAME]</code>.
      </p>
      <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-5">
        {models.map(model => (
          <ModelBadge key={model.name} name={model.name} description={model.description} isCurrent={model.isCurrent} />
        ))}
      </div>
    </div>
  );
}
