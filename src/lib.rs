#![no_std] // Pas standard.
#![no_main] // Pas main.
#![feature(allocator_api)] // API allocateur.
#![feature(raw_ref_op)] // Fixe allocateur.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

mod fat32_types; 
use fat32_types::Fat32BootSector; 

// --- GESTION ERREUR & AFFICHAGE (VGA) ---

// Adresse VGA.
const VGA_BUFFER_ADDR: usize = 0xb8000;

// Gère panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Écrit caractère.
unsafe fn vga_write_byte(byte: u8, color: u8, x: usize, y: usize) {
    let index = (y * 80 + x) * 2; 
    let buffer = VGA_BUFFER_ADDR as *mut u8;
    
    // Écrit octet.
    *buffer.add(index) = byte;
    *buffer.add(index + 1) = color; 
}

// Affiche nombre.
fn number_to_vga(num: u32, buffer: &mut [u8]) -> &[u8] {
    let mut temp = num;
    let mut i = buffer.len();
    
    if temp == 0 {
        i -= 1;
        buffer[i] = b'0';
    } else {
        while temp > 0 && i > 0 {
            i -= 1;
            buffer[i] = (temp % 10) as u8 + b'0';
            temp /= 10;
        }
    }
    // Retourne chiffres.
    &buffer[i..]
}

// --- ALLOCATEUR GLOBAL (HEAP) ---

// Taille HEAP.
const HEAP_SIZE: usize = 1024 * 1024; 

// Bloc HEAP.
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// Structure allocateur.
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize, 
}

impl BumpAllocator {
    // Crée instance.
    pub const fn new() -> Self {
        BumpAllocator { heap_start: 0, heap_end: 0, next: 0 }
    }
}

// Implémente allocation.
unsafe impl GlobalAlloc for BumpAllocator {
    
    // Alloue espace.
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        ptr::null_mut() // Squelette
    }

    // Désalloue espace.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Ignoré
    }
}

// Allocateur global.
#[global_allocator]
static mut ALLOCATOR: BumpAllocator = BumpAllocator::new(); 


// --- POINT D'ENTRÉE & PARSING FAT32 ---

// Adresse disque.
const DISK_START_ADDR: usize = 0x800000;

// Fonction de démarrage.
#[unsafe(no_mangle)] 
pub extern "C" fn _start() -> ! {
    
    // Initialisation allocateur.
    unsafe {
        // Début HEAP.
        ALLOCATOR.heap_start = &raw const HEAP as *const u8 as usize; 
        ALLOCATOR.heap_end = ALLOCATOR.heap_start + HEAP_SIZE;
        ALLOCATOR.next = ALLOCATOR.heap_start; 
    }

    // Pointeur Boot Sector.
    let boot_sector_ptr = DISK_START_ADDR as *const Fat32BootSector;
    
    // Lit Boot Sector.
    let boot_sector = unsafe { 
        &*boot_sector_ptr 
    };

    // Octets par secteur.
    let bytes_per_sector = boot_sector.bytes_per_sector; 
    
    // Message à afficher.
    let message = b"FAT32 Parser: BpS = ";
    let color = 0x0A; // Couleur vert.

    // Affiche message.
    for (idx, &byte) in message.iter().enumerate() {
        unsafe {
            vga_write_byte(byte, color, idx, 0); 
        }
    }
    
    // Buffer nombre.
    let mut buffer = [0u8; 5]; 
    let display_bytes = number_to_vga(bytes_per_sector as u32, &mut buffer);
    let offset = message.len();
    
    // Affiche nombre.
    for (idx, &byte) in display_bytes.iter().enumerate() {
        unsafe {
            vga_write_byte(byte, color, offset + idx, 0); 
        }
    }
    
    loop {} // Boucle infinie.
}
