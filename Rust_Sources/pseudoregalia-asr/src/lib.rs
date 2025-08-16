#![no_std]

mod autosplitter_settings;
mod version_offsets;

use crate::version_offsets::{get_offsets, GameVersion};
use asr::{
    future::{next_tick, retry},
    game_engine::unreal::{FNameKey, Module, Version},
    print_message,
    settings::Gui,
    string::{ArrayString, ArrayWString},
    timer::{
        reset, set_variable, set_variable_float, set_variable_int, split, start, state, TimerState,
    },
    watcher::Watcher,
    PointerSize::Bit64,
    Process,
};

asr::async_main!(stable);
asr::panic_handler!();

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
    print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("pseudoregalia-Win64-Shipping.exe").await;
        process
            .until_closes(async {
                print_message("0");
                let module = Module::wait_attach(
                    &process,
                    Version::V5_1,
                    process
                        .get_module_address("pseudoregalia-Win64-Shipping.exe")
                        .unwrap(),
                )
                .await;
                let version: Option<GameVersion>;
                {
                    let size =
                        retry(|| process.get_module_size("pseudoregalia-Win64-Shipping.exe")).await;
                    match size {
                        24850432 => {
                            version = Some(GameVersion::FullGoldPatch);
                            set_variable("Game Version", "Full Gold Patch")
                        }
                        1462272 => {
                            version = Some(GameVersion::MapUpdate);
                            set_variable("Game Version", "Map Update")
                        }
                        _ => {
                            version = None;
                            set_variable("Game Version", "Error - Unknown")
                        }
                    }
                }
                if version.is_some() {
                    let version = version.unwrap();
                    let offsets = get_offsets(&version);

                    let mut split_states: [i32; 32] = [0; 32];
                    let mut cache_upgrades_f_name: [FNameKey; 38] = [FNameKey::default(); 38];
                    let mut just_started: bool = true;

                    let mut watch_bailey_key: Watcher<bool> = Watcher::new();
                    let mut watch_underbelly_key: Watcher<bool> = Watcher::new();
                    let mut watch_tower_key: Watcher<bool> = Watcher::new();
                    let mut watch_keep_key: Watcher<bool> = Watcher::new();
                    let mut watch_theatre_key: Watcher<bool> = Watcher::new();
                    let mut watch_dream_breaker: Watcher<i32> = Watcher::new();
                    let mut watch_cling: Watcher<i32> = Watcher::new();
                    let mut watch_sun_greaves: Watcher<i32> = Watcher::new();
                    let mut watch_slide: Watcher<i32> = Watcher::new();
                    let mut watch_ascendant_light: Watcher<i32> = Watcher::new();
                    let mut watch_solar_wind: Watcher<i32> = Watcher::new();
                    let mut watch_sunsetter: Watcher<i32> = Watcher::new();
                    let mut watch_soul_cutter: Watcher<i32> = Watcher::new();
                    let mut watch_indignation: Watcher<i32> = Watcher::new();
                    let mut watch_strikebreak: Watcher<i32> = Watcher::new();
                    let mut watch_heliiacal_power: Watcher<i32> = Watcher::new();
                    let mut watch_memento: Watcher<i32> = Watcher::new();
                    let mut watch_aerial_finesse: Watcher<i32> = Watcher::new();
                    let mut watch_pilgrimage: Watcher<i32> = Watcher::new();
                    let mut watch_empathy: Watcher<i32> = Watcher::new();
                    let mut watch_good_graces: Watcher<i32> = Watcher::new();
                    let mut watch_martial_prowess: Watcher<i32> = Watcher::new();
                    let mut watch_clear_mind: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_professionalism: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_guardian: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_chivalry: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_bleeding_heart: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_nostalgia: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_devotion: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_class: Watcher<i32> = Watcher::new();
                    let mut watch_outfit_sweater: Watcher<i32> = Watcher::new();
                    let mut watch_boss_phase: Watcher<i32> = Watcher::new();
                    let mut watch_current_silver_keys: Watcher<i32> = Watcher::new();
                    let mut watch_total_silver_keys: Watcher<i32> = Watcher::new();
                    let mut watch_health_upgrade_count: Watcher<i32> = Watcher::new();
                    let mut watch_fguid: Watcher<u64> = Watcher::new();
                    let mut watch_final_boss_hp: Watcher<f64> = Watcher::new();
                    let mut watch_area_name: Watcher<ArrayString<20>> = Watcher::new();
                    let mut watch_current_outfit: Watcher<ArrayString<7>> = Watcher::new();

                    watch_bailey_key.update_infallible(false);
                    watch_underbelly_key.update_infallible(false);
                    watch_tower_key.update_infallible(false);
                    watch_keep_key.update_infallible(false);
                    watch_theatre_key.update_infallible(false);
                    watch_dream_breaker.update_infallible(0);
                    watch_cling.update_infallible(0);
                    watch_sun_greaves.update_infallible(0);
                    watch_slide.update_infallible(0);
                    watch_ascendant_light.update_infallible(0);
                    watch_solar_wind.update_infallible(0);
                    watch_sunsetter.update_infallible(0);
                    watch_soul_cutter.update_infallible(0);
                    watch_indignation.update_infallible(0);
                    watch_strikebreak.update_infallible(0);
                    watch_heliiacal_power.update_infallible(0);
                    watch_memento.update_infallible(0);
                    watch_aerial_finesse.update_infallible(0);
                    watch_pilgrimage.update_infallible(0);
                    watch_empathy.update_infallible(0);
                    watch_good_graces.update_infallible(0);
                    watch_martial_prowess.update_infallible(0);
                    watch_clear_mind.update_infallible(0);
                    watch_outfit_professionalism.update_infallible(0);
                    watch_outfit_guardian.update_infallible(0);
                    watch_outfit_chivalry.update_infallible(0);
                    watch_outfit_bleeding_heart.update_infallible(0);
                    watch_outfit_nostalgia.update_infallible(0);
                    watch_outfit_devotion.update_infallible(0);
                    watch_outfit_class.update_infallible(0);
                    watch_outfit_sweater.update_infallible(0);
                    watch_boss_phase.update_infallible(0);
                    watch_current_silver_keys.update_infallible(0);
                    watch_total_silver_keys.update_infallible(0);
                    watch_health_upgrade_count.update_infallible(0);
                    watch_fguid.update_infallible(0);
                    watch_final_boss_hp.update_infallible(1500f64);
                    watch_area_name.update_infallible(ArrayString::new());
                    watch_current_outfit.update_infallible(ArrayString::from(&"Base").unwrap());

                    print_message("1");
                    loop {
                        settings.update();
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
                                        let f_name = f_name.validate_utf8().unwrap_or_default();
                                        off[3] = off[3] + 8;
                                        match f_name {
                                            "attack" => cache_upgrades_f_name[ATTACK] = f_name_key,
                                            "wallRide" => {
                                                cache_upgrades_f_name[WALL_RIDE] = f_name_key
                                            }
                                            "airKick" => {
                                                cache_upgrades_f_name[AIR_KICK] = f_name_key
                                            }
                                            "slide" => cache_upgrades_f_name[SLIDE] = f_name_key,
                                            "Light" => cache_upgrades_f_name[LIGHT] = f_name_key,
                                            "SlideJump" => {
                                                cache_upgrades_f_name[SLIDE_JUMP] = f_name_key
                                            }
                                            "plunge" => cache_upgrades_f_name[PLUNGE] = f_name_key,
                                            "projectile" => {
                                                cache_upgrades_f_name[PROJECTILE] = f_name_key
                                            }
                                            "powerBoost" => {
                                                cache_upgrades_f_name[POWER_BOOST] = f_name_key
                                            }
                                            "chargeAttack" => {
                                                cache_upgrades_f_name[CHARGE_ATTACK] = f_name_key
                                            }
                                            "extraKick" => {
                                                cache_upgrades_f_name[EXTRA_KICK] = f_name_key
                                            }
                                            "Map" => cache_upgrades_f_name[MAP] = f_name_key,
                                            "airRecovery" => {
                                                cache_upgrades_f_name[AIR_RECOVERY] = f_name_key
                                            }
                                            "mobileHeal" => {
                                                cache_upgrades_f_name[MOBILE_HEAL] = f_name_key
                                            }
                                            "magicHaste" => {
                                                cache_upgrades_f_name[MAGIC_HASTE] = f_name_key
                                            }
                                            "HealBoost" => {
                                                cache_upgrades_f_name[HEAL_BOOST] = f_name_key
                                            }
                                            "damageBoost" => {
                                                cache_upgrades_f_name[DAMAGE_BOOST] = f_name_key
                                            }
                                            "magicPiece" => {
                                                cache_upgrades_f_name[MAGIC_PIECE] = f_name_key
                                            }
                                            "outfitPro" => {
                                                cache_upgrades_f_name[OUTFIT_PRO] = f_name_key
                                            }
                                            "outfitShoujo" => {
                                                cache_upgrades_f_name[OUTFIT_SHOUJO] = f_name_key
                                            }
                                            "outfitKnight" => {
                                                cache_upgrades_f_name[OUTFIT_KNIGHT] = f_name_key
                                            }
                                            "outfitPast" => {
                                                cache_upgrades_f_name[OUTFIT_PAST] = f_name_key
                                            }
                                            "outfitJam" => {
                                                cache_upgrades_f_name[OUTFIT_JAM] = f_name_key
                                            }
                                            "outfitFaith" => {
                                                cache_upgrades_f_name[OUTFIT_FAITH] = f_name_key
                                            }
                                            "outfitClassy" => {
                                                cache_upgrades_f_name[OUTFIT_CLASSY] = f_name_key
                                            }
                                            "outfitSweater" => {
                                                cache_upgrades_f_name[OUTFIT_SWEATER] = f_name_key
                                            }
                                            _ => {}
                                        }
                                    }
                                    if cache_upgrades_f_name.contains(&f_name_key) {
                                        if let Ok(flag) = process.read_pointer_path::<i32>(
                                            module.g_world(),
                                            Bit64,
                                            &off,
                                        ) {
                                            match cache_upgrades_f_name
                                                .iter()
                                                .position(|&r| r == f_name_key)
                                                .unwrap()
                                            {
                                                ATTACK => {
                                                    watch_dream_breaker.update_infallible(flag);
                                                    set_variable_int("attack", flag);
                                                }
                                                WALL_RIDE => {
                                                    watch_cling.update_infallible(flag);
                                                    set_variable_int("wallRide", flag);
                                                }
                                                AIR_KICK => {
                                                    watch_sun_greaves.update_infallible(flag);
                                                    set_variable_int("airKick", flag);
                                                }
                                                SLIDE => {
                                                    watch_slide.update_infallible(flag);
                                                    set_variable_int("slide", flag);
                                                }
                                                LIGHT => {
                                                    watch_ascendant_light.update_infallible(flag);
                                                    set_variable_int("Light", flag);
                                                }
                                                SLIDE_JUMP => {
                                                    watch_solar_wind.update_infallible(flag);
                                                    set_variable_int("SlideJump", flag);
                                                }
                                                PLUNGE => {
                                                    watch_sunsetter.update_infallible(flag);
                                                    set_variable_int("plunge", flag);
                                                }
                                                PROJECTILE => {
                                                    watch_soul_cutter.update_infallible(flag);
                                                    set_variable_int("projectile", flag);
                                                }
                                                POWER_BOOST => {
                                                    watch_indignation.update_infallible(flag);
                                                    set_variable_int("powerBoost", flag);
                                                }
                                                CHARGE_ATTACK => {
                                                    watch_strikebreak.update_infallible(flag);
                                                    set_variable_int("chargeAttack", flag);
                                                }
                                                EXTRA_KICK => {
                                                    watch_heliiacal_power.update_infallible(flag);
                                                    set_variable_int("extraKick", flag);
                                                }
                                                MAP => {
                                                    watch_memento.update_infallible(flag);
                                                    set_variable_int("Map", flag);
                                                }
                                                AIR_RECOVERY => {
                                                    watch_aerial_finesse.update_infallible(flag);
                                                    set_variable_int("airRecovery", flag);
                                                }
                                                MOBILE_HEAL => {
                                                    watch_pilgrimage.update_infallible(flag);
                                                    set_variable_int("mobileHeal", flag);
                                                }
                                                MAGIC_HASTE => {
                                                    watch_empathy.update_infallible(flag);
                                                    set_variable_int("magicHaste", flag);
                                                }
                                                HEAL_BOOST => {
                                                    watch_good_graces.update_infallible(flag);
                                                    set_variable_int("HealBoost", flag);
                                                }
                                                DAMAGE_BOOST => {
                                                    watch_martial_prowess.update_infallible(flag);
                                                    set_variable_int("damageBoost", flag);
                                                }
                                                MAGIC_PIECE => {
                                                    watch_clear_mind.update_infallible(flag);
                                                    set_variable_int("magicPiece", flag);
                                                }
                                                OUTFIT_PRO => {
                                                    watch_outfit_professionalism
                                                        .update_infallible(flag);
                                                    set_variable_int("outfitPro", flag);
                                                }
                                                OUTFIT_SHOUJO => {
                                                    watch_outfit_guardian.update_infallible(flag);
                                                    set_variable_int("outfitShoujo", flag);
                                                }
                                                OUTFIT_KNIGHT => {
                                                    watch_outfit_chivalry.update_infallible(flag);
                                                    set_variable_int("outfitKnight", flag);
                                                }
                                                OUTFIT_PAST => {
                                                    watch_outfit_bleeding_heart
                                                        .update_infallible(flag);
                                                    set_variable_int("outfitPast", flag);
                                                }
                                                OUTFIT_JAM => {
                                                    watch_outfit_nostalgia.update_infallible(flag);
                                                    set_variable_int("outfitJam", flag);
                                                }
                                                OUTFIT_FAITH => {
                                                    watch_outfit_devotion.update_infallible(flag);
                                                    set_variable_int("outfitFaith", flag);
                                                }
                                                OUTFIT_CLASSY => {
                                                    watch_outfit_class.update_infallible(flag);
                                                    set_variable_int("outfitClassy", flag);
                                                }
                                                OUTFIT_SWEATER => {
                                                    watch_outfit_sweater.update_infallible(flag);
                                                    set_variable_int("outfitSweater", flag);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
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
                            if watch_current_silver_keys.pair.unwrap().increased() {
                                watch_total_silver_keys.update_infallible(
                                    watch_total_silver_keys.pair.unwrap().current + 1,
                                );
                            }
                            set_variable_int("Current Keys", flag);
                            set_variable_int(
                                "Total Keys",
                                watch_total_silver_keys.pair.unwrap().current,
                            );
                        }
                        if let Ok(flag) = process.read_pointer_path::<i32>(
                            module.g_world(),
                            Bit64,
                            &offsets.health_upgrade_count,
                        ) {
                            watch_health_upgrade_count.update_infallible(flag);
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
                        if let Ok(flag) = process.read_pointer_path::<ArrayWString<20>>(
                            module.g_world(),
                            Bit64,
                            &offsets.area_name,
                        ) {
                            let s = wstring_to_string(flag);
                            watch_area_name.update_infallible(s);
                            set_variable("Area Name", s.as_str());
                        }
                        if let Ok(f_name_key) = process.read_pointer_path::<FNameKey>(
                            module.g_world(),
                            Bit64,
                            &offsets.current_outfit,
                        ) {
                            if let Ok(f_name) = module.get_fname::<7>(&process, f_name_key) {
                                let f_name = f_name.validate_utf8().unwrap_or_default();
                                match f_name {
                                    "Base" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_DEFAULT] = f_name_key
                                    }
                                    "glove" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_CLING] = f_name_key
                                    }
                                    "greaves" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_GREAVES] = f_name_key
                                    }
                                    "nun" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_DEVOTION] = f_name_key
                                    }
                                    "pro" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_PROFESSIONAL] =
                                            f_name_key
                                    }
                                    "shoujo" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_GUARDIAN] = f_name_key
                                    }
                                    "knight" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_CHIVALRY] = f_name_key
                                    }
                                    "past" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_BLEEDING_HEART] =
                                            f_name_key
                                    }
                                    "jam" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_NOSTALGIA] = f_name_key
                                    }
                                    "Class" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_CLASS] = f_name_key
                                    }
                                    "sweater" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_SWEATER] = f_name_key
                                    }
                                    "pants" => {
                                        cache_upgrades_f_name[EQUIP_OUTFIT_PANTS] = f_name_key
                                    }
                                    _ => {}
                                }
                                match cache_upgrades_f_name
                                    .iter()
                                    .position(|&r| r == f_name_key)
                                    .unwrap()
                                {
                                    EQUIP_OUTFIT_DEFAULT => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("Base").unwrap());
                                        set_variable("Current Outfit", "Base")
                                    }
                                    EQUIP_OUTFIT_CLING => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("glove").unwrap());
                                        set_variable("Current Outfit", "glove")
                                    }
                                    EQUIP_OUTFIT_GREAVES => {
                                        watch_current_outfit.update_infallible(
                                            ArrayString::from("greaves").unwrap(),
                                        );
                                        set_variable("Current Outfit", "greaves")
                                    }
                                    EQUIP_OUTFIT_PROFESSIONAL => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("pro").unwrap());
                                        set_variable("Current Outfit", "pro")
                                    }
                                    EQUIP_OUTFIT_PANTS => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("pants").unwrap());
                                        set_variable("Current Outfit", "pants")
                                    }
                                    EQUIP_OUTFIT_DEVOTION => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("nun").unwrap());
                                        set_variable("Current Outfit", "nun")
                                    }
                                    EQUIP_OUTFIT_GUARDIAN => {
                                        watch_current_outfit.update_infallible(
                                            ArrayString::from("shoujo").unwrap(),
                                        );
                                        set_variable("Current Outfit", "shoujo")
                                    }
                                    EQUIP_OUTFIT_CHIVALRY => {
                                        watch_current_outfit.update_infallible(
                                            ArrayString::from("knight").unwrap(),
                                        );
                                        set_variable("Current Outfit", "knight")
                                    }
                                    EQUIP_OUTFIT_BLEEDING_HEART => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("past").unwrap());
                                        set_variable("Current Outfit", "past")
                                    }
                                    EQUIP_OUTFIT_NOSTALGIA => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("jam").unwrap());
                                        set_variable("Current Outfit", "jam")
                                    }
                                    EQUIP_OUTFIT_CLASS => {
                                        watch_current_outfit
                                            .update_infallible(ArrayString::from("Class").unwrap());
                                        set_variable("Current Outfit", "Class")
                                    }
                                    EQUIP_OUTFIT_SWEATER => {
                                        watch_current_outfit.update_infallible(
                                            ArrayString::from("sweater").unwrap(),
                                        );
                                        set_variable("Current Outfit", "sweater")
                                    }
                                    _ => {}
                                }
                            }
                            if just_started {
                                watch_current_outfit
                                    .update_infallible(watch_current_outfit.pair.unwrap().current);
                            }
                        }
                        if just_started {
                            just_started = false;

                            split_states[ATTACK] = watch_dream_breaker.pair.unwrap().current;
                            split_states[WALL_RIDE] = watch_cling.pair.unwrap().current;
                            split_states[AIR_KICK] = watch_sun_greaves.pair.unwrap().current;
                            split_states[SLIDE] = watch_slide.pair.unwrap().current;
                            split_states[LIGHT] = watch_ascendant_light.pair.unwrap().current;
                            split_states[SLIDE_JUMP] = watch_solar_wind.pair.unwrap().current;
                            split_states[PLUNGE] = watch_sunsetter.pair.unwrap().current;
                            split_states[PROJECTILE] = watch_soul_cutter.pair.unwrap().current;
                            split_states[POWER_BOOST] = watch_indignation.pair.unwrap().current;
                            split_states[CHARGE_ATTACK] = watch_strikebreak.pair.unwrap().current;
                            split_states[EXTRA_KICK] = watch_heliiacal_power.pair.unwrap().current;
                            split_states[MAP] = watch_memento.pair.unwrap().current;
                            split_states[AIR_RECOVERY] = watch_aerial_finesse.pair.unwrap().current;
                            split_states[MOBILE_HEAL] = watch_pilgrimage.pair.unwrap().current;
                            split_states[MAGIC_HASTE] = watch_empathy.pair.unwrap().current;
                            split_states[HEAL_BOOST] = watch_good_graces.pair.unwrap().current;
                            split_states[DAMAGE_BOOST] =
                                watch_martial_prowess.pair.unwrap().current;
                            split_states[MAGIC_PIECE] = watch_clear_mind.pair.unwrap().current;
                            split_states[OUTFIT_PRO] =
                                watch_outfit_professionalism.pair.unwrap().current;
                            split_states[OUTFIT_SHOUJO] =
                                watch_outfit_guardian.pair.unwrap().current;
                            split_states[OUTFIT_KNIGHT] =
                                watch_outfit_chivalry.pair.unwrap().current;
                            split_states[OUTFIT_PAST] =
                                watch_outfit_bleeding_heart.pair.unwrap().current;
                            split_states[OUTFIT_JAM] = watch_outfit_nostalgia.pair.unwrap().current;
                            split_states[OUTFIT_FAITH] =
                                watch_outfit_devotion.pair.unwrap().current;
                            split_states[OUTFIT_CLASSY] = watch_outfit_class.pair.unwrap().current;
                            split_states[OUTFIT_SWEATER] =
                                watch_outfit_sweater.pair.unwrap().current;
                            if watch_bailey_key.pair.unwrap().current {
                                split_states[BAILEY_KEY] = 1
                            } else {
                                split_states[BAILEY_KEY] = 0
                            }
                            if watch_underbelly_key.pair.unwrap().current {
                                split_states[UNDERBELLY_KEY] = 1
                            } else {
                                split_states[UNDERBELLY_KEY] = 0
                            }
                            if watch_tower_key.pair.unwrap().current {
                                split_states[TOWER_KEY] = 1
                            } else {
                                split_states[TOWER_KEY] = 0
                            }
                            if watch_keep_key.pair.unwrap().current {
                                split_states[KEEP_KEY] = 1
                            } else {
                                split_states[KEEP_KEY] = 0
                            }
                            if watch_theatre_key.pair.unwrap().current {
                                split_states[THEATRE_KEY] = 1
                            } else {
                                split_states[THEATRE_KEY] = 0
                            }
                            split_states[HEALTH_UPGRADES] =
                                watch_health_upgrade_count.pair.unwrap().current;
                        }
                        match state() {
                            TimerState::NotRunning => {
                                watch_total_silver_keys.update_infallible(0);
                                watch_area_name.update_infallible(ArrayString::new());
                                if settings.start
                                    && watch_fguid.pair.unwrap().changed_from(&5185712904977434514)
                                {
                                    start();
                                    split_states.fill(0);
                                    watch_dream_breaker.update_infallible(0);
                                    set_variable_int("attack", 0);
                                    watch_cling.update_infallible(0);
                                    set_variable_int("wallRide", 0);
                                    watch_sun_greaves.update_infallible(0);
                                    set_variable_int("airKick", 0);
                                    watch_slide.update_infallible(0);
                                    set_variable_int("slide", 0);
                                    watch_ascendant_light.update_infallible(0);
                                    set_variable_int("Light", 0);
                                    watch_solar_wind.update_infallible(0);
                                    set_variable_int("SlideJump", 0);
                                    watch_sunsetter.update_infallible(0);
                                    set_variable_int("plunge", 0);
                                    watch_soul_cutter.update_infallible(0);
                                    set_variable_int("projectile", 0);
                                    watch_indignation.update_infallible(0);
                                    set_variable_int("powerBoost", 0);
                                    watch_strikebreak.update_infallible(0);
                                    set_variable_int("chargeAttack", 0);
                                    watch_heliiacal_power.update_infallible(0);
                                    set_variable_int("extraKick", 0);
                                    watch_memento.update_infallible(0);
                                    set_variable_int("Map", 0);
                                    watch_aerial_finesse.update_infallible(0);
                                    set_variable_int("airRecovery", 0);
                                    watch_pilgrimage.update_infallible(0);
                                    set_variable_int("mobileHeal", 0);
                                    watch_empathy.update_infallible(0);
                                    set_variable_int("magicHaste", 0);
                                    watch_good_graces.update_infallible(0);
                                    set_variable_int("HealBoost", 0);
                                    watch_martial_prowess.update_infallible(0);
                                    set_variable_int("damageBoost", 0);
                                    watch_clear_mind.update_infallible(0);
                                    set_variable_int("magicPiece", 0);
                                    watch_outfit_professionalism.update_infallible(0);
                                    set_variable_int("outfitPro", 0);
                                    watch_outfit_guardian.update_infallible(0);
                                    set_variable_int("outfitShoujo", 0);
                                    watch_outfit_chivalry.update_infallible(0);
                                    set_variable_int("outfitKnight", 0);
                                    watch_outfit_bleeding_heart.update_infallible(0);
                                    set_variable_int("outfitPast", 0);
                                    watch_outfit_nostalgia.update_infallible(0);
                                    set_variable_int("outfitJam", 0);
                                    watch_outfit_devotion.update_infallible(0);
                                    set_variable_int("outfitFaith", 0);
                                    watch_outfit_class.update_infallible(0);
                                    set_variable_int("outfitClassy", 0);
                                    watch_outfit_sweater.update_infallible(0);
                                    set_variable_int("outfitSweater", 0);
                                    watch_current_outfit
                                        .update_infallible(ArrayString::from(&"Base").unwrap());
                                    set_variable("Current Outfit", "Base");
                                    just_started = true;
                                    next_tick().await;
                                    next_tick().await;
                                    next_tick().await;
                                }
                            }
                            TimerState::Paused | TimerState::Running => {
                                if settings.reset
                                    && watch_fguid.pair.unwrap().changed_to(&5185712904977434514)
                                {
                                    reset()
                                } else if settings.split {
                                    if settings.bailey_key
                                        && watch_bailey_key.pair.unwrap().changed_to(&true)
                                        && split_states[BAILEY_KEY] == 0
                                    {
                                        split_states[BAILEY_KEY] = 1;
                                        split()
                                    }
                                    if settings.underbelly_key
                                        && watch_underbelly_key.pair.unwrap().changed_to(&true)
                                        && split_states[UNDERBELLY_KEY] == 0
                                    {
                                        split_states[UNDERBELLY_KEY] = 1;
                                        split()
                                    }
                                    if settings.tower_key
                                        && watch_tower_key.pair.unwrap().changed_to(&true)
                                        && split_states[TOWER_KEY] == 0
                                    {
                                        split_states[TOWER_KEY] = 1;
                                        split()
                                    }
                                    if settings.keep_key
                                        && watch_keep_key.pair.unwrap().changed_to(&true)
                                        && split_states[KEEP_KEY] == 0
                                    {
                                        split_states[KEEP_KEY] = 1;
                                        split()
                                    }
                                    if settings.theatre_key
                                        && watch_theatre_key.pair.unwrap().changed_to(&true)
                                        && split_states[THEATRE_KEY] == 0
                                    {
                                        split_states[THEATRE_KEY] = 1;
                                        split()
                                    }
                                    if settings.dream_breaker
                                        && watch_dream_breaker
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[ATTACK] + 1))
                                    {
                                        split_states[ATTACK] =
                                            watch_dream_breaker.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.cling
                                        && watch_cling
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[WALL_RIDE] + 1))
                                    {
                                        split_states[WALL_RIDE] = watch_cling.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.sun_greaves
                                        && watch_sun_greaves
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[AIR_KICK] + 1))
                                    {
                                        split_states[AIR_KICK] =
                                            watch_sun_greaves.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.slide
                                        && watch_slide
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[SLIDE] + 1))
                                    {
                                        split_states[SLIDE] = watch_slide.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.ascendant_light
                                        && watch_ascendant_light
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[LIGHT] + 1))
                                    {
                                        split_states[LIGHT] =
                                            watch_ascendant_light.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.solar_wind
                                        && watch_solar_wind
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[SLIDE_JUMP] + 1))
                                    {
                                        split_states[SLIDE_JUMP] =
                                            watch_solar_wind.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.sunsetter
                                        && watch_sunsetter
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[PLUNGE] + 1))
                                    {
                                        split_states[PLUNGE] =
                                            watch_sunsetter.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.soul_cutter
                                        && watch_soul_cutter
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[PROJECTILE] + 1))
                                    {
                                        split_states[PROJECTILE] =
                                            watch_soul_cutter.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.indignation
                                        && watch_indignation
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[POWER_BOOST] + 1))
                                    {
                                        split_states[POWER_BOOST] =
                                            watch_indignation.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.strikebreak
                                        && watch_strikebreak
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[CHARGE_ATTACK] + 1))
                                    {
                                        split_states[CHARGE_ATTACK] =
                                            watch_strikebreak.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.heliiacal_power
                                        && watch_heliiacal_power
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[EXTRA_KICK] + 1))
                                    {
                                        split_states[EXTRA_KICK] =
                                            watch_heliiacal_power.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.memento
                                        && watch_memento
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[MAP] + 1))
                                    {
                                        split_states[MAP] = watch_memento.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.aerial_finesse
                                        && watch_aerial_finesse
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[AIR_RECOVERY] + 1))
                                    {
                                        split_states[AIR_RECOVERY] =
                                            watch_aerial_finesse.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.pilgrimage
                                        && watch_pilgrimage
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[MOBILE_HEAL] + 1))
                                    {
                                        split_states[MOBILE_HEAL] =
                                            watch_pilgrimage.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.empathy
                                        && watch_empathy
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[MAGIC_HASTE] + 1))
                                    {
                                        split_states[MAGIC_HASTE] =
                                            watch_empathy.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.good_graces
                                        && watch_good_graces
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[HEAL_BOOST] + 1))
                                    {
                                        split_states[HEAL_BOOST] =
                                            watch_good_graces.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.martial_prowess
                                        && watch_martial_prowess
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[DAMAGE_BOOST] + 1))
                                    {
                                        split_states[DAMAGE_BOOST] =
                                            watch_martial_prowess.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.clear_mind
                                        && watch_clear_mind
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[MAGIC_PIECE] + 1))
                                    {
                                        split_states[MAGIC_PIECE] =
                                            watch_clear_mind.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.professionalism
                                        && watch_outfit_professionalism
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_PRO] + 1))
                                    {
                                        split_states[OUTFIT_PRO] =
                                            watch_outfit_professionalism.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.guardian
                                        && watch_outfit_guardian
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_SHOUJO] + 1))
                                    {
                                        split_states[OUTFIT_SHOUJO] =
                                            watch_outfit_guardian.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.chivalry
                                        && watch_outfit_chivalry
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_KNIGHT] + 1))
                                    {
                                        split_states[OUTFIT_KNIGHT] =
                                            watch_outfit_chivalry.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.bleeding_heart
                                        && watch_outfit_bleeding_heart
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_PAST] + 1))
                                    {
                                        split_states[OUTFIT_PAST] =
                                            watch_outfit_bleeding_heart.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.nostalgia
                                        && watch_outfit_nostalgia
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_JAM] + 1))
                                    {
                                        split_states[OUTFIT_JAM] =
                                            watch_outfit_nostalgia.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.devotion
                                        && watch_outfit_devotion
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_FAITH] + 1))
                                    {
                                        split_states[OUTFIT_FAITH] =
                                            watch_outfit_devotion.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.class
                                        && watch_outfit_class
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_CLASSY] + 1))
                                    {
                                        split_states[OUTFIT_CLASSY] =
                                            watch_outfit_class.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.sweater
                                        && watch_outfit_sweater
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[OUTFIT_SWEATER] + 1))
                                    {
                                        split_states[OUTFIT_SWEATER] =
                                            watch_outfit_sweater.pair.unwrap().current;
                                        split()
                                    }
                                    if settings.health_upgrades
                                        && watch_health_upgrade_count
                                            .pair
                                            .unwrap()
                                            .changed_to(&(split_states[HEALTH_UPGRADES] + 1))
                                    {
                                        split_states[HEALTH_UPGRADES] =
                                            watch_health_upgrade_count.pair.unwrap().current;
                                        split()
                                    }
                                    if watch_area_name.pair.unwrap().changed() {
                                        if settings.area_dungeon_castle
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_dungeon, &area_castle)
                                        {
                                            split()
                                        }
                                        if settings.area_dungeon_underbelly
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_dungeon, &area_underbelly)
                                        {
                                            split()
                                        }
                                        if settings.area_dungeon_theatre
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_dungeon, &area_theatre)
                                        {
                                            split()
                                        }

                                        if settings.area_castle_dungeon
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_castle, &area_dungeon)
                                        {
                                            split()
                                        }
                                        if settings.area_castle_bailey
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_castle, &area_bailey)
                                        {
                                            split()
                                        }
                                        if settings.area_castle_keep
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_castle, &area_keep)
                                        {
                                            split()
                                        }
                                        if settings.area_castle_library
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_castle, &area_library)
                                        {
                                            split()
                                        }
                                        if settings.area_castle_theatre
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_castle, &area_theatre)
                                        {
                                            split()
                                        }

                                        if settings.area_library_castle
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_library, &area_castle)
                                        {
                                            split()
                                        }

                                        if settings.area_bailey_castle
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_bailey, &area_castle)
                                        {
                                            split()
                                        }
                                        if settings.area_bailey_underbelly
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_bailey, &area_underbelly)
                                        {
                                            split()
                                        }
                                        if settings.area_bailey_tower
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_bailey, &area_tower)
                                        {
                                            split()
                                        }
                                        if settings.area_bailey_theatre
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_bailey, &area_theatre)
                                        {
                                            split()
                                        }

                                        if settings.area_keep_castle
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_keep, &area_castle)
                                        {
                                            split()
                                        }
                                        if settings.area_keep_underbelly
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_keep, &area_underbelly)
                                        {
                                            split()
                                        }
                                        if settings.area_keep_theatre
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_keep, &area_theatre)
                                        {
                                            split()
                                        }

                                        if settings.area_underbelly_dungeon
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_underbelly, &area_dungeon)
                                        {
                                            split()
                                        }
                                        if settings.area_underbelly_bailey
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_underbelly, &area_bailey)
                                        {
                                            split()
                                        }
                                        if settings.area_underbelly_keep
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_underbelly, &area_keep)
                                        {
                                            split()
                                        }

                                        if settings.area_theatre_dungeon
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_theatre, &area_dungeon)
                                        {
                                            split()
                                        }
                                        if settings.area_theatre_castle
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_theatre, &area_castle)
                                        {
                                            split()
                                        }
                                        if settings.area_theatre_bailey
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_theatre, &area_bailey)
                                        {
                                            split()
                                        }
                                        if settings.area_theatre_keep
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_theatre, &area_keep)
                                        {
                                            split()
                                        }

                                        if settings.area_tower_bailey
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_tower, &area_bailey)
                                        {
                                            split()
                                        }
                                        if settings.area_tower_boss
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_tower, &area_princess)
                                        {
                                            split()
                                        }

                                        if settings.area_boss_tower
                                            && watch_area_name
                                                .pair
                                                .unwrap()
                                                .changed_from_to(&area_princess, &area_tower)
                                        {
                                            split()
                                        }
                                    }
                                    if watch_area_name.pair.unwrap().current == area_princess
                                        && watch_final_boss_hp.pair.unwrap().current <= 0f64
                                        && watch_boss_phase.pair.unwrap().current == 1
                                    {
                                        split()
                                    }
                                    if watch_current_outfit.pair.unwrap().changed() {
                                        match watch_current_outfit.pair.clone().unwrap().current.as_str() {
                                            "Base" => if settings.equip_default {split()}
                                            "glove" => if settings.equip_cling {split()}
                                            "greaves" => if settings.equip_greaves {split()}
                                            "pro" => if settings.equip_professionalism {split()}
                                            "pants" => if settings.equip_pants {split()}
                                            "nun" => if settings.equip_devotion {split()}
                                            "shoujo" => if settings.equip_guardian {split()}
                                            "knight" => if settings.equip_chivalry {split()}
                                            "past" => if settings.equip_bleeding_heart {split()}
                                            "Class" => if settings.equip_class {split()}
                                            "sweater" => if settings.equip_sweater {split()}
                                            _=> {}
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        next_tick().await;
                    }
                }
            })
            .await;
    }
}
fn wstring_to_string<const N: usize>(input: ArrayWString<N>) -> ArrayString<N> {
    let mut i = 0;
    let mut out: ArrayString<N> = ArrayString::new();
    while i < input.len() {
        if let Ok(next) = core::str::from_utf8(&[(input[i] & 0xFF) as u8]) {
            out.push_str(next);
        }
        i = i + 1;
    }
    out
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
const EQUIP_OUTFIT_DEFAULT: usize = 26; // "Base"
const EQUIP_OUTFIT_CLING: usize = 27; // "glove"
const EQUIP_OUTFIT_GREAVES: usize = 28; // "greaves"
const EQUIP_OUTFIT_PROFESSIONAL: usize = 29; // "pro"
const EQUIP_OUTFIT_PANTS: usize = 30; // "Pants"
const EQUIP_OUTFIT_DEVOTION: usize = 31; // "nun"
const EQUIP_OUTFIT_GUARDIAN: usize = 32; // "shoujo"
const EQUIP_OUTFIT_CHIVALRY: usize = 33; // "knight"
const EQUIP_OUTFIT_BLEEDING_HEART: usize = 34; // "past"
const EQUIP_OUTFIT_NOSTALGIA: usize = 35; // "jam"
const EQUIP_OUTFIT_CLASS: usize = 36; // "Class"
const EQUIP_OUTFIT_SWEATER: usize = 37; // "sweater"
