// Fichier: src/lib.rs
#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub mod allocator;
pub mod fat32_types;

// Re-export for convenience
pub use fat32_types::Fat32BootSector;
