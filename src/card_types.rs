use strum_macros::EnumString;

/// All the card types in Mtg.
/// A card can have muliple type, a super type, multiple sub types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Type {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
    Tribal,
    Conspiracy,
    Phenomenon,
    Scheme,
    Vanguard,
    Dungeon,
    Battle,
}

pub mod artifact_types;
pub mod battle_types;
pub mod creature_types;
pub mod enchantment_types;
pub mod land_types;
pub mod planeswalker_types;
pub mod spell_type;
pub mod super_types;