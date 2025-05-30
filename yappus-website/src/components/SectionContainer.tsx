import React from 'react';

interface SectionContainerProps {
  id: string;
  title: string;
  subtitle?: string;
  children: React.ReactNode;
  className?: string;
  bgClassName?: string;
}

export default function SectionContainer({ id, title, subtitle, children, className = "", bgClassName = "bg-neutral-900" }: SectionContainerProps) {
  return (
    <section id={id} className={`py-16 md:py-24 ${bgClassName} ${className}`}>
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-12 md:mb-16">
          <h2 className="text-3xl sm:text-4xl font-bold text-slate-100 mb-3 tracking-tight">
            {title}
          </h2>
          {subtitle && (
            <p className="text-lg text-slate-400 max-w-3xl mx-auto">
              {subtitle}
            </p>
          )}
        </div>
        {children}
      </div>
    </section>
  );
}
