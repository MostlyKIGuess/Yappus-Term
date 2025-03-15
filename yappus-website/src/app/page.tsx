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
    <div className="min-h-screen bg-gradient-to-b from-gray-900 to-black text-white">
      <Header />

      <main>
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
