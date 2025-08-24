#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactType {
    Attraction,
    Blood,
    Bobblehead,
    Clue,
    Contraption,
    Equipment,
    Food,
    Fortification,
    Gold,
    Incubator,
    Infinity,
    Junk,
    Map,
    Powerstone,
    Spacecraft,
    Stone,
    Treasure,
    Vehicle,
}
impl std::str::FromStr for ArtifactType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Attraction" => Ok(Self::Attraction),
            "Blood" => Ok(Self::Blood),
            "Bobblehead" => Ok(Self::Bobblehead),
            "Clue" => Ok(Self::Clue),
            "Contraption" => Ok(Self::Contraption),
            "Equipment" => Ok(Self::Equipment),
            "Food" => Ok(Self::Food),
            "Fortification" => Ok(Self::Fortification),
            "Gold" => Ok(Self::Gold),
            "Incubator" => Ok(Self::Incubator),
            "Infinity" => Ok(Self::Infinity),
            "Junk" => Ok(Self::Junk),
            "Map" => Ok(Self::Map),
            "Powerstone" => Ok(Self::Powerstone),
            "Spacecraft" => Ok(Self::Spacecraft),
            "Stone" => Ok(Self::Stone),
            "Treasure" => Ok(Self::Treasure),
            "Vehicle" => Ok(Self::Vehicle),
            other => Err(format!("Unknown ArtifactType: {}", other.to_string())),
        }
    }
}
impl ArtifactType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Attraction => "Attraction",
            Self::Blood => "Blood",
            Self::Bobblehead => "Bobblehead",
            Self::Clue => "Clue",
            Self::Contraption => "Contraption",
            Self::Equipment => "Equipment",
            Self::Food => "Food",
            Self::Fortification => "Fortification",
            Self::Gold => "Gold",
            Self::Incubator => "Incubator",
            Self::Infinity => "Infinity",
            Self::Junk => "Junk",
            Self::Map => "Map",
            Self::Powerstone => "Powerstone",
            Self::Spacecraft => "Spacecraft",
            Self::Stone => "Stone",
            Self::Treasure => "Treasure",
            Self::Vehicle => "Vehicle",
        }
    }
}
impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl ArtifactType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Attraction,
            Self::Blood,
            Self::Bobblehead,
            Self::Clue,
            Self::Contraption,
            Self::Equipment,
            Self::Food,
            Self::Fortification,
            Self::Gold,
            Self::Incubator,
            Self::Infinity,
            Self::Junk,
            Self::Map,
            Self::Powerstone,
            Self::Spacecraft,
            Self::Stone,
            Self::Treasure,
            Self::Vehicle,
        ]
        .into_iter()
    }
}
