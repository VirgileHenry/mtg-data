#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Black,
    Blue,
    Colorless,
    Green,
    Red,
    White,
}

impl std::str::FromStr for Color {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Self::Black),
            "blue" => Ok(Self::Blue),
            "colorless" => Ok(Self::Colorless),
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            "white" => Ok(Self::White),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Black => write!(f, "black"),
            Self::Blue => write!(f, "blue"),
            Self::Colorless => write!(f, "colorless"),
            Self::Green => write!(f, "green"),
            Self::Red => write!(f, "red"),
            Self::White => write!(f, "white"),
        }
    }
}

impl Color {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Black,
            Self::Blue,
            Self::Colorless,
            Self::Green,
            Self::Red,
            Self::White,
        ].into_iter()
    }
}
