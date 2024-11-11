#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LandType {
    Cave,
    Cloud,
    Desert,
    Forest,
    Gate,
    Island,
    Lair,
    Locus,
    Mine,
    Mountain,
    Plains,
    PowerPlant,
    Sphere,
    Swamp,
    Tower,
    Urzas,
}

impl std::str::FromStr for LandType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cave" => Ok(Self::Cave),
            "Cloud" => Ok(Self::Cloud),
            "Desert" => Ok(Self::Desert),
            "Forest" => Ok(Self::Forest),
            "Gate" => Ok(Self::Gate),
            "Island" => Ok(Self::Island),
            "Lair" => Ok(Self::Lair),
            "Locus" => Ok(Self::Locus),
            "Mine" => Ok(Self::Mine),
            "Mountain" => Ok(Self::Mountain),
            "Plains" => Ok(Self::Plains),
            "Power-Plant" => Ok(Self::PowerPlant),
            "Sphere" => Ok(Self::Sphere),
            "Swamp" => Ok(Self::Swamp),
            "Tower" => Ok(Self::Tower),
            "Urza's" => Ok(Self::Urzas),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for LandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Cave => write!(f, "Cave"),
            Self::Cloud => write!(f, "Cloud"),
            Self::Desert => write!(f, "Desert"),
            Self::Forest => write!(f, "Forest"),
            Self::Gate => write!(f, "Gate"),
            Self::Island => write!(f, "Island"),
            Self::Lair => write!(f, "Lair"),
            Self::Locus => write!(f, "Locus"),
            Self::Mine => write!(f, "Mine"),
            Self::Mountain => write!(f, "Mountain"),
            Self::Plains => write!(f, "Plains"),
            Self::PowerPlant => write!(f, "Power-Plant"),
            Self::Sphere => write!(f, "Sphere"),
            Self::Swamp => write!(f, "Swamp"),
            Self::Tower => write!(f, "Tower"),
            Self::Urzas => write!(f, "Urza's"),
        }
    }
}

impl LandType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Cave,
            Self::Cloud,
            Self::Desert,
            Self::Forest,
            Self::Gate,
            Self::Island,
            Self::Lair,
            Self::Locus,
            Self::Mine,
            Self::Mountain,
            Self::Plains,
            Self::PowerPlant,
            Self::Sphere,
            Self::Swamp,
            Self::Tower,
            Self::Urzas,
        ].into_iter()
    }
}
