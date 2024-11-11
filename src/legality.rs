#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Legality {
    Legal,
    Notlegal,
    Restricted,
    Banned,
}

impl std::str::FromStr for Legality {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Legal" => Ok(Self::Legal),
            "NotLegal" => Ok(Self::Notlegal),
            "Restricted" => Ok(Self::Restricted),
            "Banned" => Ok(Self::Banned),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for Legality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Legal => write!(f, "Legal"),
            Self::Notlegal => write!(f, "NotLegal"),
            Self::Restricted => write!(f, "Restricted"),
            Self::Banned => write!(f, "Banned"),
        }
    }
}

impl Legality {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Legal,
            Self::Notlegal,
            Self::Restricted,
            Self::Banned,
        ].into_iter()
    }
}
