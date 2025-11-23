// Fichier: src/main.rs
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use bootloader::entry_point;
use core::panic::PanicInfo;
use alloc::vec::Vec;
use fat32_parser::{allocator, Fat32BootSector};

// --- CONSTANTES & VGA ---
const VGA_BUFFER_ADDR: usize = 0xb8000;
const DISK_START_ADDR: usize = 0x800000;

unsafe fn vga_print_char(byte: u8, color: u8, offset: usize) {
    let buffer = VGA_BUFFER_ADDR as *mut u8;
    *buffer.add(offset * 2) = byte;
    *buffer.add(offset * 2 + 1) = color;
}

// --- PANIC HANDLER ---
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// --- POINT D'ENTRÉE DU NOYAU ---
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    // 1. Initialisation de l'allocateur.
    unsafe {
        allocator::init_heap();
    }

    // 2. Test de l'allocateur.
    let mut numbers = Vec::new();
    numbers.push(42);

    // 3. Lecture du Boot Sector FAT32.
    let boot_sector_ptr = DISK_START_ADDR as *const Fat32BootSector;
    let boot_sector = unsafe { &*boot_sector_ptr };

    // 4. Affichage de succès.
    unsafe {
        vga_print_char(b'O', 0x0A, 0);
        vga_print_char(b'K', 0x0A, 1);
        let bps_byte = (boot_sector.bytes_per_sector / 100) as u8;
        vga_print_char(bps_byte + b'0', 0x0B, 3);
        let vec_val_char = (numbers[0] + 20) as u8;
        vga_print_char(vec_val_char, 0x0C, 5);
    }

    loop {}
}
