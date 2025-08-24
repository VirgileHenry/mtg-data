// this lib gives access to different magic: the gathering data, in a rust-like format.

mod artifact_type;
mod battle_type;
mod card_type;
mod color;
mod creature_type;
mod enchantment_type;
mod format;
mod keyword_ability;
mod keyword_action;
mod land_type;
mod legality;
mod mana_cost;
mod planeswalker_type;
mod spell_type;
mod supertype;

pub use artifact_type::ArtifactType;
pub use battle_type::BattleType;
pub use card_type::CardType;
pub use color::Color;
pub use creature_type::CreatureType;
pub use enchantment_type::EnchantmentType;
pub use format::Format;
pub use keyword_ability::KeywordAbility;
pub use keyword_action::KeywordAction;
pub use land_type::LandType;
pub use legality::Legality;
pub use mana_cost::Mana;
pub use planeswalker_type::PlaneswalkerType;
pub use spell_type::SpellType;
pub use supertype::Supertype;
