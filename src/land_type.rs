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
    Planet,
    PowerPlant,
    Sphere,
    Swamp,
    Tower,
    Town,
    Urzas,
}
impl std::str::FromStr for LandType {
    type Err = String;
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
            "Planet" => Ok(Self::Planet),
            "Power-Plant" => Ok(Self::PowerPlant),
            "Sphere" => Ok(Self::Sphere),
            "Swamp" => Ok(Self::Swamp),
            "Tower" => Ok(Self::Tower),
            "Town" => Ok(Self::Town),
            "Urza's" => Ok(Self::Urzas),
            other => Err(format!("Unknown LandType: {}", other.to_string())),
        }
    }
}
impl LandType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Cave => "Cave",
            Self::Cloud => "Cloud",
            Self::Desert => "Desert",
            Self::Forest => "Forest",
            Self::Gate => "Gate",
            Self::Island => "Island",
            Self::Lair => "Lair",
            Self::Locus => "Locus",
            Self::Mine => "Mine",
            Self::Mountain => "Mountain",
            Self::Plains => "Plains",
            Self::Planet => "Planet",
            Self::PowerPlant => "Power-Plant",
            Self::Sphere => "Sphere",
            Self::Swamp => "Swamp",
            Self::Tower => "Tower",
            Self::Town => "Town",
            Self::Urzas => "Urza's",
        }
    }
}
impl std::fmt::Display for LandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl LandType {
    pub fn all() -> impl Iterator<Item = Self> {
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
            Self::Planet,
            Self::PowerPlant,
            Self::Sphere,
            Self::Swamp,
            Self::Tower,
            Self::Town,
            Self::Urzas,
        ]
        .into_iter()
    }
}
