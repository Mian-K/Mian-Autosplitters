state ("NyxQuest")
{
    string19 LevelName : "NyxQuest.exe", 0x002CF0E0, 0x18, 0x18, 0x0, 0x18, 0x0, 0xB4, 0x10;
    float posX : "NyxQuest.exe", 0x002CF0E0, 0x20, 0x4, 0x1c, 0x0, 0xc, 0x0, 0x40;
    float posZ : "NyxQuest.exe", 0x002CF0E0, 0x20, 0x4, 0x1c, 0x0, 0xc, 0x0, 0x44;
    uint totalTimer : "NyxQuest.exe", 0x002CF0E0, 0x50;
    float levelTimerDecimal : "NyxQuest.exe", 0x002CF0E0, 0x60;
    uint levelTimerSeconds : "NyxQuest.exe", 0x002CF0E0, 0x64;
}
startup
{
    settings.Add("Splits", true);
    settings.Add("level_1",  true, "Ruins of Corinth", "Splits");         // "Levels/E1/L1.txt"
    settings.Add("level_2",  true, "Valley of the Temples", "Splits");    // "Levels/E1/L2.txt"
    settings.Add("level_3",  true, "The Dry Swamp", "Splits");            // "Levels/E1/L3.txt"
    settings.Add("level_4",  true, "Olympia", "Splits");                  // "Levels/E1/L4.txt"
    settings.Add("level_5",  true, "Aeolia", "Splits");                   // "Levels/E1/L5.txt"
    settings.Add("level_6",  true, "Temple of Aeolia", "Splits");         // "Levels/E1/L6.txt"
    settings.Add("level_7",  true, "Thessaly", "Splits");                 // "Levels/E1/L7.txt"
    settings.Add("level_8",  true, "The Forgotten Passages", "Splits");   // "Levels/E1/L8.txt"
    settings.Add("level_9",  true, "Fields of Argos", "Splits");          // "Levels/E1/L9.txt"
    settings.Add("level_10", true, "Oracle of Delphi", "Splits");         // "Levels/E1/L10.txt"
    settings.Add("level_11", true, "Mount Parnassus", "Splits");          // "Levels/E1/L11.txt"
    settings.Add("level_12", true, "Mount Parnassus ||", "Splits");       // "Levels/E1/L12.txt"
    //settings.Add("level_13", true, "Arcadia", "Splits");                  // "Levels/E1/L99.txt"
    settings.Add("Settings", true);
    settings.Add("100_no_reset", true, "No Reset after Mount Parnassus ||", "Settings");
    //settings.Add("arcadia_il", false, "Arcadia IL", "Settings");
    settings.Add("auto_il", true, "Automatic IL Run Detection", "Settings");
    settings.Add("always_il", false, "Always use IL Timer", "Settings");

    settings.SetToolTip("auto_il", "Automatically detects whether you start an IL or Full Game run\nand switches to using the Level or Total Timer accordingly.");
    settings.SetToolTip("always_il", "Always use the IL Timer.\nThis overwrites the Automatic IL Run Detection.\nDisable both to always use the Total Timer.");
    vars.post_Game = false;
    vars.TimerModel = new TimerModel { CurrentState = timer };
    vars.test = 99;
}

init
{
    if (timer.CurrentTimingMethod == TimingMethod.RealTime) // stolen from dude simulator 3, basically asks the runner to set their livesplit to game time
    {
        var timingMessage = MessageBox.Show (
               "This game has the Option to use Time without Loads (Game Time) as the timing method.\n"+
                "LiveSplit is currently set to show Real Time (RTA).\n"+
                "Would you like to set the timing method to Game Time? This will make verification easier",
                "LiveSplit | NyxQuest: Kindred Spirits",
               MessageBoxButtons.YesNo,MessageBoxIcon.Question
        );

        if (timingMessage == DialogResult.Yes)
        {
            timer.CurrentTimingMethod = TimingMethod.GameTime;
        }
    }
}

start
{
    vars.post_Game = false;
    if (current.LevelName != old.LevelName)
    {
        return new List<string>()
        {
            "Levels/E1/L1.txt",
            "Levels/E1/L2.txt",
            "Levels/E1/L3.txt",
            "Levels/E1/L4.txt",
            "Levels/E1/L5.txt",
            "Levels/E1/L6.txt",
            "Levels/E1/L7.txt",
            "Levels/E1/L8.txt",
            "Levels/E1/L9.txt",
            "Levels/E1/L10.txt",
            "Levels/E1/L11.txt",
            "Levels/E1/L12.txt",
            "Levels/E1/L99.txt"
        }.Contains(current.LevelName);
    }
}

onStart
{
    timer.IsGameTimePaused = true;
    vars.auto_il = current.totalTimer > current.levelTimerSeconds;
}

update
{
    current.convertedLevelTimer = (double)current.levelTimerDecimal + (double)current.levelTimerSeconds;

}

split
{
    float end = 813.6279f;
    if (current.LevelName == "Levels/E1/L12.txt" && settings["level_12"] && current.posX >= end && old.posX < end)
    {   //Didn't find a better way then just splitting on position for the final level, don't jump over lol
        timer.IsGameTimePaused = true;
        timer.SetGameTime(TimeSpan.FromSeconds(18 + ((vars.auto_il && settings["auto_il"]) || settings["always_il"]) ? (double)current.totalTimer : current.convertedLevelTimer));
        vars.post_Game = true;
        return true;
    }

    if (current.LevelName != old.LevelName)
    {
        timer.IsGameTimePaused = true;
        timer.SetGameTime(TimeSpan.FromSeconds(((vars.auto_il && settings["auto_il"]) || settings["always_il"]) ? (double)current.totalTimer : old.convertedLevelTimer));
        return (current.LevelName == "Levels/E1/L2.txt"  && settings["level_1"])
            || (current.LevelName == "Levels/E1/L3.txt"  && settings["level_2"])
            || (current.LevelName == "Levels/E1/L4.txt"  && settings["level_3"])
            || (current.LevelName == "Levels/E1/L5.txt"  && settings["level_4"])
            || (current.LevelName == "Levels/E1/L6.txt"  && settings["level_5"])
            || (current.LevelName == "Levels/E1/L7.txt"  && settings["level_6"])
            || (current.LevelName == "Levels/E1/L8.txt"  && settings["level_7"])
            || (current.LevelName == "Levels/E1/L9.txt"  && settings["level_8"])
            || (current.LevelName == "Levels/E1/L10.txt" && settings["level_9"])
            || (current.LevelName == "Levels/E1/L11.txt" && settings["level_10"])
            || (current.LevelName == "Levels/E1/L12.txt" && settings["level_11"]);
    }
}

isLoading
{
    if (current.convertedLevelTimer < old.convertedLevelTimer)
    {
        timer.SetGameTime(new TimeSpan(0,0,(int)current.totalTimer));
        return true;
    }

    return current.convertedLevelTimer == old.convertedLevelTimer;
}

reset
{
    return !(settings["100_no_reset"] && vars.post_Game) && !settings["arcadia_il"] && current.LevelName == "Levels/MainMenu.txt";
}

exit
{
	timer.IsGameTimePaused = true;
}
