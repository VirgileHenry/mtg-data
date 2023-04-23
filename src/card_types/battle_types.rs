use strum_macros::EnumString;

/// All the battle subtypes in Mtg. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BattleType {
    Siege,
}