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
    type Err = String;
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
            other => Err(format!("Unknown EnchantmentType: {}", other.to_string())),
        }
    }
}
impl EnchantmentType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Aura => "Aura",
            Self::Background => "Background",
            Self::Cartouche => "Cartouche",
            Self::Case => "Case",
            Self::Class => "Class",
            Self::Curse => "Curse",
            Self::Role => "Role",
            Self::Room => "Room",
            Self::Rune => "Rune",
            Self::Saga => "Saga",
            Self::Shard => "Shard",
            Self::Shrine => "Shrine",
        }
    }
}
impl std::fmt::Display for EnchantmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl EnchantmentType {
    pub fn all() -> impl Iterator<Item = Self> {
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
        ]
        .into_iter()
    }
}
