use strum_macros::{EnumString, EnumIter};

/// All colors in magic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum MtgColor {
    Red, // R
    Blue, // U
    Green, // G
    White, // W
    Black, // B
    Colorless, // C
}