// Fichier: src/lib.rs

//!  sources
//! 
//! - phil Opp development tutorial: https://os.phil-opp.com/
//! - rust  programming book
//! - rust embedded book: https://docs.rust-embedded.org/book/
//! - Cours ESGI 4A 


#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub mod allocator;
pub mod fat32_types;

// Re-export for convenience
pub use fat32_types::Fat32BootSector;
#[cfg(test)]
mod tests;
