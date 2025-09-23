//#![no_std]
#![allow(non_snake_case)]
extern crate alloc;

mod chibigEnum;

use alloc::string::ToString;
use asr::{
    future::{
        next_tick
    },
    settings::{
        Gui,
        gui::{
            Title,
            Widget
        }
    },
    Process,
    game_engine::unity::{
        mono::Module
    },
    print_message,
    PointerSize,
    timer::{
        start,
        split,
        reset,
        set_variable,
        state,
        TimerState
    },
    watcher::Watcher
};
use crate::chibigEnum::{ActEnum, PackageStateEnum};

asr::async_main!(stable);
//asr::panic_handler!();

#[derive(Gui)]
struct Settings {
    #[default = true] start: bool,
    #[default = true] reset: bool,

    #[heading_level = 0]
    overwritting_settings: Title,
    #[default = false] all_acts: bool,
    //#[default = false] all_packages: bool,




    act_finish: Title,
    /// Intro
    #[default = false] act_intro: bool,
    #[default = true] act_1: bool,
    #[default = true] act_2: bool,
    /// Interlude
    #[default = true] act_interlude: bool,
    #[default = true] act_3: bool,
    /// Party
    #[default = true] act_party: bool,
    /// Climb
    #[default = true] act_climb: bool,

    /*packages_act_1: Title,
    /// Fragile Package
    #[default = false] pack_1_fragile: bool,
    /// Sailor's Lunch
    #[default = false] pack_1_lunch: bool,
    /// Fish Bowl with Minnow
    #[default = false] pack_1_fish: bool,
    /// Returned Package
    #[default = false] pack_1_return: bool,
    /// Slightly Damaged Package
    #[default = false] pack_1_damaged: bool,
    /// Lemon Ice Cream
    #[default = false] pack_1_ice: bool,
    packages_act_2: Title,
    packages_act_interlude: Title,
    packages_act_3: Title,
    packages_party: Title,
    packages_extra: Title,*/
}
async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    let mut watch_Act = Watcher::new();
    let mut watch_flagCount = Watcher::new();

    let mut watch_Packages: [Watcher<bool>;303] = [Watcher::new();303];

    set_variable("Debug: Last Split", "");

    print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("Mika.exe").await;
        process
            .until_closes(async {
                let module = Module::wait_attach_auto_detect(&process).await;
                let image_ChibigMika = module.wait_get_image(&process,"Chibig.Mika").await;
                print_message("Got Image for Chibig.Mika");
                let class_GameService = image_ChibigMika.wait_get_class(&process,&module,"GameService").await;
                print_message("Got Class GameService");
                let class_DataService = image_ChibigMika.wait_get_class(&process,&module,"DataService").await;
                print_message("Got Class DataService");
                let class_GameSaveData = image_ChibigMika.wait_get_class(&process,&module,"GameSaveData").await;
                print_message("Got GameSaveData Struct");
                let class_PackageState = image_ChibigMika.wait_get_class(&process,&module,"PackageState").await;
                print_message("Got PackageState Struct");


                let offset_GS_DataService = class_GameService.wait_get_field_offset(&process,&module,"<DataService>k__BackingField").await;
                print_message("Got Offset: GameService > DataService");
                let offset_DS_Data = class_DataService.wait_get_field_offset(&process,&module,"<Data>k__BackingField").await;
                print_message("Got Offset: DataService > Data");
                let offset_GSD_CurrentAct = class_GameSaveData.wait_get_field_offset(&process,&module,"currentAct").await;
                print_message("Got Offset: GameSaveData > CurrentAct");
                let offset_GSD_List_Packages = class_GameSaveData.wait_get_field_offset(&process,&module,"packages").await;
                print_message("Got Offset: GameSaveData > Packages");
                let offset_GSD_Flags = class_GameSaveData.wait_get_field_offset(&process,&module,"flags").await;
                print_message("Got Offset: GameSaveData > Flags");
                let offset_PS_Type = class_PackageState.wait_get_field_offset(&process,&module,"type").await;
                print_message("Got Offset: PackageState > Type");
                let offset_PS_State = class_PackageState.wait_get_field_offset(&process,&module,"state").await;
                print_message("Got Offset: PackageState > State");
                let static_GameService;
                {
                    let static_table = class_GameService.wait_get_static_table(&process,&module).await;
                    print_message("Got Static Table");
                    let field_offset = class_GameService.wait_get_field_offset(&process,&module,"<Instance>k__BackingField").await;
                    print_message("Got Static Instance Offset");

                    static_GameService = static_table.add(field_offset as u64);
                }

                let offset_list_Items:u64 = 16;
                let offset_list_Count:u64 = 24;


                //loop
                {
                    settings.update();

                    let instance_GameService = process.read_pointer(static_GameService, PointerSize::Bit64);

                    if instance_GameService.is_ok() {
                        let instance_GameService = instance_GameService.unwrap();
                        if let Ok(instance_DataService) =
                            process.read_pointer(instance_GameService.add(offset_GS_DataService as u64), PointerSize::Bit64)
                        {
                            if let Ok(instance_GameSaveData) =
                                process.read_pointer(instance_DataService.add(offset_DS_Data as u64), PointerSize::Bit64)
                            {
                                if let Ok(flag_list) = process.read_pointer(instance_GameSaveData.add(offset_GSD_Flags as u64), PointerSize::Bit64) {
                                    if let Ok(flag) = process.read::<i32>(flag_list.add(offset_list_Count)) {
                                        watch_flagCount.update_infallible(flag);
                                    }
                                }
                                if let Ok(act) = process.read::<i32>(instance_GameSaveData.add(offset_GSD_CurrentAct as u64)) {
                                    watch_Act.update_infallible(act);
                                }
                                if let Ok(package_list) = process.read_pointer(instance_GameSaveData.add(offset_GSD_List_Packages as u64), PointerSize::Bit64) {
                                    if let Ok(mut package_list_count) = process.read::<i32>(package_list.add(offset_list_Count as u64)) {
                                        print_message(("Count: ".to_string() + package_list_count.to_string().as_str()).as_str());
                                        if let Ok(packages) = process.read_pointer(package_list.add(offset_list_Items as u64), PointerSize::Bit64) {
                                            let mut index:u64 = 0;
                                            if let Ok(packaging) = process.read_pointer(packages, PointerSize::Bit64) {
                                                print_message(("Package Address: ".to_string() + packaging.to_string().as_str()).as_str());
                                                while package_list_count > 0 {
                                                    if let Ok(package) = process.read_pointer(packaging.add(index * 16), PointerSize::Bit64) {
                                                        let package_type = process.read::<i32>(package.add(offset_PS_Type as u64));
                                                        let package_state = process.read::<i32>(package.add(offset_PS_State as u64));
                                                        print_message("Test 2!!!");
                                                        if package_type.is_ok() && package_state.is_ok() {
                                                            let package_type = package_type.unwrap();
                                                            let package_state = package_state.unwrap();
                                                            let mut msg: String = "Package: ".to_string() + package_type.to_string().as_str();
                                                            msg += " ";
                                                            msg += &package_state.to_string().as_str();

                                                            print_message(msg.as_str());
                                                            if 0 <= package_type && package_type <= 303 {
                                                                if watch_Packages[package_type as usize].pair.unwrap().old == false {
                                                                    watch_Packages[package_type as usize].update_infallible(package_state == PackageStateEnum::Delivered as i32);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    package_list_count -= 1;
                                                    index += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }


                    // Split Logic
                    match state()
                    {
                        TimerState::Running =>
                            {
                                if watch_Act.pair.unwrap().increased() && (settings.all_acts
                                    || match watch_Act.pair.unwrap().old.try_into() {
                                        Ok(ActEnum::Intro) => settings.act_intro,
                                        Ok(ActEnum::Act1) => settings.act_1,
                                        Ok(ActEnum::Act2) => settings.act_2,
                                        Ok(ActEnum::Interlude) => settings.act_interlude,
                                        Ok(ActEnum::Act3) => settings.act_3,
                                        Ok(ActEnum::Party) => settings.act_party,
                                        Ok(ActEnum::Climb) => settings.act_climb,
                                        _ => false }
                                ) {
                                    split();
                                    let mut split = "Act Value: ".to_string() + watch_Act.pair.unwrap().old.to_string().as_str();
                                    split += ": With: ";
                                    split += if settings.all_acts { "All Acts" } else { "Specific Act" };

                                    set_variable("Debug: Last Split", split.as_str());
                                }
                                if watch_flagCount.pair.unwrap().changed_to(&0)
                                    && settings.reset
                                { reset() }
                            }
                        TimerState::NotRunning =>
                            {
                                if watch_flagCount.pair.unwrap().changed_from_to(&0,&1)
                                    && settings.start
                                { start() }
                            }
                        _ => {}
                    }

                    loop
                    { // TODO: Do something on every tick.
                        next_tick().await;
                    }
                }
            })
            .await;
    }
}