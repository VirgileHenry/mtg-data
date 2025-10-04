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
            "aura" => Ok(Self::Aura),
            "background" => Ok(Self::Background),
            "cartouche" => Ok(Self::Cartouche),
            "case" => Ok(Self::Case),
            "class" => Ok(Self::Class),
            "curse" => Ok(Self::Curse),
            "role" => Ok(Self::Role),
            "room" => Ok(Self::Room),
            "rune" => Ok(Self::Rune),
            "saga" => Ok(Self::Saga),
            "shard" => Ok(Self::Shard),
            "shrine" => Ok(Self::Shrine),
            other => Err(format!("Unknown EnchantmentType: {}", other.to_string())),
        }
    }
}
impl EnchantmentType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Aura => "aura",
            Self::Background => "background",
            Self::Cartouche => "cartouche",
            Self::Case => "case",
            Self::Class => "class",
            Self::Curse => "curse",
            Self::Role => "role",
            Self::Room => "room",
            Self::Rune => "rune",
            Self::Saga => "saga",
            Self::Shard => "shard",
            Self::Shrine => "shrine",
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
