use strum_macros::EnumString;

/// All the land subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum LandType {
    Plains,
    Island,
    Mountain,
    Swamp,
    Forest,
    Desert,
    Gate,
    Lair,
    Locus,
    Mine,
    PowerPlant,
    Tower,
    Urzas,
}