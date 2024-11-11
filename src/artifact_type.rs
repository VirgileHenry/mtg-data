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
    Junk,
    Map,
    Powerstone,
    Treasure,
    Vehicle,
}

impl std::str::FromStr for ArtifactType {
    type Err = crate::ParsingError;
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
            "Junk" => Ok(Self::Junk),
            "Map" => Ok(Self::Map),
            "Powerstone" => Ok(Self::Powerstone),
            "Treasure" => Ok(Self::Treasure),
            "Vehicle" => Ok(Self::Vehicle),
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Attraction => write!(f, "Attraction"),
            Self::Blood => write!(f, "Blood"),
            Self::Bobblehead => write!(f, "Bobblehead"),
            Self::Clue => write!(f, "Clue"),
            Self::Contraption => write!(f, "Contraption"),
            Self::Equipment => write!(f, "Equipment"),
            Self::Food => write!(f, "Food"),
            Self::Fortification => write!(f, "Fortification"),
            Self::Gold => write!(f, "Gold"),
            Self::Incubator => write!(f, "Incubator"),
            Self::Junk => write!(f, "Junk"),
            Self::Map => write!(f, "Map"),
            Self::Powerstone => write!(f, "Powerstone"),
            Self::Treasure => write!(f, "Treasure"),
            Self::Vehicle => write!(f, "Vehicle"),
        }
    }
}

impl ArtifactType {
    pub fn iter() -> impl Iterator<Item = Self> {
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
            Self::Junk,
            Self::Map,
            Self::Powerstone,
            Self::Treasure,
            Self::Vehicle,
        ].into_iter()
    }
}
