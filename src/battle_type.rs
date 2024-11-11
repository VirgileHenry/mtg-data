#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleType {
    Siege,
}

impl std::str::FromStr for BattleType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Siege" => Ok(Self::Siege),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for BattleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Siege => write!(f, "Siege"),
        }
    }
}

impl BattleType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Siege,
        ].into_iter()
    }
}
