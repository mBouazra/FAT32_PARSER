// Fichier: src/main.rs


//! ## Sources
//! - bootloader crate: https://github.com/rust-osdev/bootloader
//! - phil opp tutoriel: https://os.phil-opp.com/
//! - Cours ESGI 4A


#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use bootloader::entry_point;
use core::panic::PanicInfo;
use alloc::vec::Vec;
use fat32_parser::{allocator, Fat32BootSector};

/// L'adresse où le bootloader a mis le disque en mémoire
/// J'ai trouvé cette adresse dans les docs de bootloader
const DISK_START_ADDR: usize = 0x800000;

/// Panic handler - quand y'a une erreur critique
/// Pour l'instant on loop juste indéfiniment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/* Le point d'entrée du kernel*/

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    // Étape 1: Init l'allocateur sinon on peut pas utiliser Vec
    unsafe {
        allocator::init_heap();
    }

    // Étape 2: Tester que l'allocateur marche avec un Vec
    let mut numbers = Vec::new();
    numbers.push(42);
    numbers.push(100);
    numbers.push(255);
    
    // Si ça panic pas ici c'est que l'allocateur marche
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers[0], 42);

    // Étape 3: Lire le boot sector depuis le disque
    // On cast juste le pointeur vers notre struct
    let boot_sector_ptr = DISK_START_ADDR as *const Fat32BootSector;
    let boot_sector = unsafe { &*boot_sector_ptr };

	//etape4:parser les infos du boot sector 
	let info = boot_sector.get_info();
    
    let _is_valid = info.is_valid;
    let _total_sectors = info.total_sectors;
    let _first_data = info.first_data_sector;
	let _bytes_per_sector = info.bytes_per_sector;
    // Un kernel doit jamais terminer donc on loop
    loop {}
}
