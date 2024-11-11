#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EnchantmentType {
    Aura,
    Background,
    Cartouche,
    Case,
    Class,
    Curse,
    Role,
    Room,
    Rune,
    Saga,
    Shard,
    Shrine,
}

impl std::str::FromStr for EnchantmentType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Aura" => Ok(Self::Aura),
            "Background" => Ok(Self::Background),
            "Cartouche" => Ok(Self::Cartouche),
            "Case" => Ok(Self::Case),
            "Class" => Ok(Self::Class),
            "Curse" => Ok(Self::Curse),
            "Role" => Ok(Self::Role),
            "Room" => Ok(Self::Room),
            "Rune" => Ok(Self::Rune),
            "Saga" => Ok(Self::Saga),
            "Shard" => Ok(Self::Shard),
            "Shrine" => Ok(Self::Shrine),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for EnchantmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Aura => write!(f, "Aura"),
            Self::Background => write!(f, "Background"),
            Self::Cartouche => write!(f, "Cartouche"),
            Self::Case => write!(f, "Case"),
            Self::Class => write!(f, "Class"),
            Self::Curse => write!(f, "Curse"),
            Self::Role => write!(f, "Role"),
            Self::Room => write!(f, "Room"),
            Self::Rune => write!(f, "Rune"),
            Self::Saga => write!(f, "Saga"),
            Self::Shard => write!(f, "Shard"),
            Self::Shrine => write!(f, "Shrine"),
        }
    }
}

impl EnchantmentType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Aura,
            Self::Background,
            Self::Cartouche,
            Self::Case,
            Self::Class,
            Self::Curse,
            Self::Role,
            Self::Room,
            Self::Rune,
            Self::Saga,
            Self::Shard,
            Self::Shrine,
        ].into_iter()
    }
}
