#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellType {
    Adventure,
    Arcane,
    Chorus,
    Lesson,
    Omen,
    Trap,
}
impl std::str::FromStr for SpellType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adventure" => Ok(Self::Adventure),
            "arcane" => Ok(Self::Arcane),
            "chorus" => Ok(Self::Chorus),
            "lesson" => Ok(Self::Lesson),
            "omen" => Ok(Self::Omen),
            "trap" => Ok(Self::Trap),
            other => Err(format!("Unknown SpellType: {}", other.to_string())),
        }
    }
}
impl SpellType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Adventure => "adventure",
            Self::Arcane => "arcane",
            Self::Chorus => "chorus",
            Self::Lesson => "lesson",
            Self::Omen => "omen",
            Self::Trap => "trap",
        }
    }
}
impl std::fmt::Display for SpellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl SpellType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Adventure,
            Self::Arcane,
            Self::Chorus,
            Self::Lesson,
            Self::Omen,
            Self::Trap,
        ]
        .into_iter()
    }
}
