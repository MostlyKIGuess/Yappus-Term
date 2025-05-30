"use client";

import React from 'react';
import Header from "../components/Header";
import Footer from "../components/Footer";
import HeroSection from "../components/home/HeroSection";
import TerminalPreview from "../components/home/TerminalPreview";
import FeaturesSection from "../components/home/FeaturesSection";
import InstallationSection from "../components/home/InstallationSection";
import DocumentationSection from "../components/docs/DocumentationSection";

export default function HomePage() {
  return (
    <div className="min-h-screen bg-neutral-900 text-slate-200 selection:bg-slate-600 selection:text-slate-100"> 
      <Header />
      <main className="overflow-x-hidden">
        <HeroSection />
        <TerminalPreview />
        <InstallationSection />
        <DocumentationSection />
        <FeaturesSection />
      </main>
      <Footer />
    </div>
  );
}
