pub fn get_offsets(version: &GameVersion) -> Offsets {
    match version {
        // Full Gold Patch
        GameVersion::FullGoldPatch => Offsets {
            big_key_bailey: [0, 0x1B8, 0x339],
            big_key_underbelly: [0, 0x1B8, 0x33A],
            big_key_tower: [0, 0x1B8, 0x33B],
            big_key_keep: [0, 0x1B8, 0x33C],
            big_key_theatre: [0, 0x1B8, 0x488],
            upgrade_tracker: [0, 0x1B8, 0x1D],
            boss_phase: [0, 0x30, 0xE8, 0x2A8, 0x764],
            silver_keys: [0, 0x1B8, 0x1C8],
            health_upgrade_count: [0, 0x1B8, 0x260],
            fguid: [0, 0x30, 0x210],
            final_boss_hp: [0, 0x30, 0xE8, 0x2A8, 0x638, 0xA8],
            area_name: [0, 0x1B8, 0x2A0, 0x0],
            current_outfit: [0, 0x1B8, 0x48C],
        },
        GameVersion::MapUpdate => Offsets {
            big_key_bailey: [0, 0x1B8, 0x389],
            big_key_underbelly: [0, 0x1B8, 0x38A],
            big_key_tower: [0, 0x1B8, 0x38B],
            big_key_keep: [0, 0x1B8, 0x38C],
            big_key_theatre: [0, 0x1B8, 0x4D8],
            upgrade_tracker: [0, 0x1B8, 0x1D8],
            boss_phase: [0, 0x30, 0xE8, 0x2A8, 0x764],
            silver_keys: [0, 0x1B8, 0x1C8],
            health_upgrade_count: [0, 0x1B8, 0x260],
            fguid: [0, 0x30, 0x210],
            final_boss_hp: [0, 0x30, 0xE8, 0x2A8, 0x638, 0xA8],
            area_name: [0, 0x1B8, 0x2A0, 0x0],
            current_outfit: [0, 0x1B8, 0x4DC],
        },
    }
}
pub enum GameVersion {
    FullGoldPatch = 0,
    MapUpdate = 1,
}
pub(crate) struct Offsets {
    pub big_key_bailey: [u64; 3],
    pub big_key_underbelly: [u64; 3],
    pub big_key_tower: [u64; 3],
    pub big_key_keep: [u64; 3],
    pub big_key_theatre: [u64; 3],
    pub upgrade_tracker: [u64; 3],
    pub boss_phase: [u64; 5],
    pub silver_keys: [u64; 3],
    pub health_upgrade_count: [u64; 3],
    pub fguid: [u64; 3],
    pub final_boss_hp: [u64; 6],
    pub area_name: [u64; 4],
    pub current_outfit: [u64; 3],
}
