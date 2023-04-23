use strum_macros::EnumString;

/// All the spells subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Trap,
}