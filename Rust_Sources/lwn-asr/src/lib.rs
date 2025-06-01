#![no_std]

use asr::{
    future::next_tick,
    game_engine::unity::il2cpp::{Class, Image, Module},
    print_message,
    settings::gui::{set_tooltip, Title},
    settings::Gui,
    string::{ArrayString, ArrayWString},
    time, timer,
    timer::TimerState,
    watcher::Watcher,
    PointerSize::Bit64,
    Process,
};

asr::async_main!(stable);
asr::panic_handler!();

#[derive(Gui)]
struct Settings {
  #[default = true] start: bool,
  #[default = true] split: bool,
  #[default = true] reset: bool,

  #[heading_level = 1] boss_splits: Title,
  #[default = false] armor_start: bool,
  #[default = true] armor_dead: bool,
  #[default = false] secret_armor_start: bool,
  #[default = true] secret_armor_dead: bool,
  #[default = false] tania_start: bool,
  #[default = true] tania_dead: bool,
  #[default = false] monica_1_start: bool,
  #[default = true] monica_1_dead: bool,
  #[default = false] monica_2_start: bool,
  #[default = true] monica_2_dead: bool,
  #[default = false] vanessa_1_start: bool,
  #[default = true] vanessa_1_dead: bool,
  #[default = false] vanessa_2_start: bool,
  #[default = true] vanessa_2_dead: bool,
  #[default = false] nonota_start: bool,
  #[default = true] nonota_dead: bool,

  #[heading_level = 3] mini_boss_splits: Title,
  #[default = true] knight_dead: bool,
  #[default = false] seal_1_start: bool,
  #[default = true] seal_1_dead: bool,
  #[default = false] seal_2_start: bool,
  #[default = true] seal_2_dead: bool,

  #[heading_level = 1] abyss_challenge_splits: Title,
  #[default = false] left_challenge: bool,
  #[default = false] right_challenge: bool,
  #[default = false] center_challenge: bool,

  #[heading_level = 1] cutscene_splits: Title,
  #[default = false] first_barrier_stone: bool,
  #[default = false] rescue_cat: bool,
  #[default = false] ice_trial: bool,
  #[default = false] destroy_wood_planks: bool,
  #[default = false] mid_lava_cutscene: bool,
  #[default = false] fire_trial: bool,
  #[default = false] hat_lost: bool,
  #[default = false] hat_get: bool,
  #[default = false] orb_after_get_hat: bool,
  #[default = false] thunder_trial: bool,
  #[default = false] end_of_dark_tunnel: bool,
  #[default = false] castle_4_barrier_stone: bool,
  #[default = false] open_abyss_door: bool,

  #[heading_level = 1] magic_level_splits: Title,
  #[default = false] arcane_level_2: bool,
  #[default = false] ice_level_1: bool,
  #[default = false] fire_level_1: bool,
  #[default = false] thunder_level_1: bool,
  #[default = false] absorption_level_1: bool,
  #[default = false] wind_level_1: bool,

  #[heading_level = 1] item_splits: Title,
  #[default = false] any_item: bool,
  #[default = false] last_item: bool,

  #[heading_level = 1] reset_settings: Title,
  #[default = false] death_reset: bool,
  #[default = true] title_reset: bool,
  ///Unlock NewGame+ Confirm reset
  #[default = true] ngp_unlock_reset: bool,

  #[heading_level = 1] other_settings: Title,
  #[default = false] treasure_chest: bool,
  #[default = true] split_every_boss_kill: bool,
  
  #[heading_level = 1] trial_tower: Title,
  #[default = false] split_on_timer_start: bool,
  #[default = false] start_with_timer: bool,
  #[default = false] start_on_practice_mode_exit: bool,
  #[default = false] start_on_new_game_trial_tower: bool,
  #[default = false] sync_boss_igt_in_practice_mode: bool,

  #[heading_level = 3] bosses: Title,
  #[default = true] armor: bool,
  #[default = true] tania: bool,
  #[default = true] monica_1: bool,
  #[default = true] monica_2: bool,
  #[default = true] vanessa_1: bool,
  #[default = true] vanessa_2: bool,
  #[default = true] nonota: bool,
  #[default = true] knight: bool,
  #[default = false] seal_1: bool,
  #[default = true] seal_2: bool,
}

