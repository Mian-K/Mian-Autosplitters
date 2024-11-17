state ("BladeKitten")
{
    int levelID             : "bladekitten.exe", 0x00729C18, 0x18;      // Start at 0 in order
    int gameState           : "bladekitten.exe", 0x006BEFD4;            // 0 = Load, 1 = Title Screen, 2 = Level, 3 = Pause, 4 Special Cutscene
    float healthDreadnaught : "bladekitten.exe", 0x00729C18, 0x48, 0x68, 0x1A0, 0x190, 0x6C0;
    int gameMenu            : "bladekitten.exe", 0x006AEB48;            // 0 = Ingame, 2 = MainMenu
}

startup
{
    settings.Add("Chapter 1", true);
    settings.Add("level_0", true, "War on Terra", "Chapter 1");
    settings.Add("level_1", true, "Skiff Happens", "Chapter 1");
    settings.Add("level_2", true, "Chasing Justice", "Chapter 1");
    settings.Add("level_3", true, "Urb Ex", "Chapter 1");
    settings.Add("level_4", true, "Exodus", "Chapter 1");
    settings.Add("level_5", true, "Collaborator", "Chapter 1");
    settings.Add("level_6", true, "Acland", "Chapter 1");
    settings.Add("level_7", true, "Darque Nights", "Chapter 1");
    settings.Add("level_8", true, "Berserk", "Chapter 1");
    settings.Add("level_9", true, "Sisterhood", "Chapter 1");
    settings.Add("level_10", true, "Vor Hunting", "Chapter 1");
    settings.Add("level_11", true, "Hidden Fortress", "Chapter 1");
    settings.Add("level_12", true, "Dreadnaught", "Chapter 1");
    settings.Add("level_12_as_end", true, "No DLC Split Timing?", "level_12");
    settings.SetToolTip("level_12_as_end", "For Any% no DLC timing, set this to true.\nThis will make the split trigger at the final hit instead of during level transition.");
    
    settings.Add("Chapter 2", true);
    settings.Add("level_13", true, "Awakenings", "Chapter 2");
    settings.Add("level_14", true, "Siege Mentality", "Chapter 2");
    settings.Add("level_15", true, "Incursion", "Chapter 2");
    settings.Add("level_16", true, "Excursion", "Chapter 2");
    settings.Add("level_17", true, "Revelations", "Chapter 2");
    settings.Add("level_18", true, "Hybrid", "Chapter 2");
}
split
{
    if (timer.CurrentTime.GameTime <= new TimeSpan(0,0,1)) return false;
    
    if (old.levelID == 12 && settings["level_12_as_end"])
        return current.healthDreadnaught == 0 && old.healthDreadnaught > 0 && settings["level_12"];
    else if (old.levelID == 18)
        return current.gameState == 4 && settings["level_18"];
    else return current.levelID != old.levelID && settings["level_" + old.levelID];    
}
isLoading
{
    // Load Removed Time seems to be slightly more compared to the in Game Timer, probably due to Cutscenes.
    // Can't find the actual IGT address, so we'll just use the Load Removed Time for now.
    // Changing the Leaderboard to LRT is not recommended unless there starts being a lot more interest in the game.
    return current.gameState == 0;
}
start
{
    return old.gameState == 1 && current.gameState == 0;
}
onStart
{
    timer.IsGameTimePaused = true;
    timer.SetGameTime(TimeSpan.FromSeconds(0));
}
reset
{
    return current.gameMenu == 2;
}
