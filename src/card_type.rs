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
    type Err = String;
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
            other => Err(format!("Unknown CardType: {}", other.to_string())),
        }
    }
}
impl CardType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Artifact => "Artifact",
            Self::Battle => "Battle",
            Self::Conspiracy => "Conspiracy",
            Self::Creature => "Creature",
            Self::Dungeon => "Dungeon",
            Self::Emblem => "Emblem",
            Self::Enchantment => "Enchantment",
            Self::Hero => "Hero",
            Self::Instant => "Instant",
            Self::Kindred => "Kindred",
            Self::Land => "Land",
            Self::Phenomenon => "Phenomenon",
            Self::Plane => "Plane",
            Self::Planeswalker => "Planeswalker",
            Self::Scheme => "Scheme",
            Self::Sorcery => "Sorcery",
            Self::Vanguard => "Vanguard",
        }
    }
}
impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl CardType {
    pub fn all() -> impl Iterator<Item = Self> {
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
        ]
        .into_iter()
    }
}
