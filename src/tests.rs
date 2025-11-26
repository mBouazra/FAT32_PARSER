use crate::fat32_types::Fat32BootSector;
use crate::allocator::HEAP_SIZE;

/// test1  vérifier la taille du heap
#[test]
fn test_heap_size() {
    assert_eq!(HEAP_SIZE, 1024 * 1024);
}

/// Tes2t: verifier que le boot sector fait 512 bytes
#[test]
fn test_boot_sector_size() {
use core::mem::size_of;
assert_eq!(size_of::<Fat32BootSector>(), 512);
}


/// Test3: vérifier qu'un boot sector valide est détecté
#[test]
fn test_valid_boot_sector() {let mut boot_sector = unsafe { core::mem::zeroed::<Fat32BootSector>() };
boot_sector.boot_signature_end = 0xAA55;
assert!(boot_sector.is_valid());
}

/// Test 4: Vérifier qu'un boot sector invalide est détecté
#[test]
fn test_invalid_boot_sector() {
    let mut boot_sector = unsafe { core::mem::zeroed::<Fat32BootSector>() };
boot_sector.boot_signature_end = 0x0000;
    
    assert!(!boot_sector.is_valid());}

/// Test vérifier le calcul du total de secteurs
#[test]
fn test_total_sectors() {
let mut boot_sector = unsafe { core::mem::zeroed::<Fat32BootSector>() };
boot_sector.total_sectors_32 = 2048000;
    
    assert_eq!(boot_sector.total_sectors(), 2048000);}


/// Test 6: Vérifier le calcul du premier secteur de données
#[test]
fn test_first_data_sector() {
 let mut boot_sector = unsafe { core::mem::zeroed::<Fat32BootSector>() }
boot_sector.reserved_sector_count = 32;
boot_sector.num_fats = 2;
boot_sector.fat_size_32 = 1000;
    
  // Calcul 32+(2*1000)= 2032
  assert_eq!(boot_sector.first_data_sector(), 2032);}
