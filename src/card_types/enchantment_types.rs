use strum_macros::EnumString;

/// All the enchantment subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum EnchantmentType {
    Aura,
    Cartouche,
    Curse,
    Rune,
    Saga,
    Shard,
    Shrine,
}