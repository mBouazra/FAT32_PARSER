#![no_std] // 1. TRÈS IMPORTANT : Dit à Rust de ne PAS utiliser la librairie standard (car nous n'avons PAS d'OS).
#![no_main]
// 2. Fonction qui gère les Erreurs (Panics)
// Si le programme rencontre un problème critique, il vient ici.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Pour l'instant, on boucle sans fin pour que l'ordinateur ne plante pas.
    loop {}
}

// 3. Point d'Entrée Principal
// C'est la TOUTE PREMIÈRE fonction que le processeur exécute.
// Elle doit être "unsafe" et "extern "C"" pour fonctionner sans OS.
#[unsafe(no_mangle)] // <-- CHANGEMENT MAJEUR ICI
pub extern "C" fn _start() -> ! { // <-- on retire le 'unsafe' de cette ligne
    // Pour l'instant, notre programme ne fait rien : il boucle sans fin.
    loop {}
}