async fn main() {
  let mut settings = Settings::register();
  { // set Tooltips
    set_tooltip("first_barrier_stone", "Barrier Stone for the Arcane Cast Tutorial.");
    set_tooltip("rescue_cat", "Splits when the post fight cutscene happens.");
    set_tooltip("ice_trial", "Splits when destroying the Barrier Stone for the Ice Cast Tutorial.");
    set_tooltip("destroy_wood_planks", "Destroying the Wooden Planks in Lava Ruins above the first Scissor Doll.");
    set_tooltip("fire_trial", "Splits on destroying the fire cast tutorial barrier stone.");
    set_tooltip("end_of_dark_tunnel", "Splits on skipping the last cutscene at the end of dark tunnel.");
    set_tooltip("death_reset", "Resets when Nobeta dies.");
    set_tooltip("title_reset", "Resets when you return to title screen except after beating the game.");
    set_tooltip("ngp_unlock_reset", "Resets when you confirm the NG+ unlock after beating the game.");
    set_tooltip("treasure_chest", "Splits every time when you OPEN a chest.");
    set_tooltip("split_every_boss_kill", "Allows you to split multiple times for beating the same boss.");
    set_tooltip("any_item", "Splits every item you pick up.");
    set_tooltip("last_item", "Splits when you picked up all 103 items.");
  } // set Tooltips
  print_message("Hello, World!");
  loop {
    timer::set_variable("Version", "Detached");
    let process = Process::wait_attach("LittleWitchNobeta.exe").await;
    process
      .until_closes(async {
        
        timer::set_variable("Version", "1.0.*");
        let module: Module = Module::wait_attach_auto_detect(&process).await;
        let image: Image = module.wait_get_default_image(&process).await;

        // Prefix c_ for Class
        // Prefix s_ for Offset to Static Table
        // Prefix o_ for Offset to Address
        // Prefix p_ for PointerPath
        let c_game: Class = image.wait_get_class(&process, &module, "Game").await;
        let c_asset_manager: Class = image.wait_get_class(&process, &module, "GameAssetManager").await;
        let c_scene_switch_data: Class = image.wait_get_class(&process, &module, "SceneSwitchData").await;
        let c_game_save: Class = image.wait_get_class(&process, &module, "GameSave").await;
        let c_player_stats_data: Class = image.wait_get_class(&process, &module, "PlayerStatsData").await;
        let c_stage_flag_data: Class = image.wait_get_class(&process, &module, "StageFlagData").await;
        let c_game_property_data: Class = image.wait_get_class(&process, &module, "GamePropertyData").await;
        let c_game_ui_manager: Class = image.wait_get_class(&process, &module, "GameUIManager").await;
        let c_ui_scene_loading: Class = image.wait_get_class(&process, &module, "UISceneLoading").await;
        let c_scene_manager: Class = image.wait_get_class(&process, &module, "SceneManager").await;
        let c_wizard_girl_manage: Class = image.wait_get_class(&process, &module, "WizardGirlManage").await;
        let c_player_controller: Class = image.wait_get_class(&process, &module, "PlayerController").await;
        let c_nobeta_runtime_data: Class = image.wait_get_class(&process, &module, "NobetaRuntimeData").await;
        let c_enemies_manager: Class = image.wait_get_class(&process, &module, "EnemiesManager").await;
        let c_enemy_data: Class = image.wait_get_class(&process, &module, "EnemyData").await;
        let c_npc_manage: Class = image.wait_get_class(&process, &module, "NPCManage").await;
        let c_ai_npc: Class = image.wait_get_class(&process, &module, "AI_NPC").await;
        let c_script_mode: Class = image.wait_get_class(&process, &module, "ScriptMode").await;
        let c_csv_data: Class = image.wait_get_class(&process, &module, "CSVData").await;

        let s_game: u64 = c_game.wait_get_field_offset(&process, &module, "instance").await as u64;
        let o_game_save: u64 = c_game.wait_get_field_offset(&process, &module, "gameSave").await as u64;
        let o_game_save_to_stats: u64 = c_game_save.wait_get_field_offset(&process, &module, "stats").await as u64;
        let o_game_save_to_props: u64 = c_game_save.wait_get_field_offset(&process, &module, "props").await as u64;
        let o_game_save_to_flags: u64 = c_game_save.wait_get_field_offset(&process, &module, "flags").await as u64;

        let s_scene_manager: u64 = c_game.wait_get_field_offset(&process, &module, "sceneManager").await as u64;
        let o_scene_manager_to_enemies_manager: u64 = c_scene_manager.wait_get_field_offset(&process, &module, "enemiesManager").await as u64;
        let o_enemies_manager_to_enemy_data_list: u64 = c_enemies_manager.wait_get_field_offset(&process, &module, "enemies").await as u64;
        let o_enemy_data_to_npc: u64 = c_enemy_data.wait_get_field_offset(&process, &module, "Npc").await as u64;
        let o_npc_manage_to_ai_npc: u64 = c_npc_manage.wait_get_field_offset(&process, &module, "aiNpc").await as u64;
        let o_ai_npc_to_dead: u64 = c_ai_npc.wait_get_field_offset(&process, &module, "g_bDeath").await as u64;

        let p_asset_manager_to_cached_asset: [u64; 4] = [ //<byte>((ptr, 0x20, 0x28, 0x10, 0x280)) { Name = "assetCached" },
          c_game.wait_get_field_offset(&process, &module, "assetManager").await as u64, // static
          c_asset_manager.wait_get_field_offset(&process, &module, "assetsCache").await as u64,
          // Points to Dictionary, Following Magic Values point into it the same way the ASL splitter did.
          0x10,
          0x280
        ];
        let p_scene_switch_next_scene: [u64; 3] = [ //new StringWatcher((ptr, 0x90, 0x10, 0x14), 128) { Name = "sceneName" },
          c_game.wait_get_field_offset(&process, &module, "SceneData").await as u64, // static,
          c_scene_switch_data.wait_get_field_offset(&process, &module, "nextSceneName").await as u64,
          0x14
        ];
        let p_magic_arcane: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x3C)) { Name = "Arcane" },
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "secretMagicLevel").await as u64
        ];
        let p_magic_ice: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x40)) { Name = "Ice" },
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "iceMagicLevel").await as u64
        ];
        let p_magic_fire: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x44)) { Name = "Fire" },
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "fireMagicLevel").await as u64
        ];
        let p_magic_thunder: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x48)) { Name = "Thunder" },
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "thunderMagicLevel").await as u64
        ];
        let p_magic_absorption: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x4C)) { Name = "Absorption" },
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "manaAbsorbLevel").await as u64
        ];
        let p_magic_wind: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x20, 0x50)) { Name = "Wind" }
          s_game,
          o_game_save,
          o_game_save_to_stats,
          c_player_stats_data.wait_get_field_offset(&process, &module, "windMagicLevel").await as u64
        ];
        let p_flag_crystal1: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x11)) { Name = "crystal 1" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage01Room03").await as u64
        ];
        let p_flag_save_cat: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x13)) { Name = "saveCat" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage01MeetCat").await as u64
        ];
        let p_flag_tutorial_ice: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x24)) { Name = "IceFire" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage02Room08").await as u64
        ];
        let p_flag_destroy_wood: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x38)) { Name = "destroy wood" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage03Room04Event01").await as u64
        ];
        let p_flag_tutorial_fire: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x3C)) { Name = "destroy crystal with fire" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage03Room06").await as u64
        ];
        let p_flag_monica_bear: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x3F)) { Name = "monica bear" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage03Boss01Clear").await as u64
        ];
        let p_flag_hat_lost: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x42)) { Name = "hatLost" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage04Room01HatLost").await as u64
        ];
        let p_flag_hat_get: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x43)) { Name = "hatGet" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage04Room01HatGet").await as u64
        ];
        let p_flag_tutorial_thunder: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x48)) { Name = "destroy 3 crystals" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage04Room06").await as u64
        ];
        let p_flag_dark_tunnel_done: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x4C)) { Name = "dark tunnel" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage04Room08").await as u64
        ];
        let p_flag_post_hat_orb: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x51)) { Name = "destroy orb" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage04Room01To04CrystalBall").await as u64
        ];
        let p_flag_crystal_for_teleport: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x60)) { Name = "destroy crystal open teleport" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage05Room07_03").await as u64
        ];
        let p_flag_abyss_door: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x64)) { Name = "open door abyss" }
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage06Act02DoorPlayer").await as u64
        ];
        let p_flag_abyss_challenge_left: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x6E)) { Name = "chalL" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage06RoomCentralAct03").await as u64
        ];
        let p_flag_abyss_challenge_right: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x6F)) { Name = "chalR" },
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage06RoomCentralAct04").await as u64
        ];
        let p_flag_abyss_challenge_center: [u64; 4] = [ //<byte>((ptr, 0x0, 0x18, 0x30, 0x70)) { Name = "chalC" }
          s_game,
          o_game_save,
          o_game_save_to_flags,
          c_stage_flag_data.wait_get_field_offset(&process, &module, "stage06RoomCentralAct05").await as u64
        ];
        let p_chest_count: [u64; 4] = [
          s_game,
          o_game_save,
          o_game_save_to_props,
          c_game_property_data.wait_get_field_offset(&process, &module, "treasureChestCollection").await as u64
        ];
        let p_progress_label: [u64; 5] = [ //<bool>((ptr, 0x0, 0x40, 0x30, 0x28, 0xA0)) { Name = "progressLabel" },
          s_game,
          c_game.wait_get_field_offset(&process, &module, "ui").await as u64,
          c_game_ui_manager.wait_get_field_offset(&process, &module, "loading").await as u64,
          c_ui_scene_loading.wait_get_field_offset(&process, &module, "progressLabel").await as u64,
          0xA0
        ];
        let p_stage_id: [u64; 2] = [ //<byte>((ptr, 0x78, 0xE8)) { Name = "StageID" },
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "stageId").await as u64
        ];
        let p_on_system_menu: [u64; 2] = [ //<bool>((ptr, 0x78, 0xD0)) { Name = "onSystemMenu" },
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "onSystemMenu").await as u64
        ];
        let p_stage_state: [u64; 2] = [ //<byte>((ptr, 0x78, 0x58)) { Name = "stageState" }, //state == 0 (normal = 0, death = 1, cutscene = 2, pray = 3)
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "stageState").await as u64
        ];
        let p_boss_dialogue: [u64; 2] = [ //<byte>((ptr, 0x78, 0x88)) { Name = "bossDialogue" },
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "bossDialogue").await as u64
        ];
        let p_player_is_dead: [u64; 5] = [ //<bool>((ptr, 0x78, 0xA0, 0x18, 0x20, 0x11)) { Name = "isDead" },
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "wizardGirl").await as u64,
          c_wizard_girl_manage.wait_get_field_offset(&process, &module, "playerController").await as u64,
          c_player_controller.wait_get_field_offset(&process, &module, "runtimeData").await as u64,
          c_nobeta_runtime_data.wait_get_field_offset(&process, &module, "isDead").await as u64
        ];
        let p_boss_armor: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x138, 0x10, 0x70, 0x166)) { Name = "Armor" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x138,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_secret: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x218, 0x10, 0x70, 0x166)) { Name = "Secret" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x218,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_tania: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x1C0, 0x10, 0x70, 0x166)) { Name = "Tania" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x1C0,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_monica_1: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x140, 0x10, 0x70, 0x166)) { Name = "Monica1" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x140,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_monica_2: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0xF8, 0x10, 0x70, 0x166)) { Name = "Monica2" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0xF8,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_vanessa_1: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x78, 0x10, 0x70, 0x166)) { Name = "Vanessa1" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x78,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_vanessa_2: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x220, 0x10, 0x70, 0x166)) { Name = "Vanessa2" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x220,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_nonota: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0xA8, 0x10, 0x70, 0x166)) { Name = "Nonota" }
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0xA8,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_knight: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x288, 0x10, 0x70, 0x166)) { Name = "Knight" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x288,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_seal_1: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x218, 0x10, 0x70, 0x166)) { Name = "seal_1" },
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x218,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_boss_seal_2: [u64; 8] = [ //<byte>((ptr, 0x78, 0xA8, 0x10, 0x10, 0x270, 0x10, 0x70, 0x166)) { Name = "seal_2" }
          s_scene_manager,
          o_scene_manager_to_enemies_manager,
          o_enemies_manager_to_enemy_data_list,
          0x10,
          0x270,
          o_enemy_data_to_npc,
          o_npc_manage_to_ai_npc,
          o_ai_npc_to_dead
        ];
        let p_script_name: [u64; 5] = [
          s_scene_manager,
          c_scene_manager.wait_get_field_offset(&process, &module, "scriptSystem").await as u64,
          c_script_mode.wait_get_field_offset(&process, &module, "g_ScriptData").await as u64,
          c_csv_data.wait_get_field_offset(&process, &module, "assetName").await as u64,
          0x14
        ];
        let p_items: [u64; 5] = [
          s_game,
          o_game_save,
          o_game_save_to_props,
          c_game_property_data.wait_get_field_offset(&process, &module, "propCollection").await as u64,
          0x20
        ];

        let version: bool; // True if 1.1.0+, False if 1.0.5-
        //Trial Tower Stuff
        let c_boss_rush_data = image.get_class(&process, &module, "BossRushData");
        let p_tt_armor_timer:[u64; 4];
        let p_tt_tania_timer:[u64;4];
        let p_tt_monica_timer: [u64; 4];
        let p_tt_vanessa_1_timer: [u64; 4];
        let p_tt_vanessa_2_timer: [u64; 4];
        let p_tt_nonota_timer: [u64; 4];
        let p_tt_knight_timer: [u64; 4];
        let p_tt_seal_timer: [u64; 4];
        let p_tt_test_timer: [u64; 4];
        let p_tt_test_mode: [u64; 4];
        let p_tt_armor: [u64; 8];
        let p_tt_tania: [u64; 8];
        let p_tt_monica_1: [u64; 8];
        let p_tt_monica_2: [u64; 8];
        let p_tt_knight: [u64; 8];
        let p_tt_vanessa_1: [u64; 8];
        let p_tt_vanessa_2: [u64; 8];
        let p_tt_seal_1: [u64; 8];
        let p_tt_seal_2:[u64; 8];
        let p_tt_nonota:[u64; 8];
        {
          if c_boss_rush_data.is_some() { // Version 1.1.0 or later
            timer::set_variable("Version", "1.1.*");
            version = true;
            let c_boss_rush_data = c_boss_rush_data.unwrap();
            let o_game_save_to_boss_rush: u64 = c_game_save.wait_get_field_offset(&process, &module, "bossRush").await as u64;

            p_tt_armor_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss01Time").await as u64
            ];
            p_tt_tania_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss02Time").await as u64
            ];
            p_tt_monica_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss03Time").await as u64
            ];
            p_tt_vanessa_1_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss04Time").await as u64
            ];
            p_tt_vanessa_2_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss05Time").await as u64
            ];
            p_tt_nonota_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "boss06Time").await as u64
            ];
            p_tt_knight_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "knightTime").await as u64
            ];
            p_tt_seal_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "sealGhostTime").await as u64
            ];
            p_tt_test_timer = [
              s_game,
              o_game_save,
              o_game_save_to_boss_rush,
              c_boss_rush_data.wait_get_field_offset(&process, &module, "TestBattleTime").await as u64
            ];
            p_tt_test_mode = [
              s_game,
              o_game_save,
              o_game_save_to_flags,
              c_stage_flag_data.wait_get_field_offset(&process, &module, "stageBRTest").await as u64
            ];

            p_tt_armor = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x50,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_tania = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x60,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_monica_1 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x40,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_monica_2 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x68,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_knight = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x58,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_vanessa_1 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x38,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_vanessa_2 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x28,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_seal_1 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x20,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_seal_2 = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x30,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
            p_tt_nonota = [
              s_scene_manager,
              o_scene_manager_to_enemies_manager,
              o_enemies_manager_to_enemy_data_list,
              0x10,
              0x48,
              o_enemy_data_to_npc,
              o_npc_manage_to_ai_npc,
              o_ai_npc_to_dead
            ];
          } else { version = false;
            // Initializing variables in the case that will NEVER see them be used so the compiler shuts up about it!!!!!
            p_tt_armor_timer = [0;4];
            p_tt_tania_timer =[0;4];
            p_tt_monica_timer = [0; 4];
            p_tt_vanessa_1_timer = [0; 4];
            p_tt_vanessa_2_timer = [0; 4];
            p_tt_nonota_timer = [0; 4];
            p_tt_knight_timer = [0; 4];
            p_tt_seal_timer = [0; 4];
            p_tt_test_timer = [0; 4];
            p_tt_test_mode = [0; 4];
            p_tt_armor = [0; 8];
            p_tt_tania = [0; 8];
            p_tt_monica_1 = [0; 8];
            p_tt_monica_2 = [0; 8];
            p_tt_knight = [0; 8];
            p_tt_vanessa_1 = [0; 8];
            p_tt_vanessa_2 = [0; 8];
            p_tt_seal_1 = [0; 8];
            p_tt_seal_2 = [0; 8];
            p_tt_nonota = [0; 8];
          } // Nothing to see here, just initializing Variables for no reason cause the compiler is too dumb to see that they are not capable of being used if this else is hit.
        }
      

    let static_table = c_game.wait_get_static_table(&process, &module).await;

        timer::set_variable("isLoading", "True");
        timer::set_variable("sceneName", "Uninitialized");
        timer::set_variable("scriptName", "None");
        let mut is_loading;
        let mut scene_name:Watcher<ArrayString<128>> = Watcher::new();
        let mut magic_arcane:Watcher<u8> = Watcher::new();
        let mut magic_ice:Watcher<u8> = Watcher::new();
        let mut magic_fire:Watcher<u8> = Watcher::new();
        let mut magic_thunder:Watcher<u8> = Watcher::new();
        let mut magic_absorption:Watcher<u8> = Watcher::new();
        let mut magic_wind:Watcher<u8> = Watcher::new();
        let mut flag_crystal1:Watcher<u8> = Watcher::new();
        let mut flag_save_cat:Watcher<u8> = Watcher::new();
        let mut flag_tutorial_ice:Watcher<u8> = Watcher::new();
        let mut flag_destroy_wood:Watcher<u8> = Watcher::new();
        let mut flag_tutorial_fire:Watcher<u8> = Watcher::new();
        let mut flag_monica_bear:bool = false;
        let mut flag_hat_lost:Watcher<u8> = Watcher::new();
        let mut flag_hat_get:Watcher<u8> = Watcher::new();
        let mut flag_tutorial_thunder:Watcher<u8> = Watcher::new();
        let mut flag_post_hat_orb:Watcher<u8> = Watcher::new();
        let mut flag_dark_tunnel_done:Watcher<u8> = Watcher::new();
        let mut flag_crystal_for_teleport:Watcher<u8> = Watcher::new();
        let mut flag_abyss_door:Watcher<u8> = Watcher::new();
        let mut flag_abyss_challenge_left:Watcher<u8> = Watcher::new();
        let mut flag_abyss_challenge_right:Watcher<u8> = Watcher::new();
        let mut flag_abyss_challenge_center:Watcher<u8> = Watcher::new();
        let mut chest_count:Watcher<u8> = Watcher::new();
        let mut progress_label:Watcher<bool> = Watcher::new();
        let mut stage_id:u8 = 255;
        let mut on_system_menu:Watcher<bool> = Watcher::new();
        let mut stage_state:Watcher<u8> = Watcher::new();
        let mut boss_dialogue:Watcher<u8> = Watcher::new();
        let mut player_is_dead:Watcher<bool> = Watcher::new();
        let mut boss_armor:Watcher<u8> = Watcher::new();
        let mut boss_secret:Watcher<u8> = Watcher::new();
        let mut boss_tania:Watcher<u8> = Watcher::new();
        let mut boss_monica_1:Watcher<u8> = Watcher::new();
        let mut boss_monica_2:Watcher<u8> = Watcher::new();
        let mut boss_vanessa_1:Watcher<u8> = Watcher::new();
        let mut boss_vanessa_2:Watcher<u8> = Watcher::new();
        let mut boss_nonota:Watcher<u8> = Watcher::new();
        let mut boss_knight:Watcher<u8> = Watcher::new();
        let mut boss_seal_1:Watcher<u8> = Watcher::new();
        let mut boss_seal_2:Watcher<u8> = Watcher::new();
        let mut script_name:Watcher<ArrayString<128>> = Watcher::new();
        let mut item_count:u8 = 0;
        let mut tt_armor_time:Watcher<f32> = Watcher::new();
        let mut tt_tania_time:Watcher<f32> = Watcher::new();
        let mut tt_monica_time:Watcher<f32> = Watcher::new();
        let mut tt_vanessa_1_time:Watcher<f32> = Watcher::new();
        let mut tt_vanessa_2_time:Watcher<f32> = Watcher::new();
        let mut tt_nonota_time:Watcher<f32> = Watcher::new();
        let mut tt_knight_time:Watcher<f32> = Watcher::new();
        let mut tt_seal_time:Watcher<f32> = Watcher::new();
        let mut tt_test_time:Watcher<f32> = Watcher::new();
        let mut tt_test_mode:bool = false;
        let mut tt_armor:Watcher<u8> = Watcher::new();
        let mut tt_tania:Watcher<u8> = Watcher::new();
        let mut tt_monica_1:Watcher<u8> = Watcher::new();
        let mut tt_monica_2:Watcher<u8> = Watcher::new();
        let mut tt_knight:Watcher<u8> = Watcher::new();
        let mut tt_vanessa_1:Watcher<u8> = Watcher::new();
        let mut tt_vanessa_2:Watcher<u8> = Watcher::new();
        let mut tt_seal_1:Watcher<u8> = Watcher::new();
        let mut tt_seal_2:Watcher<u8> = Watcher::new();
        let mut tt_nonota:Watcher<u8> = Watcher::new();
        
        let empty = ArrayString::<128>::from("").unwrap();
        scene_name.update_infallible(empty);
        magic_arcane.update_infallible(0);
        magic_ice.update_infallible(0);
        magic_fire.update_infallible(0);
        magic_thunder.update_infallible(0);
        magic_absorption.update_infallible(0);
        magic_wind.update_infallible(0);
        flag_crystal1.update_infallible(0);
        flag_save_cat.update_infallible(0);
        flag_tutorial_ice.update_infallible(0);
        flag_destroy_wood.update_infallible(0);
        flag_tutorial_fire.update_infallible(0);
        flag_hat_lost.update_infallible(0);
        flag_hat_get.update_infallible(0);
        flag_tutorial_thunder.update_infallible(0);
        flag_post_hat_orb.update_infallible(0);
        flag_dark_tunnel_done.update_infallible(0);
        flag_crystal_for_teleport.update_infallible(0);
        flag_abyss_door.update_infallible(0);
        flag_abyss_challenge_left.update_infallible(0);
        flag_abyss_challenge_right.update_infallible(0);
        flag_abyss_challenge_center.update_infallible(0);
        chest_count.update_infallible(0);
        progress_label.update_infallible(false);
        on_system_menu.update_infallible(false);
        stage_state.update_infallible(0);
        boss_dialogue.update_infallible(0);
        player_is_dead.update_infallible(false);
        boss_armor.update_infallible(0);
        boss_secret.update_infallible(0);
        boss_tania.update_infallible(0);
        boss_monica_1.update_infallible(0);
        boss_monica_2.update_infallible(0);
        boss_vanessa_1.update_infallible(0);
        boss_vanessa_2.update_infallible(0);
        boss_nonota.update_infallible(0);
        boss_knight.update_infallible(0);
        boss_seal_1.update_infallible(0);
        boss_seal_2.update_infallible(0);
        script_name.update_infallible(empty);
        tt_armor_time.update_infallible(100f32);
        tt_tania_time.update_infallible(100f32);
        tt_monica_time.update_infallible(100f32);
        tt_vanessa_1_time.update_infallible(100f32);
        tt_vanessa_2_time.update_infallible(100f32);
        tt_nonota_time.update_infallible(100f32);
        tt_knight_time.update_infallible(100f32);
        tt_seal_time.update_infallible(100f32);
        tt_test_time.update_infallible(100f32);
        tt_armor.update_infallible(0);
        tt_tania.update_infallible(0);
        tt_monica_1.update_infallible(0);
        tt_monica_2.update_infallible(0);
        tt_knight.update_infallible(0);
        tt_vanessa_1.update_infallible(0);
        tt_vanessa_2.update_infallible(0);
        tt_seal_1.update_infallible(0);
        tt_seal_2.update_infallible(0);
        tt_nonota.update_infallible(0);

        let mut lava_middle_cutscene_seen:bool = false;
        let mut armor_start:bool = false;
        let mut armor_dead:bool = false;
        let mut tania_start:bool = false;
        let mut tania_dead:bool = false;
        let mut monica_1_start:bool = false;
        let mut monica_1_dead:bool = false;
        let mut monica_2_start:bool = false;
        let mut monica_2_dead:bool = false;
        let mut vanessa_1_start:bool = false;
        let mut vanessa_1_dead:bool = false;
        let mut vanessa_2_start:bool = false;
        let mut vanessa_2_dead:bool = false;
        let mut nonota_start:bool = false;
        let mut nonota_dead:bool = false;
        let mut secret_start:bool = false;
        let mut secret_dead:bool = false;
        let mut knight_dead:bool = false;
        let mut seal_1_start:bool = false;
        let mut seal_1_dead:bool = false;
        let mut seal_2_start:bool = false;
        let mut seal_2_dead:bool = false;

        let mut max_items:u8 = 0;
        let mut last_item:bool = false;

        let mut trial_tower:bool = false;
        let mut tt_dead:bool = false;
        
        
        print_message("Start Loop");
        loop {
          settings.update();

          { // Update Pointers
            if let Ok(asset_cached) = process.read_pointer_path::<u8>(static_table, Bit64, &p_asset_manager_to_cached_asset) {
              is_loading = asset_cached == 255;
            } else { is_loading = true; }
            if let Ok(next_scene_name) = process.read_pointer_path::<ArrayWString<128>>(static_table, Bit64, &p_scene_switch_next_scene) {
              scene_name.update_infallible(wstring_to_string(next_scene_name));
            }
            if let Ok(arcane) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_arcane) {
              magic_arcane.update_infallible(arcane);
            }
            if let Ok(ice) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_ice) {
              magic_ice.update_infallible(ice);
            }
            if let Ok(fire) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_fire) {
              magic_fire.update_infallible(fire);
            }
            if let Ok(thunder) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_thunder) {
              magic_thunder.update_infallible(thunder);
            }
            if let Ok(absorption) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_absorption) {
              magic_absorption.update_infallible(absorption);
            }
            if let Ok(wind) = process.read_pointer_path::<u8>(static_table, Bit64, &p_magic_wind) {
              magic_wind.update_infallible(wind);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_crystal1) {
              flag_crystal1.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_save_cat) {
              flag_save_cat.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_tutorial_ice) {
              flag_tutorial_ice.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_destroy_wood) {
              flag_destroy_wood.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_tutorial_fire) {
              flag_tutorial_fire.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_monica_bear) {
              flag_monica_bear = flag == 1;
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_hat_lost) {
              flag_hat_lost.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_hat_get) {
              flag_hat_get.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_tutorial_thunder) {
              flag_tutorial_thunder.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_post_hat_orb) {
              flag_post_hat_orb.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_dark_tunnel_done) {
              flag_dark_tunnel_done.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_crystal_for_teleport) {
              flag_crystal_for_teleport.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_abyss_door) {
              flag_abyss_door.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_abyss_challenge_left) {
              flag_abyss_challenge_left.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_abyss_challenge_right) {
              flag_abyss_challenge_right.update_infallible(flag);
            }
            if let Ok(flag) = process.read_pointer_path::<u8>(static_table, Bit64, &p_flag_abyss_challenge_center) {
              flag_abyss_challenge_center.update_infallible(flag);
            }
            if let Ok(chests) = process.read_pointer_path::<u8>(static_table, Bit64, &p_chest_count) {
              chest_count.update_infallible(chests);
            }
            if let Ok(label) = process.read_pointer_path::<bool>(static_table, Bit64, &p_progress_label) {
              progress_label.update_infallible(label);
            }
            if let Ok(id) = process.read_pointer_path::<u8>(static_table, Bit64, &p_stage_id) {
              stage_id = id;
            }
            if let Ok(in_menu) = process.read_pointer_path::<bool>(static_table, Bit64, &p_on_system_menu) {
              on_system_menu.update_infallible(in_menu);
            }
            if let Ok(state) = process.read_pointer_path::<u8>(static_table, Bit64, &p_stage_state) {
              stage_state.update_infallible(state);
            }
            if let Ok(dialogue) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_dialogue) {
              boss_dialogue.update_infallible(dialogue);
            }
            if let Ok(is_dead) = process.read_pointer_path::<bool>(static_table, Bit64, &p_player_is_dead) {
              player_is_dead.update_infallible(is_dead);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_armor) {
              boss_armor.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_secret) {
              boss_secret.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_tania) {
              boss_tania.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_monica_1) {
              boss_monica_1.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_monica_2) {
              boss_monica_2.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_vanessa_1) {
              boss_vanessa_1.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_vanessa_2) {
              boss_vanessa_2.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_nonota) {
              boss_nonota.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_knight) {
              boss_knight.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_seal_1) {
              boss_seal_1.update_infallible(boss);
            }
            if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_boss_seal_2) {
              boss_seal_2.update_infallible(boss);
            }
            if let Ok(script) = process.read_pointer_path::<ArrayWString<128>>(static_table, Bit64, &p_script_name) {
              script_name.update_infallible(wstring_to_string(script));
            }
            {
              let mut i: u8 = 0;
              let mut count: u8 = 0;
              let mut err: bool = false;
              while i < 103 {
                if let Ok(item) = process.read_pointer_path::<u8>(static_table, Bit64, &[p_items[0], p_items[1], p_items[2], p_items[3], p_items[4] + i as u64]) {
                  if item != 0 { count = count + 1 }
                } else { err = true }
                if err { break; }
                i = i + 1;
              }
              if !err {
                item_count = count;
              }
            }
            if version {
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_armor_timer) {
                tt_armor_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_tania_timer) {
                tt_tania_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_monica_timer) {
                tt_monica_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_vanessa_1_timer) {
                tt_vanessa_1_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_vanessa_2_timer) {
                tt_vanessa_2_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_nonota_timer) {
                tt_nonota_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_knight_timer) {
                tt_knight_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_seal_timer) {
                tt_seal_time.update_infallible(time);
              }
              if let Ok(time) = process.read_pointer_path::<f32>(static_table, Bit64, &p_tt_test_timer) {
                tt_test_time.update_infallible(time);
              }
              if let Ok(mode) = process.read_pointer_path::<bool>(static_table, Bit64, &p_tt_test_mode) {
                tt_test_mode = mode;
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_armor) {
                tt_armor.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_tania) {
                tt_tania.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_monica_1) {
                tt_monica_1.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_monica_2) {
                tt_monica_2.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_knight) {
                tt_knight.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_vanessa_1) {
                tt_vanessa_1.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_vanessa_2) {
                tt_vanessa_2.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_seal_1) {
                tt_seal_1.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_seal_2) {
                tt_seal_2.update_infallible(boss);
              }
              if let Ok(boss) = process.read_pointer_path::<u8>(static_table, Bit64, &p_tt_nonota) {
                tt_nonota.update_infallible(boss);
              }
            }
          } // Update Pointers


          timer::set_variable("isLoading", if is_loading { "True" } else { "False" });
          timer::set_variable("sceneName", scene_name.clone().pair.unwrap().current.as_str());
          timer::set_variable("scriptName", script_name.clone().pair.unwrap().current.as_str());

          // Reset
          {
            if settings.reset && timer::state() != TimerState::NotRunning {
              let staff: ArrayString<128> = ArrayString::<128>::from("Staff").unwrap();
              let title: ArrayString<128> = ArrayString::<128>::from("Title").unwrap();
              if settings.ngp_unlock_reset && scene_name.pair.unwrap().changed_from_to(&staff, &title) {
                print_message("NG+ Unlock Reset");
                timer::reset()
              }
              if settings.title_reset && scene_name.pair.unwrap().changed_to(&title) && scene_name.pair.unwrap().old != staff {
                print_message("Return to Title Reset");
                timer::reset()
              }
              if settings.death_reset && player_is_dead.pair.unwrap().changed_to(&true) {
                print_message("Death Reset");
                timer::reset()
              }
            }
          }
          
          // Reset variables if timer is not running
          if timer::state() == TimerState::NotRunning {
            timer::set_game_time(time::Duration::new(0,0));
            lava_middle_cutscene_seen = false;
            armor_start = false;
            armor_dead = false;
            tania_start = false;
            tania_dead = false;
            monica_1_start = false;
            monica_1_dead = false;
            monica_2_start = false;
            monica_2_dead = false;
            vanessa_1_start = false;
            vanessa_1_dead = false;
            vanessa_2_start = false;
            vanessa_2_dead = false;
            nonota_start = false;
            nonota_dead = false;
            secret_start = false;
            secret_dead = false;
            knight_dead = false;
            seal_1_start = false;
            seal_1_dead = false;
            seal_2_start = false;
            seal_2_dead = false;
            max_items = 0;
            last_item = false;
            tt_dead = false;
          }
          if player_is_dead.pair.unwrap().changed_to(&false) { tt_dead = true }
          // Load Removal / Game Time
          if settings.sync_boss_igt_in_practice_mode && tt_test_mode {
            timer::pause_game_time();
            timer::set_game_time(time::Duration::seconds_f32(tt_test_time.pair.unwrap().current))
          } else if is_loading { timer::pause_game_time() } else { timer::resume_game_time() }
          // Start
          {
            if settings.start && timer::state() == TimerState::NotRunning {
              let act1: ArrayString<128> = ArrayString::<128>::from("Act01_02").unwrap();
              let boss_rush: ArrayString<128> = ArrayString::<128>::from("BossRush01").unwrap();
              let reset_boss_rush: ArrayString<128> = ArrayString::<128>::from("BossRush_Dialogue_ResetBossRush_Submit").unwrap();
              let exit_test_mode: ArrayString<128> = ArrayString::<128>::from("BossRush_Dialogue_RemoveTestMode_Submit").unwrap();
              if scene_name.pair.unwrap().changed_to(&act1) {
                print_message("Start Main Game");
                trial_tower = false;
                timer::start()
              }
              if version {
                if settings.start_on_new_game_trial_tower && scene_name.pair.unwrap().changed_to(&boss_rush) {
                  print_message("Trial Tower start.");
                  trial_tower = true;
                  timer::start()
                }
                if settings.start_on_practice_mode_exit && stage_state.pair.unwrap().changed()
                  && (script_name.pair.unwrap().current == reset_boss_rush
                  || script_name.pair.unwrap().current == exit_test_mode) {
                  print_message("Practice Mode Exit Start");
                  trial_tower = true;
                  timer::start()
                }
              }
              if settings.start_with_timer {
                if tt_test_time.pair.unwrap().old == 0f32 && tt_test_time.pair.unwrap().current > 0f32 { print_message("Start TT Test Time"); trial_tower = true; timer::start()}
                if tt_armor_time.pair.unwrap().old == 0f32 && tt_armor_time.pair.unwrap().current > 0f32 { print_message("Start TT Armor Time"); trial_tower = true; timer::start() }
                if tt_tania_time.pair.unwrap().old == 0f32 && tt_tania_time.pair.unwrap().current > 0f32 { print_message("Start TT Tania Time"); trial_tower = true; timer::start() }
                if tt_monica_time.pair.unwrap().old == 0f32 && tt_monica_time.pair.unwrap().current > 0f32 { print_message("Start TT Monica Time"); trial_tower = true; timer::start() }
                if tt_vanessa_1_time.pair.unwrap().old == 0f32 && tt_vanessa_1_time.pair.unwrap().current > 0f32 { print_message("Start TT Vanessa 1 Time"); trial_tower = true; timer::start() }
                if tt_vanessa_2_time.pair.unwrap().old == 0f32 && tt_vanessa_2_time.pair.unwrap().current > 0f32 { print_message("Start TT Vanessa 2 Time"); trial_tower = true; timer::start() }
                if tt_nonota_time.pair.unwrap().old == 0f32 && tt_nonota_time.pair.unwrap().current > 0f32 { print_message("Start TT Nonota Time"); trial_tower = true; timer::start() }
                if tt_knight_time.pair.unwrap().old == 0f32 && tt_knight_time.pair.unwrap().current > 0f32 { print_message("Start TT Knight Time"); trial_tower = true; timer::start() }
                if tt_seal_time.pair.unwrap().old == 0f32 && tt_seal_time.pair.unwrap().current > 0f32 { print_message("Start TT Seal Time"); trial_tower = true; timer::start() }
              }
            }
          }
          
          // Split
          {
            if settings.split && timer::state() != TimerState::NotRunning && timer::state() != TimerState::Ended && timer::state() != TimerState::Unknown {
              if !trial_tower {
                // Main Game Boss Start Splits
                {
                  if settings.armor_start && boss_dialogue.pair.unwrap().changed_to(&1) && !armor_start {
                    armor_start = true;
                    print_message("Split Armor Start");
                    timer::split()
                  }
                  if settings.tania_start && boss_dialogue.pair.unwrap().changed_to(&2) && !tania_start {
                    tania_start = true;
                    print_message("Split Tania Start");
                    timer::split()
                  }
                  if boss_dialogue.pair.unwrap().changed_to(&3) {
                    if settings.monica_1_start && !flag_monica_bear && !monica_1_start {
                      monica_1_start = true;
                      print_message("Split Monica 1 Start");
                      timer::split()
                    } else if settings.monica_2_start && flag_monica_bear && !monica_2_start {
                      monica_2_start = true;
                      print_message("Split Monica 2 Start");
                      timer::split()
                    }
                  }
                  if settings.vanessa_1_start && boss_dialogue.pair.unwrap().changed_to(&4) && !vanessa_1_start {
                    vanessa_1_start = true;
                    print_message("Split Vanessa 1 Start");
                    timer::split()
                  }
                  if settings.vanessa_2_start && boss_dialogue.pair.unwrap().changed_to(&5) && !vanessa_2_start {
                    vanessa_2_start = true;
                    print_message("Split Vanessa 2 Start");
                    timer::split()
                  }
                  if settings.nonota_start && boss_dialogue.pair.unwrap().changed_to(&6) && !nonota_start {
                    nonota_start = true;
                    print_message("Split Nonota Start");
                    timer::split()
                  }
                  if settings.secret_armor_start && boss_dialogue.pair.unwrap().changed_to(&7) && !secret_start {
                    secret_start = true;
                    print_message("Split Secret Armor Start");
                    timer::split()
                  }
                  let seal_1: ArrayString<128> = ArrayString::<128>::from("Act06_Room05").unwrap();
                  let seal_2: ArrayString<128> = ArrayString::<128>::from("Act06_Room06").unwrap();
                  if settings.seal_1_start && script_name.pair.unwrap().changed_to(&seal_1) && !seal_1_start {
                    seal_1_start = true;
                    print_message("Split Seal 1 Start");
                    timer::split()
                  }
                  if settings.seal_2_start && script_name.pair.unwrap().changed_to(&seal_2) && !seal_2_start {
                    seal_2_start = true;
                    print_message("Split Seal 2 Start");
                    timer::split()
                  }
                }

                // Main Game Boss Dead Splits
                {
                  if settings.armor_dead && stage_id == 2 && boss_armor.pair.unwrap().old +1 == boss_armor.pair.unwrap().current && !armor_dead {
                    armor_dead = !settings.split_every_boss_kill;
                    print_message("Split Armor Dead");
                    timer::split()
                  }
                  if settings.secret_armor_dead && stage_id == 2 && boss_secret.pair.unwrap().old +1 == boss_secret.pair.unwrap().current && !secret_dead {
                    secret_dead = !settings.split_every_boss_kill;
                    print_message("Split Secret Armor Dead");
                    timer::split()
                  }
                  if settings.tania_dead && stage_id == 3 && boss_tania.pair.unwrap().old +1 == boss_tania.pair.unwrap().current && !tania_dead {
                    tania_dead = !settings.split_every_boss_kill;
                    print_message("Split Tania Dead");
                    timer::split()
                  }
                  if settings.monica_1_dead && stage_id == 4 && boss_monica_1.pair.unwrap().old +1 == boss_monica_1.pair.unwrap().current && !monica_1_dead {
                    monica_1_dead = !settings.split_every_boss_kill;
                    print_message("Split Monica 1 Dead");
                    timer::split()
                  }
                  if settings.monica_2_dead && stage_id == 4 && boss_monica_2.pair.unwrap().old +1 == boss_monica_2.pair.unwrap().current && !monica_2_dead {
                    monica_2_dead = !settings.split_every_boss_kill;
                    print_message("Split Monica 2 Dead");
                    timer::split()
                  }
                  if settings.vanessa_1_dead && stage_id == 5 && boss_vanessa_1.pair.unwrap().old +1 == boss_vanessa_1.pair.unwrap().current && !vanessa_1_dead {
                    vanessa_1_dead = !settings.split_every_boss_kill;
                    print_message("Split Vanessa 1 Dead");
                    timer::split()
                  }
                  if settings.vanessa_2_dead && stage_id == 6 && boss_vanessa_2.pair.unwrap().old +1 == boss_vanessa_2.pair.unwrap().current && !vanessa_2_dead {
                    vanessa_2_dead = !settings.split_every_boss_kill;
                    print_message("Split Vanessa 2 Dead");
                    timer::split()
                  }
                  if settings.nonota_dead && stage_id == 7 && boss_nonota.pair.unwrap().old +1 == boss_nonota.pair.unwrap().current && !nonota_dead {
                    nonota_dead = !settings.split_every_boss_kill;
                    print_message("Split Nonota Dead");
                    timer::split()
                  }
                  if settings.knight_dead && stage_id == 5 && boss_knight.pair.unwrap().old +1 == boss_knight.pair.unwrap().current && !knight_dead {
                    knight_dead = !settings.split_every_boss_kill;
                    print_message("Split Knight Dead");
                    timer::split()
                  }
                  if settings.seal_1_dead && stage_id == 6 && boss_seal_1.pair.unwrap().old +1 == boss_seal_1.pair.unwrap().current && !seal_1_dead {
                    seal_1_dead = !settings.split_every_boss_kill;
                    print_message("Split Seal 1 Dead");
                    timer::split()
                  }
                  if settings.seal_2_dead && stage_id == 6 && boss_seal_2.pair.unwrap().old +1 == boss_seal_2.pair.unwrap().current && !seal_2_dead {
                    seal_2_dead = !settings.split_every_boss_kill;
                    print_message("Split Seal 2 Dead");
                    timer::split()
                  }
                }

                // Cutscenes
                {
                  if settings.first_barrier_stone && flag_crystal1.pair.unwrap().old +1 == flag_crystal1.pair.unwrap().current {
                    print_message("Split First Barrier Stone");
                    timer::split()
                  }
                  if settings.rescue_cat && flag_save_cat.pair.unwrap().old +1 == flag_save_cat.pair.unwrap().current {
                    print_message("Split Cat Rescue");
                    timer::split()
                  }
                  if settings.ice_trial && flag_tutorial_ice.pair.unwrap().old +1 == flag_tutorial_ice.pair.unwrap().current {
                    print_message("Split Ice Trial");
                    timer::split()
                  }
                  if settings.destroy_wood_planks && flag_destroy_wood.pair.unwrap().old +1 == flag_destroy_wood.pair.unwrap().current {
                    print_message("Split Destroy Wood");
                    timer::split()
                  }
                  let lava_ruins_cutscene: ArrayString<128> = ArrayString::<128>::from("Act04_Room05").unwrap();
                  if settings.mid_lava_cutscene && script_name.pair.unwrap().changed_to(&lava_ruins_cutscene) && !lava_middle_cutscene_seen {
                    // The flag for this cutscene is delayed by 8 seconds, implementing it this way for more consistent splitting.
                    lava_middle_cutscene_seen = true;
                    print_message("Split Lava Ruins Middle Cutscene");
                    timer::split()
                  }
                  if settings.fire_trial && flag_tutorial_fire.pair.unwrap().old +1 == flag_tutorial_fire.pair.unwrap().current {
                    print_message("Split Fire Trial");
                    timer::split()
                  }
                  if settings.hat_lost && flag_hat_lost.pair.unwrap().old +1 == flag_hat_lost.pair.unwrap().current {
                    print_message("Split Hat Lost");
                    timer::split()
                  }
                  if settings.hat_get && flag_hat_get.pair.unwrap().old +1 == flag_hat_get.pair.unwrap().current {
                    print_message("Split Hat Get");
                    timer::split()
                  }
                  if settings.orb_after_get_hat && flag_post_hat_orb.pair.unwrap().old +1 == flag_post_hat_orb.pair.unwrap().current {
                    print_message("Split Post Hat Get Orb");
                    timer::split()
                  }
                  if settings.thunder_trial && flag_tutorial_thunder.pair.unwrap().old +1 == flag_tutorial_thunder.pair.unwrap().current {
                    print_message("Split Thunder Trial");
                    timer::split()
                  }
                  if settings.end_of_dark_tunnel && flag_dark_tunnel_done.pair.unwrap().old +1 == flag_dark_tunnel_done.pair.unwrap().current {
                    print_message("Split Dark Tunnel End Cutscene");
                    timer::split()
                  }
                  if settings.castle_4_barrier_stone && flag_crystal_for_teleport.pair.unwrap().old +1 == flag_crystal_for_teleport.pair.unwrap().current {
                    print_message("Split Crystal for teleporter");
                    timer::split()
                  }
                  if settings.open_abyss_door && flag_abyss_door.pair.unwrap().old +1 == flag_abyss_door.pair.unwrap().current {
                    print_message("Split Abyss Door");
                    timer::split()
                  }
                  if stage_id == 7 {
                    if settings.left_challenge && flag_abyss_challenge_left.pair.unwrap().old +1 == flag_abyss_challenge_left.pair.unwrap().current {
                      print_message("Split Abyss Challenge Left");
                      timer::split()
                    }
                    if settings.right_challenge && flag_abyss_challenge_right.pair.unwrap().old +1 == flag_abyss_challenge_right.pair.unwrap().current {
                      print_message("Split Abyss Challenge Right");
                      timer::split()
                    }
                    if settings.center_challenge && flag_abyss_challenge_center.pair.unwrap().old +1 == flag_abyss_challenge_center.pair.unwrap().current {
                      print_message("Split Abyss Challenge Center");
                      timer::split()
                    }
                  }
                }

                // Magic
                {
                  if settings.arcane_level_2 && magic_arcane.pair.unwrap().changed_to(&2) {
                    print_message("Split Arcane 2");
                    timer::split()
                  }
                  if settings.ice_level_1 && magic_ice.pair.unwrap().changed_to(&1) {
                    print_message("Split Ice 1");
                    timer::split()
                  }
                  if settings.fire_level_1 && magic_fire.pair.unwrap().changed_to(&1) {
                    print_message("Split Fire 1");
                    timer::split()
                  }
                  if settings.thunder_level_1 && magic_thunder.pair.unwrap().changed_to(&1) {
                    print_message("Split Thunder 1");
                    timer::split()
                  }
                  if settings.absorption_level_1 && magic_absorption.pair.unwrap().changed_to(&1) {
                    print_message("Split Absorption 1");
                    timer::split()
                  }
                  if settings.wind_level_1 && magic_wind.pair.unwrap().changed_to(&1) {
                    print_message("Split Wind 1");
                    timer::split()
                  }
                }

                // Items
                {
                  if settings.any_item && item_count > max_items {
                    max_items = item_count;
                    print_message("Split Any Item");
                    timer::split()
                  } else if settings.last_item && item_count == 103 && !last_item {
                    last_item = true;
                    print_message("Split Last Item");
                    timer::split()
                  }
                  if item_count > max_items { max_items = item_count }

                  if settings.treasure_chest && chest_count.pair.unwrap().old +1 == chest_count.pair.unwrap().current {
                    print_message("Split Chest");
                    timer::split()
                  }
                }
              } else {
                // Trial Tower
                if version {
                  // Trial Tower Timer Start Splits
                  {
                    if settings.split_on_timer_start && !tt_dead {
                      if tt_armor_time.pair.unwrap().old == 0f32 && tt_armor_time.pair.unwrap().increased() {
                        print_message("Split TT Armor start");
                        timer::split()
                      }
                      if tt_tania_time.pair.unwrap().old == 0f32 && tt_tania_time.pair.unwrap().increased() {
                        print_message("Split TT Tania start");
                        timer::split()
                      }
                      if tt_monica_time.pair.unwrap().old == 0f32 && tt_monica_time.pair.unwrap().increased() {
                        print_message("Split TT Monica start");
                        timer::split()
                      }
                      if tt_vanessa_1_time.pair.unwrap().old == 0f32 && tt_vanessa_1_time.pair.unwrap().increased() {
                        print_message("Split TT Vanessa 1 start");
                        timer::split()
                      }
                      if tt_vanessa_2_time.pair.unwrap().old == 0f32 && tt_vanessa_2_time.pair.unwrap().increased() {
                        print_message("Split TT Vanessa 2 start");
                        timer::split()
                      }
                      if tt_nonota_time.pair.unwrap().old == 0f32 && tt_nonota_time.pair.unwrap().increased() {
                        print_message("Split TT Nonota start");
                        timer::split()
                      }
                      if tt_knight_time.pair.unwrap().old == 0f32 && tt_knight_time.pair.unwrap().increased() {
                        print_message("Split TT Knight start");
                        timer::split()
                      }
                      if tt_seal_time.pair.unwrap().old == 0f32 && tt_seal_time.pair.unwrap().increased() {
                        print_message("Split TT Seal start");
                        timer::split()
                      }
                    }
                    if settings.armor && tt_armor.pair.unwrap().old +1 == tt_armor.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Armor Dead");
                      timer::split()
                    }
                    if settings.tania && tt_tania.pair.unwrap().old +1 == tt_tania.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Tania Dead");
                      timer::split()
                    }
                    if settings.monica_1 && tt_monica_1.pair.unwrap().old +1 == tt_monica_1.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Monica 1 Dead");
                      timer::split()
                    }
                    if settings.monica_2 && tt_monica_2.pair.unwrap().old +1 == tt_monica_2.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Monica 2 Dead");
                      timer::split()
                    }
                    if settings.knight && tt_knight.pair.unwrap().old +1 == tt_knight.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Knight Dead");
                      timer::split()
                    }
                    if settings.vanessa_1 && tt_vanessa_1.pair.unwrap().old +1 == tt_vanessa_1.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Vanessa 1 Dead");
                      timer::split()
                    }
                    if settings.vanessa_2 && tt_vanessa_2.pair.unwrap().old +1 == tt_vanessa_2.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Vanessa 2 Dead");
                      timer::split()
                    }
                    if settings.seal_1 && tt_seal_1.pair.unwrap().old +1 == tt_seal_1.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Seal 1 Dead");
                      timer::split()
                    }
                    if settings.seal_2 && tt_seal_2.pair.unwrap().old +1 == tt_seal_2.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Seal 2 Dead");
                      timer::split()
                    }
                    if settings.nonota && tt_nonota.pair.unwrap().old +1 == tt_nonota.pair.unwrap().current {
                      tt_dead = false;
                      print_message("Split TT Nonota Dead");
                      timer::split()
                    }
                  }
                }
              }
            }
          }
          if player_is_dead.pair.is_some() {
            if player_is_dead.pair.unwrap().changed_to(&true) {
              print_message("Dead.")
            }
          }
          next_tick().await;
        }
      }).await;
  }
}

fn wstring_to_string(input: ArrayWString<128>) -> ArrayString<128> {
  let mut i = 0;
  let mut out:ArrayString<128> = ArrayString::new();
  while i < input.len() {
    if let Ok(next) = core::str::from_utf8(&[(input[i] & 0xFF) as u8]) {
      out.push_str(next);
    }
    i = i+1;
  }
  out
}