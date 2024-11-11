#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellType {
    Adventure,
    Arcane,
    Chorus,
    Lesson,
    Trap,
}

impl std::str::FromStr for SpellType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Adventure" => Ok(Self::Adventure),
            "Arcane" => Ok(Self::Arcane),
            "Chorus" => Ok(Self::Chorus),
            "Lesson" => Ok(Self::Lesson),
            "Trap" => Ok(Self::Trap),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for SpellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Adventure => write!(f, "Adventure"),
            Self::Arcane => write!(f, "Arcane"),
            Self::Chorus => write!(f, "Chorus"),
            Self::Lesson => write!(f, "Lesson"),
            Self::Trap => write!(f, "Trap"),
        }
    }
}

impl SpellType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Adventure,
            Self::Arcane,
            Self::Chorus,
            Self::Lesson,
            Self::Trap,
        ].into_iter()
    }
}
