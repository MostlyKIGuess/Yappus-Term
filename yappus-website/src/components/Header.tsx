"use client";
import React, { useState, useEffect } from "react";
import { motion } from "framer-motion";
import { FaGithub, FaBook, FaDownload, FaPuzzlePiece } from "react-icons/fa";
import { FiMenu, FiX } from "react-icons/fi";

interface NavLinkProps {
  href: string;
  children: React.ReactNode;
  icon?: React.ReactNode;
}

interface MobileNavLinkProps extends NavLinkProps {
  onClick: () => void;
}

export default function Header() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <header 
      className={`fixed w-full py-3 z-50 transition-all duration-300 ease-out
                  ${isScrolled ? 'bg-neutral-900/80 backdrop-blur-lg border-b border-neutral-700/60 shadow-lg' : 'bg-transparent border-b border-transparent'}`}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center">
        <motion.a
          href="#"
          className="flex items-center space-x-2"
          whileHover={{ scale: 1.05 }}
          transition={{ type: "spring", stiffness: 400, damping: 17 }}
        >
          <span className="text-slate-100 font-bold text-2xl tracking-tight">Yappus</span>
          <motion.span 
            className="text-slate-400 font-light text-2xl tracking-tight"
            initial={{ opacity: 0.8 }}
            animate={{ opacity: [0.8, 0.5, 0.8] }}
            transition={{ duration: 3, repeat: Infinity, repeatType: "reverse", ease: "easeInOut" }}
          >
            Terminal
          </motion.span>
        </motion.a>

        <nav className="hidden md:flex items-center space-x-7">
          <NavLink href="#installation" icon={<FaDownload />}>Installation</NavLink>
          <NavLink href="#documentation" icon={<FaBook />}>Docs</NavLink>
          <NavLink href="#features" icon={<FaPuzzlePiece />}>Features</NavLink>
          <NavLink href="https://github.com/MostlyKIGuess/Yappus-Term" icon={<FaGithub />}>
            GitHub
          </NavLink>
        </nav>

        <button
          className="md:hidden text-slate-300 hover:text-slate-100 transition-colors"
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          aria-label="Toggle menu"
        >
          {mobileMenuOpen ? <FiX size={26} /> : <FiMenu size={26} />}
        </button>
      </div>

      {mobileMenuOpen && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: 'auto' }}
          exit={{ opacity: 0, height: 0 }}
          transition={{ duration: 0.3, ease: "easeInOut" }}
          className="md:hidden bg-neutral-800/95 backdrop-blur-md mt-2 mx-2 rounded-lg shadow-xl overflow-hidden border border-neutral-700"
        >
          <nav className="flex flex-col px-3 py-3 space-y-1">
            <MobileNavLink href="#installation" onClick={() => setMobileMenuOpen(false)} icon={<FaDownload />}>Installation</MobileNavLink>
            <MobileNavLink href="#documentation" onClick={() => setMobileMenuOpen(false)} icon={<FaBook />}>Documentation</MobileNavLink>
            <MobileNavLink href="#features" onClick={() => setMobileMenuOpen(false)} icon={<FaPuzzlePiece />}>Features</MobileNavLink>
            <MobileNavLink href="https://github.com/MostlyKIGuess/Yappus-Term" onClick={() => setMobileMenuOpen(false)} icon={<FaGithub />}>GitHub</MobileNavLink>
          </nav>
        </motion.div>
      )}
    </header>
  );
}

function NavLink({ href, children, icon }: NavLinkProps) {
  return (
    <motion.a
      href={href}
      className="text-slate-400 hover:text-slate-100 transition-colors duration-200 flex items-center gap-1.5 group text-sm font-medium py-2"
      whileHover={{ y: -2 }}
      transition={{ type: "spring", stiffness: 300, damping: 15 }}
    >
      {icon && (
        <span className="text-slate-500 group-hover:text-slate-300 transition-colors duration-200">
          {React.cloneElement(icon as React.ReactElement, )}
        </span>
      )}
      {children}
    </motion.a>
  );
}

function MobileNavLink({ href, onClick, children, icon }: MobileNavLinkProps) {
  return (
    <motion.a
      href={href}
      onClick={onClick}
      className="text-slate-300 hover:bg-slate-700 hover:text-slate-100 px-3 py-3 rounded-md transition-all duration-200 flex items-center gap-3 text-base"
      whileTap={{ scale: 0.98, backgroundColor: 'rgb(51 65 85)'}} 
    >
      {icon && (
        <span className="text-slate-400 w-5 h-5 flex items-center justify-center">
          {React.cloneElement(icon as React.ReactElement, )}
        </span>
      )}
      <span className="font-medium">{children}</span>
    </motion.a>
  );
}
