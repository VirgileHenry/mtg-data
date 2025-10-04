#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Legality {
    Legal,
    Notlegal,
    Restricted,
    Banned,
}
impl std::str::FromStr for Legality {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "legal" => Ok(Self::Legal),
            "notlegal" => Ok(Self::Notlegal),
            "restricted" => Ok(Self::Restricted),
            "banned" => Ok(Self::Banned),
            other => Err(format!("Unknown Legality: {}", other.to_string())),
        }
    }
}
impl Legality {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Legal => "legal",
            Self::Notlegal => "notlegal",
            Self::Restricted => "restricted",
            Self::Banned => "banned",
        }
    }
}
impl std::fmt::Display for Legality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Legality {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Legal, Self::Notlegal, Self::Restricted, Self::Banned].into_iter()
    }
}
