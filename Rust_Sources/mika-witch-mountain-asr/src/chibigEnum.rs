#![no_std]
use alloc::fmt;
use num_enum::{
    TryFromPrimitive,
    IntoPrimitive
};


pub enum PackageEnum {
    None = 0,
    OVelPackage = 1,
    Lunch = 2,
    Fish = 3,
    WrongPackage = 4,
    BrramPackage = 5,
    LemonIceCream = 6,
    Lantern = 10,
    GlowingMushrooms = 11,
    MushroomAnalysis = 12,
    ElectricalBill = 13,
    DroneReceipt = 14,
    Drone = 15,
    LetterForSon = 16,
    LetterForMother = 17,
    PinkAlgae = 18,
    MaraIceCream = 19,
    SparkPlug = 30,
    Vase = 40,
    Coal = 41,
    Canary = 42,
    ThankYouLetter = 43,
    MillPiece = 44,
    Astrolabe = 45,
    FirstAidKit = 46,
    CellarKey = 47,
    Radio = 48,
    Pendulum = 49,
    Flask = 50,
    Violin = 51,
    Invitation = 52,
    Painting = 53,
    SeaArtifact = 54,
    LemonIceCreamParty = 70,
    MaraIceCreamParty = 71,
    ChocolateIceCreamParty = 72,
    MintIceCreamParty = 73,
    LostWatch = 100,
    Flute = 101,
    Amphora = 102,
    GiantEgg = 103,
    FamilyPhoto = 104,
    HotTeapot = 105,
    MountainFruit = 106,
    Television = 107,
    FarmInvoice = 108,
    Kite1 = 109,
    Kite2 = 110,
    Kite3 = 111,
    OstrichChick1 = 112,
    OstrichChick2 = 113,
    OstrichChick3 = 114,
    Crystal = 115,
    Kappa1 = 116,
    Kappa2 = 117,
    Kappa3 = 118,
    Sponge = 206,
    FishTank = 200,
    FishTankWater = 201,
    FishTankFish = 202,
    FishTankKappa = 203,
    SeaArtifact2 = 204,
    WindArtifact = 205,
    BackerPackage = 300,
    NewPainting = 301,
    OldPainting = 302,
    Count = 303,
}

pub enum PackageStateEnum
{
    NotSpawned,
    SpawnedButNotAccepted,
    KnowPackage,
    Accepted,
    Delivered
}
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ActEnum
{
    None = -1,
    Intro,
    Act1,
    Act2,
    Interlude,
    Act3,
    Party,
    Climb,
    PostGame
}
impl fmt::Display for ActEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActEnum::None => write!(f, "None"),
            ActEnum::Intro => write!(f, "Intro"),
            ActEnum::Act1 => write!(f, "Act1"),
            ActEnum::Act2 => write!(f, "Act2"),
            ActEnum::Interlude => write!(f, "Interlude"),
            ActEnum::Act3 => write!(f, "Act3"),
            ActEnum::Party => write!(f, "Party"),
            ActEnum::Climb => write!(f, "Climb"),
            ActEnum::PostGame => write!(f, "PostGame"),
            _ => write!(f, "Unknown")
        }
    }
}