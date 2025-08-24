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
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic" => Ok(Self::Basic),
            "Elite" => Ok(Self::Elite),
            "Legendary" => Ok(Self::Legendary),
            "Ongoing" => Ok(Self::Ongoing),
            "Snow" => Ok(Self::Snow),
            "Token" => Ok(Self::Token),
            "World" => Ok(Self::World),
            other => Err(format!("Unknown Supertype: {}", other.to_string())),
        }
    }
}
impl Supertype {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Elite => "Elite",
            Self::Legendary => "Legendary",
            Self::Ongoing => "Ongoing",
            Self::Snow => "Snow",
            Self::Token => "Token",
            Self::World => "World",
        }
    }
}
impl std::fmt::Display for Supertype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Supertype {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Basic,
            Self::Elite,
            Self::Legendary,
            Self::Ongoing,
            Self::Snow,
            Self::Token,
            Self::World,
        ]
        .into_iter()
    }
}
