#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Emblem,
    Enchantment,
    Hero,
    Instant,
    Kindred,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Vanguard,
}

impl std::str::FromStr for CardType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Artifact" => Ok(Self::Artifact),
            "Battle" => Ok(Self::Battle),
            "Conspiracy" => Ok(Self::Conspiracy),
            "Creature" => Ok(Self::Creature),
            "Dungeon" => Ok(Self::Dungeon),
            "Emblem" => Ok(Self::Emblem),
            "Enchantment" => Ok(Self::Enchantment),
            "Hero" => Ok(Self::Hero),
            "Instant" => Ok(Self::Instant),
            "Kindred" => Ok(Self::Kindred),
            "Land" => Ok(Self::Land),
            "Phenomenon" => Ok(Self::Phenomenon),
            "Plane" => Ok(Self::Plane),
            "Planeswalker" => Ok(Self::Planeswalker),
            "Scheme" => Ok(Self::Scheme),
            "Sorcery" => Ok(Self::Sorcery),
            "Vanguard" => Ok(Self::Vanguard),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Artifact => write!(f, "Artifact"),
            Self::Battle => write!(f, "Battle"),
            Self::Conspiracy => write!(f, "Conspiracy"),
            Self::Creature => write!(f, "Creature"),
            Self::Dungeon => write!(f, "Dungeon"),
            Self::Emblem => write!(f, "Emblem"),
            Self::Enchantment => write!(f, "Enchantment"),
            Self::Hero => write!(f, "Hero"),
            Self::Instant => write!(f, "Instant"),
            Self::Kindred => write!(f, "Kindred"),
            Self::Land => write!(f, "Land"),
            Self::Phenomenon => write!(f, "Phenomenon"),
            Self::Plane => write!(f, "Plane"),
            Self::Planeswalker => write!(f, "Planeswalker"),
            Self::Scheme => write!(f, "Scheme"),
            Self::Sorcery => write!(f, "Sorcery"),
            Self::Vanguard => write!(f, "Vanguard"),
        }
    }
}

impl CardType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Artifact,
            Self::Battle,
            Self::Conspiracy,
            Self::Creature,
            Self::Dungeon,
            Self::Emblem,
            Self::Enchantment,
            Self::Hero,
            Self::Instant,
            Self::Kindred,
            Self::Land,
            Self::Phenomenon,
            Self::Plane,
            Self::Planeswalker,
            Self::Scheme,
            Self::Sorcery,
            Self::Vanguard,
        ].into_iter()
    }
}
