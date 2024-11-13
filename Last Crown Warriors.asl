state("gambatte_qt") {}
state("gambatte") {}
state("gambatte_speedrun") {}

init { 
    version = "Made for 0.7a Update 2";
}

startup {
    settings.Add("plains_1", true, "Sudden Ambush");
    settings.Add("plains_2", true, "Forrest Conquest");
    settings.Add("plains_3", true, "Perilous Path");
    settings.Add("mountains_1", true, "A Slyde Detour");
    settings.Add("mountains_2", true, "Pottery Peak");
    settings.Add("mountains_3", true, "Dimlit Truancy");
    


    Assembly.Load(File.ReadAllBytes("Components/emu-help-v2")).CreateInstance("GBC");
    vars.Helper.Load = (Func<dynamic, bool>)(emu =>
    {
        emu.Make<byte>("level", 0x132D);
        emu.Make<byte>("game_State", 0x0318); 
        /* 
            0x01 = Title Screen,
            0x05 = Game Over,
            0x06 = Victory,
            0x08 = How to Play
            0x0D = Intro Cutscene 
            0x0F = Cutscene Sideview Plains
            0x14 = Pre Game Credits "Potatoteto"
            0x16 = Settings
            0x19 = Cutscene Sideview Mountains
            0x1C = Map
            0x20 = Cutscene Top Down
            0x24 = Level Plains
            0x3C = Level Mountains 2
            0x4B = Pause
            0x4D = Pre Game Settings
        */
        return true;
    });
}
start {
    return current.game_State == 0x20 && old.game_State == 0x01;
}
split {
    if (current.game_State != old.game_State && current.game_State == 0x06)
    {
        print("Split");
        if (current.level == 0x01) return settings["plains_1"];
        if (current.level == 0x03) return settings["plains_2"];
        if (current.level == 0x02) return settings["plains_3"];
        if (current.level == 0x06) return settings["mountains_1"];
        if (current.level == 0x08) return settings["mountains_2"];
        if (current.level == 0x07) return settings["mountains_3"];
    }
}
reset {
    return current.game_State == 0x14;
}