use strum_macros::{EnumString, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum AbilityKeyword {
    Flying,
    Mentor
}
