use crate::fat32_types::Fat32BootSector;
use crate::allocator::HEAP_SIZE;

/// Test: Vérifier la taille du heap
#[test]
fn test_heap_size() {
    assert_eq!(HEAP_SIZE, 1024 * 1024);
}

/// Test: Vérifier que le boot sector fait 512 bytes
#[test]
fn test_boot_sector_size() {
    use core::mem::size_of;
    assert_eq!(size_of::<Fat32BootSector>(), 512);
}
