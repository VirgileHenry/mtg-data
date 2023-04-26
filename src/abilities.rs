use strum_macros::{EnumString, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum AbilityKeyword {
    Flying,
    Mentor,
    Rebound,
}
