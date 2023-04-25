use strum_macros::EnumString;

/// All the artifact subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ArtifactType {
    Clue,
    Equipment,
    Food,
    Gold,
    Treasure,
    Vehicle,
    Contraption,
    Fortification,
}