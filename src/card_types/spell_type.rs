use strum_macros::EnumString;

/// All the spells subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Trap,
}