use asr::settings::gui::{set_tooltip, Title};
use asr::settings::Gui;

pub fn set_tooltips() {
    {
        set_tooltip(
            "area_castle_keep",
            "There are 2 Transitions that trigger with this.",
        );
        set_tooltip(
            "area_castle_theatre",
            "There are 2 Transitions that trigger with this.",
        );
        set_tooltip(
            "area_keep_castle",
            "There are 2 Transitions that trigger with this.",
        );
        set_tooltip(
            "area_theatre_castle",
            "There are 2 Transitions that trigger with this.",
        );
    }
}

#[derive(Gui)]
pub struct Settings {
    #[default = true]
    pub start: bool,
    #[default = true]
    pub split: bool,
    #[default = true]
    pub reset: bool,

    #[heading_level = 1]
    golden_keys: Title,
    #[default = true]
    pub bailey_key: bool,
    #[default = true]
    pub underbelly_key: bool,
    #[default = true]
    pub tower_key: bool,
    #[default = true]
    pub keep_key: bool,
    #[default = true]
    pub theatre_key: bool,

    #[heading_level = 1]
    item_splits: Title,
    #[default = true]
    pub dream_breaker: bool,
    #[default = true]
    pub indignation: bool,
    #[default = true]
    pub slide: bool,
    #[default = true]
    pub solar_wind: bool,
    #[default = true]
    pub sun_greaves: bool,
    #[default = true]
    pub sunsetter: bool,
    #[default = true]
    pub cling: bool,
    #[default = true]
    pub ascendant_light: bool,
    #[default = true]
    pub strikebreak: bool,
    #[default = true]
    pub soul_cutter: bool,

    #[heading_level = 1]
    sub_items: Title,
    #[default = true]
    pub clear_mind: bool,
    #[default = true]
    pub empathy: bool,
    #[default = true]
    pub good_graces: bool,
    #[default = true]
    pub martial_prowess: bool,
    #[default = true]
    pub pilgrimage: bool,
    #[default = true]
    pub aerial_finesse: bool,
    #[default = true]
    pub heliiacal_power: bool,
    /// Memento (Map)
    #[default = true]
    pub memento: bool,
    #[default = true]
    pub health_upgrades: bool,

    #[heading_level = 1]
    costume_splits: Title,
    /// Professionalism (Castle Sansa)
    #[default = true]
    pub professionalism: bool,
    /// Guardian (Sansa Keep)
    #[default = true]
    pub guardian: bool,
    /// Chivalry (Empty Bailey)
    #[default = true]
    pub chivalry: bool,
    /// Bleeding Heart (Tower Remains)
    #[default = true]
    pub bleeding_heart: bool,
    /// Nostalgia (The Underbelly)
    #[default = true]
    pub nostalgia: bool,
    /// Devotion (Dillapidated Dungeon)
    #[default = true]
    pub devotion: bool,
    /// Class (Twilight Theater)
    #[default = true]
    pub class: bool,
    /// Sweater (Listless Library)
    #[default = true]
    pub sweater: bool,

    #[heading_level = 3]
    costume_equip_splits: Title,
    /// Default (Basic Vest)
    pub equip_default: bool,
    /// Sun Greaves
    pub equip_greaves: bool,
    /// Cling Gem (Cling Sleeve)
    pub equip_cling: bool,
    /// Give Sybil Pants (Big Pants)
    pub equip_pants: bool,
    /// Professionalism (Professional)
    pub equip_professionalism: bool,
    /// Guardian
    pub equip_guardian: bool,
    /// Chivalry (Soldier)
    pub equip_chivalry: bool,
    /// Bleeding Heart
    pub equip_bleeding_heart: bool,
    /// Nostalgia (XIX)
    pub equip_nostalgia: bool,
    /// Devotion (Sol Sister)
    pub equip_devotion: bool,
    /// Class (Classy)
    pub equip_class: bool,
    /// Sweater (Sleepytime)
    pub equip_sweater: bool,

    #[heading_level = 1]
    area_splits: Title,
    #[heading_level = 2]
    from_dilapidated_dungeon: Title,
    /// Castle Sansa
    #[default = false]
    pub area_dungeon_castle: bool,
    /// The Underbelly
    #[default = false]
    pub area_dungeon_underbelly: bool,
    /// Twilight Theatre
    #[default = false]
    pub area_dungeon_theatre: bool,
    #[heading_level = 2]
    from_castle_sansa: Title,
    /// Dilapidated Dungeon
    #[default = false]
    pub area_castle_dungeon: bool,
    /// Empty Bailey
    #[default = false]
    pub area_castle_bailey: bool,
    /// Sansa Keep (2)
    #[default = false]
    pub area_castle_keep: bool,
    /// Listless Library
    #[default = false]
    pub area_castle_library: bool,
    /// Twilight Theatre (2)
    #[default = false]
    pub area_castle_theatre: bool,
    #[heading_level = 2]
    from_listless_library: Title,
    /// Castle Sansa
    #[default = false]
    pub area_library_castle: bool,
    #[heading_level = 2]
    pub from_empty_bailey: Title,
    /// Castle Sansa
    #[default = false]
    pub area_bailey_castle: bool,
    /// The Underbelly
    #[default = false]
    pub area_bailey_underbelly: bool,
    /// Tower Remains
    #[default = false]
    pub area_bailey_tower: bool,
    /// Twilight Theatre
    #[default = false]
    pub area_bailey_theatre: bool,
    #[heading_level = 2]
    from_sansa_keep: Title,
    /// Castle Sansa (2)
    #[default = false]
    pub area_keep_castle: bool,
    /// The Underbelly
    #[default = false]
    pub area_keep_underbelly: bool,
    /// Twilight Theatre
    #[default = false]
    pub area_keep_theatre: bool,
    #[heading_level = 2]
    from_the_underbelly: Title,
    /// Dilapidated Dungeon
    #[default = false]
    pub area_underbelly_dungeon: bool,
    /// Empty Bailey
    #[default = false]
    pub area_underbelly_bailey: bool,
    /// Sansa Keep
    #[default = false]
    pub area_underbelly_keep: bool,
    #[heading_level = 2]
    from_twilight_theatre: Title,
    /// Dilapidated Dungeon
    #[default = false]
    pub area_theatre_dungeon: bool,
    /// Castle Sansa (2)
    #[default = false]
    pub area_theatre_castle: bool,
    /// Empty Bailey
    #[default = false]
    pub area_theatre_bailey: bool,
    /// Sansa Keep
    #[default = false]
    pub area_theatre_keep: bool,
    #[heading_level = 2]
    from_tower_remains: Title,
    /// Empty Bailey
    #[default = false]
    pub area_tower_bailey: bool,
    /// Final Boss Arena
    #[default = false]
    pub area_tower_boss: bool,
    #[heading_level = 2]
    from_final_boss_arena: Title,
    /// Tower Remains
    #[default = false]
    pub area_boss_tower: bool,
}
