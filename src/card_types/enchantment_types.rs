use strum_macros::EnumString;

/// All the enchantment subtypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EnchantmentType {
    Aura,
    Cartouche,
    Curse,
    Rune,
    Saga,
    Shard,
    Shrine,
}