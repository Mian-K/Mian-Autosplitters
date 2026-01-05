#![no_std]

mod autosplitter_settings;
mod version_offsets;

use crate::version_offsets::{get_offsets, GameVersion};
use asr::{
    future::{next_tick, retry},
    game_engine::unreal::{FNameKey, Module, Version},
    print_message,
    settings::Gui,
    string::ArrayString,
    timer::{
        reset, set_variable, set_variable_float, set_variable_int, split, start, state, TimerState,
    },
    watcher::Watcher,
    PointerSize::Bit64,
    Process,
};

asr::async_main!(stable);
asr::panic_handler!();
const PROCESS_NAMES: &[&str] = &["pseudoregalia-Win64-Shipping.exe", "pseudoregalia.exe"];

async fn main() {
    let mut settings = autosplitter_settings::Settings::register();
    autosplitter_settings::set_tooltips();

    let area_dungeon: ArrayString<20> = ArrayString::from(&"ZONE_Dungeon").unwrap();
    let area_castle: ArrayString<20> = ArrayString::from(&"ZONE_LowerCastle").unwrap();
    let area_library: ArrayString<20> = ArrayString::from(&"Zone_Library").unwrap();
    let area_bailey: ArrayString<20> = ArrayString::from(&"ZONE_Exterior").unwrap();
    let area_keep: ArrayString<20> = ArrayString::from(&"Zone_Upper").unwrap();
    let area_underbelly: ArrayString<20> = ArrayString::from(&"Zone_Caves").unwrap();
    let area_theatre: ArrayString<20> = ArrayString::from(&"Zone_Theatre").unwrap();
    let area_tower: ArrayString<20> = ArrayString::from(&"Zone_Tower").unwrap();
    let area_princess: ArrayString<20> = ArrayString::from(&"Zone_PrincessChamber").unwrap();
    let area_title: ArrayString<20> = ArrayString::from(&"TitleScreen").unwrap();

    let costume_base: ArrayString<7> = ArrayString::from(&"Base").unwrap();
    let costume_glove: ArrayString<7> = ArrayString::from(&"glove").unwrap();
    let costume_greaves: ArrayString<7> = ArrayString::from(&"greaves").unwrap();
    let costume_pro: ArrayString<7> = ArrayString::from(&"pro").unwrap();
    let costume_pants: ArrayString<7> = ArrayString::from(&"pants").unwrap();
    let costume_nun: ArrayString<7> = ArrayString::from(&"nun").unwrap();
    let costume_shoujo: ArrayString<7> = ArrayString::from(&"shoujo").unwrap();
    let costume_knight: ArrayString<7> = ArrayString::from(&"knight").unwrap();
    let costume_past: ArrayString<7> = ArrayString::from(&"past").unwrap();
    let costume_jam: ArrayString<7> = ArrayString::from(&"jam").unwrap();
    let costume_class: ArrayString<7> = ArrayString::from(&"Class").unwrap();
    let costume_sweater: ArrayString<7> = ArrayString::from(&"sweater").unwrap();

    print_message("Autosplitter initialized. Searching for process...");

    loop {
        let mut process_name: &str = "";
        let process = retry(|| {
            PROCESS_NAMES.iter().find_map(|name| {
                process_name = name;
                Process::attach(name)
            })
        })
        .await;
        process.until_closes(async {
            print_message("Process Found: ");
            print_message(process_name);
            let version: Option<GameVersion>;
            {
                let size = retry(|| process.get_module_size(process_name)).await;
                match size {
                    2125824 | 246771712 => {
                        version = Some(GameVersion::EarlyFGP);
                        set_variable("Game Version", "Early FGP")
                    }
                    24850432 | 111476736 => {
                        version = Some(GameVersion::FullGoldPatch);
                        set_variable("Game Version", "Full Gold Patch")
                    }
                    1462272 | 111513600 => {
                        version = Some(GameVersion::MapUpdate);
                        set_variable("Game Version", "Map Update")
                    }
                    32768 | 246767616 => {
                        version = Some(GameVersion::GameJam);
                        set_variable("Game Version", "Game Jam / Post Jam Update")
                    }
                    _ => {
                        version = None;
                        set_variable("Game Version", "Error - Unknown");
                        set_variable_int("Module Size", size)
                    }
                }
            }

            if let Some(version) = version {
                let module = retry(|| Module::attach(
                    &process,
                    Version::V5_1,
                    process.get_module_address(process_name).unwrap(),
                )).await;
                let offsets = get_offsets(&version);

                let mut split_states: [i32; 32] = [0; 32];
                let mut goatlings_talked: [bool; 19] = [false; 19];
                let mut just_started: bool = true;

                let mut watch_bailey_key: Watcher<bool> = Watcher::new();
                let mut watch_underbelly_key: Watcher<bool> = Watcher::new();
                let mut watch_tower_key: Watcher<bool> = Watcher::new();
                let mut watch_keep_key: Watcher<bool> = Watcher::new();
                let mut watch_theatre_key: Watcher<bool> = Watcher::new();
                let mut dream_breaker: i32 = 0;
                let mut cling: i32 = 0;
                let mut sun_greaves: i32 = 0;
                let mut slide: i32 = 0;
                let mut ascendant_light: i32 = 0;
                let mut solar_wind: i32 = 0;
                let mut sunsetter: i32 = 0;
                let mut soul_cutter: i32 = 0;
                let mut indignation: i32 = 0;
                let mut strikebreak: i32 = 0;
                let mut heliiacal_power: i32 = 0;
                let mut memento: i32 = 0;
                let mut aerial_finesse: i32 = 0;
                let mut pilgrimage: i32 = 0;
                let mut empathy: i32 = 0;
                let mut good_graces: i32 = 0;
                let mut martial_prowess: i32 = 0;
                let mut clear_mind: i32 = 0;
                let mut outfit_professionalism: i32 = 0;
                let mut outfit_guardian: i32 = 0;
                let mut outfit_chivalry: i32 = 0;
                let mut outfit_bleeding_heart: i32 = 0;
                let mut outfit_nostalgia: i32 = 0;
                let mut outfit_devotion: i32 = 0;
                let mut outfit_class: i32 = 0;
                let mut outfit_sweater: i32 = 0;
                let mut watch_current_silver_keys: Watcher<i32> = Watcher::new();
                let mut total_silver_keys: i32 = 0;
                let mut health_upgrade_count: i32 = 0;
                let mut watch_fguid: Watcher<u64> = Watcher::new();
                let mut watch_final_boss_hp: Watcher<f64> = Watcher::new();
                let mut watch_boss_phase: Watcher<i32> = Watcher::new();
                let mut watch_area_name: Watcher<ArrayString<20>> = Watcher::new();
                let mut watch_current_outfit: Watcher<ArrayString<7>> = Watcher::new();
                let mut watch_jam_final: Watcher<bool> = Watcher::new();
                let mut goatlings_currently_talking: [Watcher<bool>; 19] = [Watcher::new(); 19];

                watch_bailey_key.update_infallible(false);
                watch_underbelly_key.update_infallible(false);
                watch_tower_key.update_infallible(false);
                watch_keep_key.update_infallible(false);
                watch_theatre_key.update_infallible(false);
                watch_fguid.update_infallible(0);
                watch_final_boss_hp.update_infallible(1500f64);
                watch_boss_phase.update_infallible(0);
                watch_area_name.update_infallible(ArrayString::new());
                watch_current_outfit.update_infallible(costume_base);
                watch_jam_final.update_infallible(false);
                for goatling in goatlings_currently_talking.iter_mut() {
                    goatling.update_infallible(false);
                    goatling.update_infallible(false);
                }

                print_message("Start Loop");
                loop {
                    set_variable_int("World", module.g_world().value());
                    settings.update();
                    if version == GameVersion::GameJam {
                        if let Ok(flag) = process.read_pointer_path::<u8>(
                            module.g_world(),
                            Bit64,
                            &offsets.jam_door
                        ) {
                            watch_jam_final.update_infallible(flag != 0);
                        } else {
                            watch_jam_final.update_infallible(false);
                        }
                        if let Some(jam_final) = watch_jam_final.pair {
                            if jam_final.current
                            {
                                set_variable("Game Jam Final", "True")
                            } else {
                                set_variable("Game Jam Final", "False")
                            }
                        } else {
                            set_variable("Game Jam Final", "False")
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<bool>(
                        module.g_world(),
                        Bit64,
                        &offsets.big_key_bailey,
                    ) {
                        watch_bailey_key.update_infallible(flag);
                        if flag {
                            set_variable("Bailey Key", "True")
                        } else {
                            set_variable("Bailey Key", "False")
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<bool>(
                        module.g_world(),
                        Bit64,
                        &offsets.big_key_underbelly,
                    ) {
                        watch_underbelly_key.update_infallible(flag);
                        if flag {
                            set_variable("Underbelly Key", "True")
                        } else {
                            set_variable("Underbelly Key", "False")
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<bool>(
                        module.g_world(),
                        Bit64,
                        &offsets.big_key_tower,
                    ) {
                        watch_tower_key.update_infallible(flag);
                        if flag {
                            set_variable("Tower Key", "True")
                        } else {
                            set_variable("Tower Key", "False")
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<bool>(
                        module.g_world(),
                        Bit64,
                        &offsets.big_key_keep,
                    ) {
                        watch_keep_key.update_infallible(flag);
                        if flag {
                            set_variable("Keep Key", "True")
                        } else {
                            set_variable("Keep Key", "False")
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<bool>(
                        module.g_world(),
                        Bit64,
                        &offsets.big_key_theatre,
                    ) {
                        watch_theatre_key.update_infallible(flag);
                        if flag {
                            set_variable("Theatre Key", "True")
                        } else {
                            set_variable("Theatre Key", "False")
                        }
                    }
                    {
                        let mut off: [u64; 4] = [
                            offsets.upgrade_tracker[0],
                            offsets.upgrade_tracker[1],
                            offsets.upgrade_tracker[2],
                            0,
                        ];
                        while off[3] < 0x208 {
                            if let Ok(f_name_key) = process.read_pointer_path::<FNameKey>(
                                module.g_world(),
                                Bit64,
                                &off,
                            ) {
                                if let Ok(f_name) = module.get_fname::<13>(&process, f_name_key)
                                {
                                    if let Ok(f_name) = f_name.validate_utf8() {
                                        off[3] = off[3] + 8;
                                        if let Ok(flag) = process.read_pointer_path::<i32>(
                                            module.g_world(),
                                            Bit64,
                                            &off,
                                        ) {
                                            match f_name {
                                                "attack" => {
                                                    dream_breaker = flag;
                                                    set_variable_int("attack", flag);
                                                }
                                                "WallRide" | "wallRide" => {
                                                    cling = flag;
                                                    set_variable_int("wallRide", flag);
                                                }
                                                "airKick" => {
                                                    sun_greaves = flag;
                                                    set_variable_int("airKick", flag);
                                                }
                                                "Slide" | "slide" => {
                                                    slide = flag;
                                                    set_variable_int("slide", flag);
                                                }
                                                "Light" => {
                                                    ascendant_light = flag;
                                                    set_variable_int("Light", flag);
                                                }
                                                "SlideJump" => {
                                                    solar_wind = flag;
                                                    set_variable_int("SlideJump", flag);
                                                }
                                                "plunge" => {
                                                    sunsetter = flag;
                                                    set_variable_int("plunge", flag);
                                                }
                                                "Projectile" | "projectile" => {
                                                    soul_cutter = flag;
                                                    set_variable_int("projectile", flag);
                                                }
                                                "PowerBoost" | "powerBoost" => {
                                                    indignation = flag;
                                                    set_variable_int("powerBoost", flag);
                                                }
                                                "chargeAttack" => {
                                                    strikebreak = flag;
                                                    set_variable_int("chargeAttack", flag);
                                                }
                                                "extraKick" => {
                                                    heliiacal_power = flag;
                                                    set_variable_int("extraKick", flag);
                                                }
                                                "Map" => {
                                                    memento = flag;
                                                    set_variable_int("Map", flag);
                                                }
                                                "airRecovery" => {
                                                    aerial_finesse = flag;
                                                    set_variable_int("airRecovery", flag);
                                                }
                                                "mobileHeal" => {
                                                    pilgrimage = flag;
                                                    set_variable_int("mobileHeal", flag);
                                                }
                                                "magicHaste" => {
                                                    empathy = flag;
                                                    set_variable_int("magicHaste", flag);
                                                }
                                                "HealBoost" => {
                                                    good_graces = flag;
                                                    set_variable_int("HealBoost", flag);
                                                }
                                                "damageBoost" => {
                                                    martial_prowess = flag;
                                                    set_variable_int("damageBoost", flag);
                                                }
                                                "magicPiece" => {
                                                    clear_mind = flag;
                                                    set_variable_int("magicPiece", flag);
                                                }
                                                "outfitPro" => {
                                                    outfit_professionalism = flag;
                                                    set_variable_int("outfitPro", flag);
                                                }
                                                "outfitShoujo" => {
                                                    outfit_guardian = flag;
                                                    set_variable_int("outfitShoujo", flag);
                                                }
                                                "outfitKnight" => {
                                                    outfit_chivalry = flag;
                                                    set_variable_int("outfitKnight", flag);
                                                }
                                                "outfitPast" => {
                                                    outfit_bleeding_heart
                                                      = flag;
                                                    set_variable_int("outfitPast", flag);
                                                }
                                                "outfitJam" => {
                                                    outfit_nostalgia = flag;
                                                    set_variable_int("outfitJam", flag);
                                                }
                                                "outfitFaith" => {
                                                    outfit_devotion = flag;
                                                    set_variable_int("outfitFaith", flag);
                                                }
                                                "outfitClassy" => {
                                                    outfit_class = flag;
                                                    set_variable_int("outfitClassy", flag);
                                                }
                                                "outfitSweater" => {
                                                    outfit_sweater = flag;
                                                    set_variable_int("outfitSweater", flag);
                                                }
                                                _ => {}
                                            }
                                        }
                                    } else { off[3] = off[3] + 8; }
                                } else { off[3] = off[3] + 8; }
                            } else { off[3] = off[3] + 8; }
                            off[3] = off[3] + 12;
                        }
                    }
                    if let Ok(flag) = process.read_pointer_path::<i32>(
                        module.g_world(),
                        Bit64,
                        &offsets.boss_phase,
                    ) {
                        watch_boss_phase.update_infallible(flag);
                        set_variable_int("Boss Phase", flag);
                    }
                    if let Ok(flag) = process.read_pointer_path::<i32>(
                        module.g_world(),
                        Bit64,
                        &offsets.silver_keys,
                    ) {
                        watch_current_silver_keys.update_infallible(flag);
                        if let Some(silver_keys) = watch_current_silver_keys.pair {
                            if silver_keys.increased() {
                                total_silver_keys = total_silver_keys + 1;
                            }
                        }
                        set_variable_int("Current Keys", flag);
                        set_variable_int("Total Keys", total_silver_keys);
                    }
                    if let Ok(flag) = process.read_pointer_path::<i32>(
                        module.g_world(),
                        Bit64,
                        &offsets.health_upgrade_count,
                    ) {
                        health_upgrade_count = flag;
                        set_variable_int("Health Upgrades", flag);
                    }
                    if let Ok(flag) = process.read_pointer_path::<u64>(
                        module.g_world(),
                        Bit64,
                        &offsets.fguid,
                    ) {
                        watch_fguid.update_infallible(flag);
                        set_variable_int("FGUID", flag);
                    }
                    if let Ok(flag) = process.read_pointer_path::<f64>(
                        module.g_world(),
                        Bit64,
                        &offsets.final_boss_hp,
                    ) {
                        watch_final_boss_hp.update_infallible(flag);
                        set_variable_float("Boss HP", flag);
                    }
                    if let Ok(f_name_key) = process.read_pointer_path::<FNameKey>(
                        module.g_world(),
                        Bit64,
                        &offsets.area_name,
                    ) {
                        if let Ok(f_name) = module.get_fname::<20>(&process, f_name_key) {
                            if let Ok(f_name) = f_name.validate_utf8() {
                                let area_name: ArrayString<20>;
                                match f_name {
                                    "ZONE_Dungeon" => {
                                        area_name = area_dungeon;
                                        update_goatlings(&process, &module, &offsets.goatlings_dungeon, &mut goatlings_currently_talking, 0);
                                    }
                                    "ZONE_LowerCastle" => {
                                        area_name = area_castle;
                                        update_goatlings(&process, &module, &offsets.goatlings_castle, &mut goatlings_currently_talking, 5);
                                    }
                                    "Zone_Library" => { area_name = area_library }
                                    "ZONE_Exterior" => {
                                        area_name = area_bailey;
                                        update_goatlings(&process, &module, &offsets.goatlings_bailey, &mut goatlings_currently_talking, 18);
                                    }
                                    "Zone_Upper" => {
                                        area_name = area_keep;
                                        update_goatlings(&process, &module, &offsets.goatlings_keep, &mut goatlings_currently_talking, 11);
                                    }
                                    "Zone_Caves" => { area_name = area_underbelly }
                                    "Zone_Theatre" => {
                                        area_name = area_theatre;
                                        update_goatlings(&process, &module, &offsets.goatlings_theatre, &mut goatlings_currently_talking, 13);
                                    }
                                    "Zone_Tower" => { area_name = area_tower }
                                    "Zone_PrincessChamber" => { area_name = area_princess }
                                    "TitleScreen" => { area_name = area_title }
                                    _ => { area_name = ArrayString::default() }
                                }
                                if area_name != ArrayString::default() {
                                    watch_area_name.update_infallible(area_name);
                                    if let Some(watch_area_name) = watch_area_name.pair {
                                        if watch_area_name.changed() {
                                            set_variable("Old Area Name", watch_area_name.old.as_str());
                                            set_variable("Area Name", area_name.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Ok(f_name_key) = process.read_pointer_path::<FNameKey>(
                        module.g_world(),
                        Bit64,
                        &offsets.current_outfit,
                    ) {
                        if let Ok(f_name) = module.get_fname::<7>(&process, f_name_key) {
                            if let Ok(f_name) = f_name.validate_utf8() {
                                match f_name {
                                    "Base" => {
                                        watch_current_outfit.update_infallible(costume_base);
                                        set_variable("Current Outfit", "Base")
                                    }
                                    "glove" => {
                                        watch_current_outfit.update_infallible(costume_glove);
                                        set_variable("Current Outfit", "glove")
                                    }
                                    "greaves" => {
                                        watch_current_outfit.update_infallible(costume_greaves);
                                        set_variable("Current Outfit", "greaves")
                                    }
                                    "nun" => {
                                        watch_current_outfit
                                          .update_infallible(costume_nun);
                                        set_variable("Current Outfit", "nun")
                                    }
                                    "pro" => {
                                        watch_current_outfit.update_infallible(costume_pro);
                                        set_variable("Current Outfit", "pro")
                                    }
                                    "shoujo" => {
                                        watch_current_outfit
                                          .update_infallible(costume_shoujo);
                                        set_variable("Current Outfit", "shoujo")
                                    }
                                    "knight" => {
                                        watch_current_outfit
                                          .update_infallible(costume_knight);
                                        set_variable("Current Outfit", "knight")
                                    }
                                    "past" => {
                                        watch_current_outfit
                                          .update_infallible(costume_past);
                                        set_variable("Current Outfit", "past")
                                    }
                                    "jam" => {
                                        watch_current_outfit
                                          .update_infallible(costume_jam);
                                        set_variable("Current Outfit", "jam")
                                    }
                                    "Class" => {
                                        watch_current_outfit
                                          .update_infallible(costume_class);
                                        set_variable("Current Outfit", "Class")
                                    }
                                    "sweater" => {
                                        watch_current_outfit
                                          .update_infallible(costume_sweater);
                                        set_variable("Current Outfit", "sweater")
                                    }
                                    "pants" => {
                                        watch_current_outfit.update_infallible(costume_pants);
                                        set_variable("Current Outfit", "pants")
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    match state() {
                        TimerState::NotRunning => {
                            total_silver_keys = 0;
                            just_started = true;
                            if let Some(fguid) = watch_fguid.pair {
                                if settings.start
                                  && fguid.changed_from(&5185712904977434514)
                                {
                                    start();
                                    print_message("Run Started");
                                    split_states.fill(0);
                                    dream_breaker = 0;
                                    set_variable_int("attack", 0);
                                    cling = 0;
                                    set_variable_int("wallRide", 0);
                                    sun_greaves = 0;
                                    set_variable_int("airKick", 0);
                                    slide = 0;
                                    set_variable_int("slide", 0);
                                    ascendant_light = 0;
                                    set_variable_int("Light", 0);
                                    solar_wind = 0;
                                    set_variable_int("SlideJump", 0);
                                    sunsetter = 0;
                                    set_variable_int("plunge", 0);
                                    soul_cutter = 0;
                                    set_variable_int("projectile", 0);
                                    indignation = 0;
                                    set_variable_int("powerBoost", 0);
                                    strikebreak = 0;
                                    set_variable_int("chargeAttack", 0);
                                    heliiacal_power = 0;
                                    set_variable_int("extraKick", 0);
                                    memento = 0;
                                    set_variable_int("Map", 0);
                                    aerial_finesse = 0;
                                    set_variable_int("airRecovery", 0);
                                    pilgrimage = 0;
                                    set_variable_int("mobileHeal", 0);
                                    empathy = 0;
                                    set_variable_int("magicHaste", 0);
                                    good_graces = 0;
                                    set_variable_int("HealBoost", 0);
                                    martial_prowess = 0;
                                    set_variable_int("damageBoost", 0);
                                    clear_mind = 0;
                                    set_variable_int("magicPiece", 0);
                                    outfit_professionalism = 0;
                                    set_variable_int("outfitPro", 0);
                                    outfit_guardian = 0;
                                    set_variable_int("outfitShoujo", 0);
                                    outfit_chivalry = 0;
                                    set_variable_int("outfitKnight", 0);
                                    outfit_bleeding_heart = 0;
                                    set_variable_int("outfitPast", 0);
                                    outfit_nostalgia = 0;
                                    set_variable_int("outfitJam", 0);
                                    outfit_devotion = 0;
                                    set_variable_int("outfitFaith", 0);
                                    outfit_class = 0;
                                    set_variable_int("outfitClassy", 0);
                                    outfit_sweater = 0;
                                    set_variable_int("outfitSweater", 0);
                                    watch_current_outfit.update_infallible(costume_base);
                                    set_variable("Current Outfit", "Base");
                                    watch_jam_final.update_infallible(false);
                                    set_variable("Game Jam Final", "False");

                                    // Need to delay to prevent weirdness during start.
                                    next_tick().await;
                                    next_tick().await;
                                    next_tick().await;
                                }
                            }
                        }
                        TimerState::Paused | TimerState::Running => {
                            if just_started {
                                just_started = false;

                                split_states[ATTACK] = dream_breaker;
                                split_states[WALL_RIDE] = cling;
                                split_states[AIR_KICK] = sun_greaves;
                                split_states[SLIDE] = slide;
                                split_states[LIGHT] = ascendant_light;
                                split_states[SLIDE_JUMP] = solar_wind;
                                split_states[PLUNGE] = sunsetter;
                                split_states[PROJECTILE] = soul_cutter;
                                split_states[POWER_BOOST] = indignation;
                                split_states[CHARGE_ATTACK] = strikebreak;
                                split_states[EXTRA_KICK] = heliiacal_power;
                                split_states[MAP] = memento;
                                split_states[AIR_RECOVERY] = aerial_finesse;
                                split_states[MOBILE_HEAL] = pilgrimage;
                                split_states[MAGIC_HASTE] = empathy;
                                split_states[HEAL_BOOST] = good_graces;
                                split_states[DAMAGE_BOOST] = martial_prowess;
                                split_states[MAGIC_PIECE] = clear_mind;
                                split_states[OUTFIT_PRO] = outfit_professionalism;
                                split_states[OUTFIT_SHOUJO] = outfit_guardian;
                                split_states[OUTFIT_KNIGHT] = outfit_chivalry;
                                split_states[OUTFIT_PAST] = outfit_bleeding_heart;
                                split_states[OUTFIT_JAM] = outfit_nostalgia;
                                split_states[OUTFIT_FAITH] = outfit_devotion;
                                split_states[OUTFIT_CLASSY] = outfit_class;
                                split_states[OUTFIT_SWEATER] = outfit_sweater;
                                split_states[HEALTH_UPGRADES] = health_upgrade_count;
                                if let Some(bailey_key) = watch_bailey_key.pair {
                                    if bailey_key.current {
                                        split_states[BAILEY_KEY] = 1
                                    } else {
                                        split_states[BAILEY_KEY] = 0
                                    }
                                } else {
                                    split_states[BAILEY_KEY] = 0
                                }
                                if let Some(underbelly_key) = watch_underbelly_key.pair {
                                    if underbelly_key.current {
                                        split_states[UNDERBELLY_KEY] = 1
                                    } else {
                                        split_states[UNDERBELLY_KEY] = 0
                                    }
                                } else {
                                    split_states[UNDERBELLY_KEY] = 0
                                }
                                if let Some(tower_key) = watch_tower_key.pair {
                                    if tower_key.current {
                                        split_states[TOWER_KEY] = 1
                                    } else {
                                        split_states[TOWER_KEY] = 0
                                    }
                                } else {
                                    split_states[TOWER_KEY] = 0
                                }
                                if let Some(keep_key) = watch_keep_key.pair {
                                    if keep_key.current {
                                        split_states[KEEP_KEY] = 1
                                    } else {
                                        split_states[KEEP_KEY] = 0
                                    }
                                } else {
                                    split_states[KEEP_KEY] = 0
                                }
                                if let Some(theatre_key) = watch_theatre_key.pair {
                                    if theatre_key.current {
                                        split_states[THEATRE_KEY] = 1
                                    } else {
                                        split_states[THEATRE_KEY] = 0
                                    }
                                } else {
                                    split_states[THEATRE_KEY] = 0
                                }
                                goatlings_talked = [false; 19];
                                if let Some(current_outfit) = watch_current_outfit.pair {
                                    watch_current_outfit
                                      .update_infallible(current_outfit.current);
                                }
                            }
                            if let Some(fguid) = watch_fguid.pair {
                                if settings.reset && fguid.changed_to(&5185712904977434514)
                                {
                                    print_message("Run Reset");
                                    reset();
                                    continue
                                }
                            }
                            if settings.split {
                                if settings.bailey_key
                                  && split_states[BAILEY_KEY] == 0
                                {
                                    if let Some(bailey_key) = watch_bailey_key.pair {
                                        if bailey_key.changed_to(&true) {
                                            print_message("Split: Bailey Key Pickup");
                                            split_states[BAILEY_KEY] = 1;
                                            split()
                                        }
                                    }
                                }
                                if settings.underbelly_key
                                  && split_states[UNDERBELLY_KEY] == 0
                                {
                                    if let Some(underbelly_key) = watch_underbelly_key.pair {
                                        if underbelly_key.changed_to(&true) {
                                            print_message("Split: Underbelly Key Pickup");
                                            split_states[UNDERBELLY_KEY] = 1;
                                            split()
                                        }
                                    }
                                }
                                if settings.tower_key
                                  && split_states[TOWER_KEY] == 0
                                {
                                    if let Some(tower_key) = watch_tower_key.pair {
                                        if tower_key.changed_to(&true) {
                                            print_message("Split: Tower Key Pickup");
                                            split_states[TOWER_KEY] = 1;
                                            split()
                                        }
                                    }
                                }
                                if settings.keep_key
                                  && split_states[KEEP_KEY] == 0
                                {
                                    if let Some(keep_key) = watch_keep_key.pair {
                                        if keep_key.changed_to(&true) {
                                            print_message("Split: Keep Key Pickup");
                                            split_states[KEEP_KEY] = 1;
                                            split()
                                        }
                                    }
                                }
                                if settings.theatre_key
                                  && split_states[THEATRE_KEY] == 0
                                {
                                    if let Some(theatre_key) = watch_theatre_key.pair {
                                        if theatre_key.changed_to(&true) {
                                            print_message("Split: Theatre Key Pickup");
                                            split_states[THEATRE_KEY] = 1;
                                            split()
                                        }
                                    }
                                }
                                if number_in_range(dream_breaker - split_states[ATTACK], 1, 3)
                                {
                                    split_states[ATTACK] = dream_breaker;
                                    if settings.dream_breaker {
                                        print_message("Split: Dream Breaker Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(cling - split_states[WALL_RIDE], 1, 3)
                                {
                                    split_states[WALL_RIDE] = cling;
                                    if settings.cling {
                                        print_message("Split: Cling Pickup");
                                        split()
                                    }
                                }

                                if number_in_range(sun_greaves - split_states[AIR_KICK], 1, 3)
                                {
                                    split_states[AIR_KICK] = sun_greaves;
                                    if settings.sun_greaves {
                                        print_message("Split: Sun Greaves Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(slide - split_states[SLIDE], 1, 3)
                                {
                                    split_states[SLIDE] = slide;
                                    if settings.slide {
                                        print_message("Split: Slide Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(ascendant_light - split_states[LIGHT], 1, 3)
                                {
                                    split_states[LIGHT] = ascendant_light;
                                    if settings.ascendant_light {
                                        print_message("Split: Ascendant Light Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(solar_wind - split_states[SLIDE_JUMP], 1, 3)
                                {
                                    split_states[SLIDE_JUMP] = solar_wind;
                                    if settings.solar_wind {
                                        print_message("Split: Solar Wind Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(sunsetter - split_states[PLUNGE], 1, 3)
                                {
                                    split_states[PLUNGE] = sunsetter;
                                    if settings.sunsetter {
                                        print_message("Split: Sunsetter Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(soul_cutter - split_states[PROJECTILE], 1, 3)
                                {
                                    split_states[PROJECTILE] = soul_cutter;
                                    if settings.soul_cutter {
                                        print_message("Split: Soul Cutter Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(indignation - split_states[POWER_BOOST], 1, 3)
                                {
                                    split_states[POWER_BOOST] = indignation;
                                    if settings.indignation {
                                        print_message("Split: Indignation Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(strikebreak - split_states[CHARGE_ATTACK], 1, 3)
                                {
                                    split_states[CHARGE_ATTACK] = strikebreak;
                                    if settings.strikebreak {
                                        print_message("Split: Strikebreak Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(heliiacal_power - split_states[EXTRA_KICK], 1, 3)
                                {
                                    split_states[EXTRA_KICK] = heliiacal_power;
                                    if settings.heliiacal_power {
                                        print_message("Split: Heliiacal Power Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(memento - split_states[MAP], 1, 3)
                                {
                                    split_states[MAP] = memento;
                                    if settings.memento {
                                        print_message("Split: Memento Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(aerial_finesse - split_states[AIR_RECOVERY], 1, 3)
                                {
                                    split_states[AIR_RECOVERY] = aerial_finesse;
                                    if settings.aerial_finesse {
                                        print_message("Split: Aerial Finesse Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(pilgrimage - split_states[MOBILE_HEAL], 1, 3)
                                {
                                    split_states[MOBILE_HEAL] = pilgrimage;
                                    if settings.pilgrimage {
                                        print_message("Split: Pilgrimage Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(empathy - split_states[MAGIC_HASTE], 1, 3)
                                {
                                    split_states[MAGIC_HASTE] = empathy;
                                    if settings.empathy {
                                        match empathy {
                                            1 => print_message("Split: Empathy Pickup #1"),
                                            2 => print_message("Split: Empathy Pickup #2"),
                                            _ => print_message("Split: Empathy Pickup #Error [Out of Bounds]"),
                                        }
                                        split()
                                    }
                                }
                                if number_in_range(good_graces - split_states[HEAL_BOOST], 1, 3)
                                {
                                    split_states[HEAL_BOOST] = good_graces;
                                    if settings.good_graces {
                                        match good_graces {
                                            1 => print_message("Split: Good Graces Pickup #1"),
                                            2 => print_message("Split: Good Graces Pickup #2"),
                                            _ => print_message("Split: Good Graces Pickup #Error [Out of Bounds]"),
                                        }
                                        split()
                                    }
                                }
                                if number_in_range(martial_prowess - split_states[DAMAGE_BOOST], 1, 3)
                                {
                                    split_states[DAMAGE_BOOST] = martial_prowess;
                                    if settings.martial_prowess {
                                        print_message("Split: Martial Prowess Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(clear_mind - split_states[MAGIC_PIECE], 1, 3)
                                {
                                    split_states[MAGIC_PIECE] = clear_mind;
                                    if settings.clear_mind {
                                        match clear_mind {
                                            1 => print_message("Split: Clear Mind Pickup #1"),
                                            2 => print_message("Split: Clear Mind Pickup #2"),
                                            3 => print_message("Split: Clear Mind Pickup #3"),
                                            _ => print_message("Split: Clear Mind Pickup #Error [Out of Bounds]"),
                                        }
                                        split()
                                    }
                                }
                                if number_in_range(outfit_professionalism - split_states[OUTFIT_PRO], 1, 3)
                                {
                                    split_states[OUTFIT_PRO] = outfit_professionalism;
                                    if settings.professionalism {
                                        print_message("Split: Outfit Professionalism Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_guardian - split_states[OUTFIT_SHOUJO], 1, 3)
                                {
                                    split_states[OUTFIT_SHOUJO] = outfit_guardian;
                                    if settings.guardian {
                                        print_message("Split: Outfit Guardian Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_chivalry - split_states[OUTFIT_KNIGHT], 1, 3)
                                {
                                    split_states[OUTFIT_KNIGHT] = outfit_chivalry;
                                    if settings.chivalry {
                                        print_message("Split: Outfit Chivalry Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_bleeding_heart - split_states[OUTFIT_PAST], 1, 3)
                                {
                                    split_states[OUTFIT_PAST] = outfit_bleeding_heart;
                                    if settings.bleeding_heart {
                                        print_message("Split: Outfit Bleeding Heart Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_nostalgia - split_states[OUTFIT_JAM], 1, 3)
                                {
                                    split_states[OUTFIT_JAM] = outfit_nostalgia;
                                    if settings.nostalgia {
                                        print_message("Split: Outfit Nostalgia Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_devotion - split_states[OUTFIT_FAITH], 1, 3)
                                {
                                    split_states[OUTFIT_FAITH] = outfit_devotion;
                                    if settings.devotion {
                                        print_message("Split: Outfit Devotion Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_class - split_states[OUTFIT_CLASSY], 1, 3)
                                {
                                    split_states[OUTFIT_CLASSY] = outfit_class;
                                    if settings.class {
                                        print_message("Split: Outfit Class Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(outfit_sweater - split_states[OUTFIT_SWEATER], 1, 3)
                                {
                                    split_states[OUTFIT_SWEATER] = outfit_sweater;
                                    if settings.sweater {
                                        print_message("Split: Outfit Sweater Pickup");
                                        split()
                                    }
                                }
                                if number_in_range(health_upgrade_count - split_states[HEALTH_UPGRADES], 1, 3)
                                {
                                    split_states[HEALTH_UPGRADES] = health_upgrade_count;
                                    if settings.health_upgrades {
                                        match health_upgrade_count {
                                            1 => print_message("Split: Health Upgrade Pickup #1"),
                                            2 => print_message("Split: Health Upgrade Pickup #2"),
                                            3 => print_message("Split: Health Upgrade Pickup #3"),
                                            4 => print_message("Split: Health Upgrade Pickup #4"),
                                            5 => print_message("Split: Health Upgrade Pickup #5"),
                                            6 => print_message("Split: Health Upgrade Pickup #6"),
                                            7 => print_message("Split: Health Upgrade Pickup #7"),
                                            8 => print_message("Split: Health Upgrade Pickup #8"),
                                            9 => print_message("Split: Health Upgrade Pickup #9"),
                                            10 => print_message("Split: Health Upgrade Pickup #10"),
                                            11 => print_message("Split: Health Upgrade Pickup #11"),
                                            12 => print_message("Split: Health Upgrade Pickup #12"),
                                            13 => print_message("Split: Health Upgrade Pickup #13"),
                                            14 => print_message("Split: Health Upgrade Pickup #14"),
                                            15 => print_message("Split: Health Upgrade Pickup #15"),
                                            16 => print_message("Split: Health Upgrade Pickup #16"),
                                            _ => print_message("Split: Health Upgrade Pickup #Error [Out of Bounds]"),
                                        }
                                        split()
                                    }
                                }
                                if let Some(area_name) = watch_area_name.pair {
                                    if area_name.changed() {
                                        if settings.all_transitions && area_name.old != area_title && area_name.current != area_title {
                                            print_message("Split: All Transitions:");
                                            print_message(area_name.old.as_str());
                                            print_message(area_name.current.as_str());
                                            split()
                                        } else {
                                            if settings.area_dungeon_castle
                                              && area_name.changed_from_to(&area_dungeon, &area_castle)
                                            {
                                                print_message("Split: Area Dungeon > Castle");
                                                split()
                                            }
                                            if settings.area_dungeon_underbelly
                                              && area_name.changed_from_to(&area_dungeon, &area_underbelly)
                                            {
                                                print_message("Split: Area Dungeon > Underbelly");
                                                split()
                                            }
                                            if settings.area_dungeon_theatre
                                              && area_name.changed_from_to(&area_dungeon, &area_theatre)
                                            {
                                                print_message("Split: Area Dungeon > Theatre");
                                                split()
                                            }

                                            if settings.area_castle_dungeon
                                              && area_name.changed_from_to(&area_castle, &area_dungeon)
                                            {
                                                print_message("Split: Area Castle > Dungeon");
                                                split()
                                            }
                                            if settings.area_castle_bailey
                                              && area_name.changed_from_to(&area_castle, &area_bailey)
                                            {
                                                print_message("Split: Area Castle > Bailey");
                                                split()
                                            }
                                            if settings.area_castle_keep
                                              && area_name.changed_from_to(&area_castle, &area_keep)
                                            {
                                                print_message("Split: Area Castle > Keep");
                                                split()
                                            }
                                            if settings.area_castle_library
                                              && area_name.changed_from_to(&area_castle, &area_library)
                                            {
                                                print_message("Split: Area Castle > Library");
                                                split()
                                            }
                                            if settings.area_castle_theatre
                                              && area_name.changed_from_to(&area_castle, &area_theatre)
                                            {
                                                print_message("Split: Area Castle > Theatre");
                                                split()
                                            }

                                            if settings.area_library_castle
                                              && area_name.changed_from_to(&area_library, &area_castle)
                                            {
                                                print_message("Split: Area Library > Castle");
                                                split()
                                            }

                                            if settings.area_bailey_castle
                                              && area_name.changed_from_to(&area_bailey, &area_castle)
                                            {
                                                print_message("Split: Area Bailey > Castle");
                                                split()
                                            }
                                            if settings.area_bailey_underbelly
                                              && area_name.changed_from_to(&area_bailey, &area_underbelly)
                                            {
                                                print_message("Split: Area Bailey > Underbelly");
                                                split()
                                            }
                                            if settings.area_bailey_tower
                                              && area_name.changed_from_to(&area_bailey, &area_tower)
                                            {
                                                print_message("Split: Area Bailey > Tower");
                                                split()
                                            }
                                            if settings.area_bailey_theatre
                                              && area_name.changed_from_to(&area_bailey, &area_theatre)
                                            {
                                                print_message("Split: Area Bailey > Theatre");
                                                split()
                                            }

                                            if settings.area_keep_castle
                                              && area_name.changed_from_to(&area_keep, &area_castle)
                                            {
                                                print_message("Split: Area Keep > Castle");
                                                split()
                                            }
                                            if settings.area_keep_underbelly
                                              && area_name.changed_from_to(&area_keep, &area_underbelly)
                                            {
                                                print_message("Split: Area Keep > Underbelly");
                                                split()
                                            }
                                            if settings.area_keep_theatre
                                              && area_name.changed_from_to(&area_keep, &area_theatre)
                                            {
                                                print_message("Split: Area Keep > Theatre");
                                                split()
                                            }

                                            if settings.area_underbelly_dungeon
                                              && area_name.changed_from_to(&area_underbelly, &area_dungeon)
                                            {
                                                print_message("Split: Area Underbelly > Dungeon");
                                                split()
                                            }
                                            if settings.area_underbelly_bailey
                                              && area_name.changed_from_to(&area_underbelly, &area_bailey)
                                            {
                                                print_message("Split: Area Underbelly > Bailey");
                                                split()
                                            }
                                            if settings.area_underbelly_keep
                                              && area_name.changed_from_to(&area_underbelly, &area_keep)
                                            {
                                                print_message("Split: Area Underbelly > Keep");
                                                split()
                                            }

                                            if settings.area_theatre_dungeon
                                              && area_name.changed_from_to(&area_theatre, &area_dungeon)
                                            {
                                                print_message("Split: Area Theatre > Dungeon");
                                                split()
                                            }
                                            if settings.area_theatre_castle
                                              && area_name.changed_from_to(&area_theatre, &area_castle)
                                            {
                                                print_message("Split: Area Theatre > Castle");
                                                split()
                                            }
                                            if settings.area_theatre_bailey
                                              && area_name.changed_from_to(&area_theatre, &area_bailey)
                                            {
                                                print_message("Split: Area Theatre > Bailey");
                                                split()
                                            }
                                            if settings.area_theatre_keep
                                              && area_name.changed_from_to(&area_theatre, &area_keep)
                                            {
                                                print_message("Split: Area Theatre > Keep");
                                                split()
                                            }

                                            if settings.area_tower_bailey
                                              && area_name.changed_from_to(&area_tower, &area_bailey)
                                            {
                                                print_message("Split: Area Tower > Bailey");
                                                split()
                                            }
                                            if settings.area_tower_boss
                                              && area_name.changed_from_to(&area_tower, &area_princess)
                                            {
                                                print_message("Split: Area Tower > Princess");
                                                split()
                                            }

                                            if settings.area_boss_tower
                                              && area_name.changed_from_to(&area_princess, &area_tower)
                                            {
                                                print_message("Split: Area Princess > Tower");
                                                split()
                                            }
                                        }
                                    }
                                    if area_name.current == area_princess {
                                        if let Some(hp) = watch_final_boss_hp.pair {
                                            if let Some(phase) = watch_boss_phase.pair {
                                                if hp.current <= 0f64 && phase.current == 1
                                                {
                                                    print_message("Split: Boss Defeated (Repeat)");
                                                    split()
                                                }
                                            }
                                        }
                                    }
                                    if area_name.current == area_tower {
                                        if let Some(final_door) = watch_jam_final.pair
                                        {
                                            if final_door.current {
                                                print_message("Split: Game Jam Final Door (Repeat)");
                                                split()
                                            }
                                        }
                                    }
                                }

                                if let Some(current_outfit) = watch_current_outfit.pair {
                                    if current_outfit.changed() {
                                        match current_outfit.current.as_str() {
                                            "Base" => if settings.equip_default {
                                                print_message("Split: Outfit Change > Basic Vest");
                                                split()
                                            }
                                            "glove" => if settings.equip_cling {
                                                print_message("Split: Outfit Change > Cling Sleeve");
                                                split()
                                            }
                                            "greaves" => if settings.equip_greaves {
                                                print_message("Split: Outfit Change > Sun Greaves");
                                                split()
                                            }
                                            "pro" => if settings.equip_professionalism {
                                                print_message("Split: Outfit Change > Professional");
                                                split()
                                            }
                                            "pants" => if settings.equip_pants {
                                                print_message("Split: Outfit Change > Big Pants");
                                                split()
                                            }
                                            "nun" => if settings.equip_devotion {
                                                print_message("Split: Outfit Change > Sol Sister");
                                                split()
                                            }
                                            "shoujo" => if settings.equip_guardian {
                                                print_message("Split: Outfit Change > Guardian");
                                                split()
                                            }
                                            "knight" => if settings.equip_chivalry {
                                                print_message("Split: Outfit Change > Soldier");
                                                split()
                                            }
                                            "past" => if settings.equip_bleeding_heart {
                                                print_message("Split: Outfit Change > Bleeding Heart");
                                                split()
                                            }
                                            "jam" => if settings.equip_nostalgia {
                                                print_message("Split: Outfit Change > XIX");
                                                split()
                                            }
                                            "Class" => if settings.equip_class {
                                                print_message("Split: Outfit Change > Classy");
                                                split()
                                            }
                                            "sweater" => if settings.equip_sweater {
                                                print_message("Split: Outfit Change > Sleepytime");
                                                split()
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                set_variable_int("Goatling", -1);
                                for (index, goatling) in goatlings_currently_talking.iter().enumerate() {
                                    if let Some(goatling) = goatling.pair {
                                        if goatling.current == true { set_variable_int("Goatling", index as u64) }
                                        if goatling.changed_to(&false) && !goatlings_talked[index] {
                                            goatlings_talked[index] = true;
                                            if settings.all_goatlings {
                                                print_message("Split: All Goatlings");
                                                split();
                                            } else {
                                                match index {
                                                    0 => {
                                                        if settings.repentant_goatling {
                                                            print_message("Split: Repentant Goatling (Dilapidated Dungeon)");
                                                            split()
                                                        }
                                                    }
                                                    1 => {
                                                        if settings.defeatist_goatling {
                                                            print_message("Split: Defeatist Goatling (Dilapidated Dungeon)");
                                                            split()
                                                        }
                                                    }
                                                    2 => {
                                                        if settings.rambling_goatling {
                                                            print_message("Split: Rambling Goatling (Dilapidated Dungeon)");
                                                            split()
                                                        }
                                                    }
                                                    3 => {
                                                        if settings.unwelcoming_goatling {
                                                            print_message("Split: Unwelcoming Goatling (Dilapidated Dungeon)");
                                                            split()
                                                        }
                                                    }
                                                    4 => {
                                                        if settings.mirror_room_goatling {
                                                            print_message("Split: Mirror Room Goatling (Dilapidated Dungeon)");
                                                            split()
                                                        }
                                                    }
                                                    5 => {
                                                        if settings.bubblephobic_goatling {
                                                            print_message("Split: Bubblephobic Goatling (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    6 => {
                                                        if settings.crystal_licker_goatling {
                                                            print_message("Split: Crystal Licker Goatling (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    7 => {
                                                        if settings.gazebo_goatling {
                                                            print_message("Split: Gazebo Goatling (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    8 => {
                                                        if settings.trapped_goatling {
                                                            print_message("Split: Trapped Goatling (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    9 => {
                                                        if settings.memento_goatling {
                                                            print_message("Split: Memento Goatling (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    10 => {
                                                        if settings.goatling_near_library {
                                                            print_message("Split: Goatling near Library (Castle Sansa)");
                                                            split()
                                                        }
                                                    }
                                                    11 => {
                                                        if settings.furnitureless_goatling {
                                                            print_message("Split: Furniture-less Goatling (Sansa Keep)");
                                                            split()
                                                        }
                                                    }
                                                    12 => {
                                                        if settings.distorted_goatling {
                                                            print_message("Split: Distorted Goatling (Sansa Keep)");
                                                            split()
                                                        }
                                                    }
                                                    13 => {
                                                        if settings.murderous_goatling {
                                                            print_message("Split: Murderous Goatling (Twilight Theatre)");
                                                            split()
                                                        }
                                                    }
                                                    14 => {
                                                        if settings.bean_casserole_goatling {
                                                            print_message("Split: Bean Casserole Goatling (Twilight Theatre)");
                                                            split()
                                                        }
                                                    }
                                                    15 => {
                                                        if settings.theatre_goer_goatling_1 {
                                                            print_message("Split: Theatre Goer Goatling 1 (Twilight Theatre)");
                                                            split()
                                                        }
                                                    }
                                                    16 => {
                                                        if settings.theatre_goer_goatling_2 {
                                                            print_message("Split: Theatre Goer Goatling 2 (Twilight Theatre)");
                                                            split()
                                                        }
                                                    }
                                                    17 => {
                                                        if settings.theatre_manager_goatling {
                                                            print_message("Split: Theatre Manager Goatling (Twilight Theatre)");
                                                            split()
                                                        }
                                                    }
                                                    18 => {
                                                        if settings.alley_goatling {
                                                            print_message("Split: Alley Goatling (Empty Bailey)");
                                                            split()
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                        }
                    }
                    next_tick().await;
                }
            } else { next_tick().await; }
        })
          .await;
        print_message("Disconnected... Searching for new process...")
    }
}

fn number_in_range(input: i32, min: i32, max: i32) -> bool {
    min <= input && input <= max
}
fn update_goatlings(
    process: &Process,
    module: &Module,
    offsets: &[[u64; 5]],
    goatlings: &mut [Watcher<bool>],
    start_index: usize,
) {
    let mut index: usize = start_index;
    for offset in offsets {
        if let Ok(flag) = process.read_pointer_path::<u64>(module.g_world(), Bit64, offset) {
            goatlings[index].update_infallible(flag != 0);
            index = index + 1;
        }
    }
}
const ATTACK: usize = 0;
const WALL_RIDE: usize = 1;
const AIR_KICK: usize = 2;
const SLIDE: usize = 3;
const LIGHT: usize = 4;
const SLIDE_JUMP: usize = 5;
const PLUNGE: usize = 6;
const PROJECTILE: usize = 7;
const POWER_BOOST: usize = 8;
const CHARGE_ATTACK: usize = 9;
const EXTRA_KICK: usize = 10;
const MAP: usize = 11;
const AIR_RECOVERY: usize = 12;
const MOBILE_HEAL: usize = 13;
const MAGIC_HASTE: usize = 14;
const HEAL_BOOST: usize = 15;
const DAMAGE_BOOST: usize = 16;
const MAGIC_PIECE: usize = 17;
const OUTFIT_PRO: usize = 18;
const OUTFIT_SHOUJO: usize = 19;
const OUTFIT_KNIGHT: usize = 20;
const OUTFIT_PAST: usize = 21;
const OUTFIT_JAM: usize = 22;
const OUTFIT_FAITH: usize = 23;
const OUTFIT_CLASSY: usize = 24;
const OUTFIT_SWEATER: usize = 25;
const BAILEY_KEY: usize = 26;
const UNDERBELLY_KEY: usize = 27;
const TOWER_KEY: usize = 28;
const KEEP_KEY: usize = 29;
const THEATRE_KEY: usize = 30;
const HEALTH_UPGRADES: usize = 31;
