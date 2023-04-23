use strum_macros::EnumString;

/// All the supertypes in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SuperType {
    Basic,
    Legendary,
    Snow,
    World,
    Ongoing,
}