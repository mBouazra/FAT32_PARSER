#![no_std] // 1. Désactive la librairie standard (pas d'OS)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // 2. Gestionnaire d'erreur : en cas de bug, on boucle indéfiniment
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 3. Point d'entrée : la première fonction exécutée par le processeur
    // Dans notre cas, on boucle aussi pour l'instant.
    loop {}
}
