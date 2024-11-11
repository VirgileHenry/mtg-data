#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Supertype {
    Basic,
    Elite,
    Legendary,
    Ongoing,
    Snow,
    Token,
    World,
}

impl std::str::FromStr for Supertype {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic" => Ok(Self::Basic),
            "Elite" => Ok(Self::Elite),
            "Legendary" => Ok(Self::Legendary),
            "Ongoing" => Ok(Self::Ongoing),
            "Snow" => Ok(Self::Snow),
            "Token" => Ok(Self::Token),
            "World" => Ok(Self::World),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for Supertype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Basic => write!(f, "Basic"),
            Self::Elite => write!(f, "Elite"),
            Self::Legendary => write!(f, "Legendary"),
            Self::Ongoing => write!(f, "Ongoing"),
            Self::Snow => write!(f, "Snow"),
            Self::Token => write!(f, "Token"),
            Self::World => write!(f, "World"),
        }
    }
}

impl Supertype {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Basic,
            Self::Elite,
            Self::Legendary,
            Self::Ongoing,
            Self::Snow,
            Self::Token,
            Self::World,
        ].into_iter()
    }
}
