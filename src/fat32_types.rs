// Fichier: src/fat32_types.rs

// Importe une macro pour autoriser les champs non utilisés
#[allow(dead_code)] 
// force l'alignement exact des données du disque
#[repr(packed)]
pub struct Fat32BootSector {
    // Décalage 0x00
    pub jump_boot: [u8; 3],             // Saut d'amorçage
    pub oem_name: [u8; 8],              // Nom OEM

    // Décalage 0x0B
    pub bytes_per_sector: u16,          // Octets par secteur (Ex: 512)
    pub sectors_per_cluster: u8,        // Secteurs par cluster
    pub reserved_sector_count: u16,     // Secteurs réservés
    pub num_fats: u8,                   // Nombre de copies de la FAT
    pub root_entry_count: u16,          // Obsolète
    pub total_sectors_16: u16,          // Obsolète
    pub media_type: u8,                 // Type de média
    pub fat_size_16: u16,               // Obsolète
    pub sectors_per_track: u16,         // Secteurs par piste
    pub num_heads: u16,                 // Nombre de têtes
    pub hidden_sectors: u32,            // Secteurs cachés
    pub total_sectors_32: u32,          // Total de secteurs (pour FAT32)

    // Décalage 0x24
    pub fat_size_32: u32,               // Taille de la FAT en secteurs
    pub ext_flags: u16,                 // Indicateurs étendus
    pub fs_version: u16,                // Version du FS
    pub root_cluster: u32,              // Cluster de départ de la racine
    pub fs_info: u16,                   // Secteur d'information
    pub backup_boot_sector: u16,        // Secteur de secours
    pub reserved_0: [u8; 12],           // Réservé

    // Décalage 0x40
    pub drive_num: u8,                  // Numéro du lecteur
    pub reserved_1: u8,                 // Réservé
    pub boot_signature: u8,             // Signature d'amorçage (0x29)
    pub volume_id: u32,                 // ID du volume
    pub volume_label: [u8; 11],         // Nom du volume
    pub fs_type_label: [u8; 8],         // Label du FS (FAT32)
    pub reserved_2: [u8; 420],          // Remplissage

    // Décalage 0x1FE
    pub boot_signature_end: u16,        // Signature de fin de secteur (0xAA55)
}
