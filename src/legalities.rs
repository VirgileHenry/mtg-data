use strum_macros::EnumString;

/// All legalities cards can have in Mtg.
/// They are generally associated with a format for a single card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}