use strum_macros::EnumString;

/// All the land subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
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