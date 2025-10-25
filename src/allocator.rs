// Fichier: src/allocator.rs

use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    cell::UnsafeCell,
};

/// Taille constante du Heap (1 MiB).
pub const HEAP_SIZE: usize = 1024 * 1024;

/// Espace mémoire statique pour le tas (1 MiB).
#[no_mangle]
pub static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Bump Allocator simple.
pub struct BumpAllocator {
    next: UnsafeCell<usize>,
    heap_start: UnsafeCell<usize>,
    heap_end: UnsafeCell<usize>,
}

impl BumpAllocator {
    /// Construit un nouvel allocateur non initialisé.
    pub const fn new() -> Self {
        BumpAllocator { 
            next: UnsafeCell::new(0), 
            heap_start: UnsafeCell::new(0), 
            heap_end: UnsafeCell::new(0) 
        }
    }

    /// # Safety
    /// Initialise le tas.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        *self.heap_start.get() = heap_start;
        *self.heap_end.get() = heap_start + heap_size;
        *self.next.get() = heap_start;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    /// # Safety
    /// Alloue un bloc mémoire.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let next_ptr = self.next.get();
        let heap_end = *self.heap_end.get(); 
        let next_value = *next_ptr;

        let alloc_start = (next_value + layout.align() - 1) & !(layout.align() - 1);
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end <= heap_end {
            *next_ptr = alloc_end;
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    /// # Safety
    /// Désallocation ignorée.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op
    }
}

// Enregistrement de l'instance comme allocateur global.
#[global_allocator]
pub static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();
