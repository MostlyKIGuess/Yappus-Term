import React from "react";
import DocExample from "../ui/DocExample";
import ModelBadge from "../ui/ModelBadge";

export default function ModelSection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Model Selection
      </h3>
      <p className="text-gray-300 mb-6">
        Yappus supports multiple Gemini models with different
        capabilities:
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        <DocExample
          title="View Available Models"
          description="See all available AI models"
          command="yappus model"
        />
        <DocExample
          title="Change Model"
          description="Switch to a more powerful model"
          command="yappus model GEMINI_2_5_PRO"
        />
      </div>

      
            <div className="mt-6 grid grid-cols-2 md:grid-cols-3 gap-4">
              <ModelBadge
                name="GEMINI_FLASH"
                description="Latest Gemini 2.0 model (default)"
                colorClass="from-emerald-500 to-green-600"
              />
              <ModelBadge
                name="GEMINI_2_5_PRO"
                description="Most powerful for complex tasks"
                colorClass="from-purple-500 to-pink-500"
              />
              <ModelBadge
                name="GEMINI_2_5_FLASH"
                description="High performance with excellent reasoning"
                colorClass="from-blue-500 to-indigo-600"
              />
              <ModelBadge
                name="GEMINI_1_5_PRO"
                description="Very capable legacy model"
                colorClass="from-orange-500 to-red-600"
              />
              <ModelBadge
                name="GEMINI_1_5_FLASH"
                description="Fast and efficient responses"
                colorClass="from-yellow-500 to-amber-600"
              />
            </div>
    </div>
  );
}
