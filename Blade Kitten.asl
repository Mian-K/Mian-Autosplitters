state ("BladeKitten")
{
    int levelID : "bladekitten.exe", 0x00729C18, 0x18;      // Start at 0 in order
    int gameState : "bladekitten.exe", 0x6BEFD4;            // 0 = Load, 1 = Title Screen, 2 = Level, 3 = Pause
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
    return current.levelID != old.levelID && settings["level_" + old.levelID];    
}
isLoading
{
    return current.gameState == 0;
}
start
{
    return old.gameState == 1 && current.gameState == 0;
}
reset
{
    return old.gameState == 1 && current.gameState == 1;
}   
