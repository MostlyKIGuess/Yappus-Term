import React, { useState } from "react";
import { motion } from "framer-motion";
import { FiMenu, FiX } from "react-icons/fi";
import { FaGithub, FaBook, FaDownload, FaPuzzlePiece } from "react-icons/fa";

export default function Header() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <header className="fixed w-full bg-gray-900/80 backdrop-blur-md py-4 z-50">
      <div className="max-w-7xl mx-auto px-4 flex justify-between items-center">
        <motion.div 
          className="flex items-center space-x-2"
          whileHover={{ scale: 1.05 }}
          transition={{ type: "spring", stiffness: 400, damping: 10 }}
        >
          <span className="text-emerald-400 font-bold text-xl">Yappus</span>
          <motion.span 
            className="text-white font-light text-xl"
            initial={{ opacity: 1 }}
            animate={{ 
              opacity: [1, 0.7, 1], 
              textShadow: ["0 0 0px rgba(110,231,183,0)", "0 0 10px rgba(110,231,183,0.5)", "0 0 0px rgba(110,231,183,0)"] 
            }}
            transition={{ duration: 2.5, repeat: Infinity, repeatType: "reverse" }}
          >
            Terminal
          </motion.span>
        </motion.div>

        <nav className="hidden md:flex items-center space-x-8">
          <NavLink href="#installation" icon={<FaDownload />}>Installation</NavLink>
          <NavLink href="#docs" icon={<FaBook />}>Documentation</NavLink>
          <NavLink href="#features" icon={<FaPuzzlePiece />}>Features</NavLink>
          <NavLink href="https://github.com/MostlyKIGuess/Yappus-Term" icon={<FaGithub />}>
            GitHub
          </NavLink>
        </nav>

        <button
          className="md:hidden text-white"
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
        >
          {mobileMenuOpen ? <FiX size={24} /> : <FiMenu size={24} />}
        </button>
      </div>

      {mobileMenuOpen && (
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="md:hidden bg-gray-800/90 backdrop-blur-md py-4"
        >
          <div className="flex flex-col space-y-4 px-4">
            <MobileNavLink
              href="#installation"
              onClick={() => setMobileMenuOpen(false)}
              icon={<FaDownload />}
            >
              Installation
            </MobileNavLink>
            <MobileNavLink
              href="#docs"
              onClick={() => setMobileMenuOpen(false)}
              icon={<FaBook />}
            >
              Documentation
            </MobileNavLink>
            <MobileNavLink
              href="#features"
              onClick={() => setMobileMenuOpen(false)}
              icon={<FaPuzzlePiece />}
            >
              Features
            </MobileNavLink>
            <MobileNavLink
              href="https://github.com/MostlyKIGuess/Yappus-Term"
              onClick={() => setMobileMenuOpen(false)}
              icon={<FaGithub />}
            >
              GitHub
            </MobileNavLink>
          </div>
        </motion.div>
      )}
    </header>
  );
}

interface NavLinkProps {
  href: string;
  children: React.ReactNode;
  icon?: React.ReactNode;
}

function NavLink({ href, children, icon }: NavLinkProps) {
  return (
    <motion.a
      href={href}
      className="text-gray-300 hover:text-emerald-400 transition-colors duration-200 flex items-center gap-2 group"
      whileHover={{ scale: 1.05 }}
      transition={{ type: "spring", stiffness: 400, damping: 17 }}
    >
      {icon && (
        <motion.span 
          className="text-emerald-400 opacity-70 group-hover:opacity-100" 
          whileHover={{ rotate: 360 }}
          transition={{ duration: 0.5 }}
        >
          {icon}
        </motion.span>
      )}
      {children}
    </motion.a>
  );
}

interface MobileNavLinkProps {
  href: string;
  onClick: () => void;
  children: React.ReactNode;
  icon?: React.ReactNode;
}

function MobileNavLink({ href, onClick, children, icon }: MobileNavLinkProps) {
  return (
    <motion.a
      href={href}
      onClick={onClick}
      className="text-gray-300 hover:text-emerald-400 py-3 transition-colors duration-200 flex items-center gap-3 border-b border-gray-700/50 last:border-b-0"
      whileTap={{ scale: 0.97 }}
    >
      {icon && (
        <span className="text-emerald-400 w-5 h-5 flex items-center justify-center">
          {icon}
        </span>
      )}
      <span className="text-lg">{children}</span>
    </motion.a>
  );
}
