// this lib gives access to different magic: the gathering data, in a rust-like format.

mod abilities;
mod card_types;
mod color;
mod format;
mod legalities;
mod mana_cost;

pub use abilities::AbilityKeyword;
pub use card_types::{
    Type,
    artifact_types::ArtifactType,
    battle_types::BattleType,
    creature_types::CreatureType,
    enchantment_types::EnchantmentType,
    land_types::LandType,
    planeswalker_types::PlaneswalkerType,
    spell_type::SpellType,
    super_types::SuperType,
};
pub use color::MtgColor;
pub use format::Format;
pub use legalities::Legality;
pub use mana_cost::ManaCost;