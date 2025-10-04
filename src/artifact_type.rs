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
    Terminus,
    Treasure,
    Vehicle,
}
impl std::str::FromStr for ArtifactType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attraction" => Ok(Self::Attraction),
            "blood" => Ok(Self::Blood),
            "bobblehead" => Ok(Self::Bobblehead),
            "clue" => Ok(Self::Clue),
            "contraption" => Ok(Self::Contraption),
            "equipment" => Ok(Self::Equipment),
            "food" => Ok(Self::Food),
            "fortification" => Ok(Self::Fortification),
            "gold" => Ok(Self::Gold),
            "incubator" => Ok(Self::Incubator),
            "infinity" => Ok(Self::Infinity),
            "junk" => Ok(Self::Junk),
            "map" => Ok(Self::Map),
            "powerstone" => Ok(Self::Powerstone),
            "spacecraft" => Ok(Self::Spacecraft),
            "stone" => Ok(Self::Stone),
            "terminus" => Ok(Self::Terminus),
            "treasure" => Ok(Self::Treasure),
            "vehicle" => Ok(Self::Vehicle),
            other => Err(format!("Unknown ArtifactType: {}", other.to_string())),
        }
    }
}
impl ArtifactType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Attraction => "attraction",
            Self::Blood => "blood",
            Self::Bobblehead => "bobblehead",
            Self::Clue => "clue",
            Self::Contraption => "contraption",
            Self::Equipment => "equipment",
            Self::Food => "food",
            Self::Fortification => "fortification",
            Self::Gold => "gold",
            Self::Incubator => "incubator",
            Self::Infinity => "infinity",
            Self::Junk => "junk",
            Self::Map => "map",
            Self::Powerstone => "powerstone",
            Self::Spacecraft => "spacecraft",
            Self::Stone => "stone",
            Self::Terminus => "terminus",
            Self::Treasure => "treasure",
            Self::Vehicle => "vehicle",
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
            Self::Terminus,
            Self::Treasure,
            Self::Vehicle,
        ]
        .into_iter()
    }
}
