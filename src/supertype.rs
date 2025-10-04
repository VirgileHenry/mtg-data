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
            "basic" => Ok(Self::Basic),
            "elite" => Ok(Self::Elite),
            "legendary" => Ok(Self::Legendary),
            "ongoing" => Ok(Self::Ongoing),
            "snow" => Ok(Self::Snow),
            "token" => Ok(Self::Token),
            "world" => Ok(Self::World),
            other => Err(format!("Unknown Supertype: {}", other.to_string())),
        }
    }
}
impl Supertype {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Elite => "elite",
            Self::Legendary => "legendary",
            Self::Ongoing => "ongoing",
            Self::Snow => "snow",
            Self::Token => "token",
            Self::World => "world",
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
