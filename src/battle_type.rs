#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleType {
    Siege,
}
impl std::str::FromStr for BattleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "siege" => Ok(Self::Siege),
            other => Err(format!("Unknown BattleType: {}", other.to_string())),
        }
    }
}
impl BattleType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Siege => "siege",
        }
    }
}
impl std::fmt::Display for BattleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl BattleType {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Siege].into_iter()
    }
}
