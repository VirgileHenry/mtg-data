#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Format {
    Alchemy,
    Brawl,
    Commander,
    Duel,
    Explorer,
    Future,
    Gladiator,
    Historic,
    Historicbrawl,
    Legacy,
    Modern,
    Oathbreaker,
    Pauper,
    Paupercommander,
    Penny,
    Pionner,
    Predh,
    Premodern,
    Standard,
    Vintage,
}
impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Alchemy" => Ok(Self::Alchemy),
            "Brawl" => Ok(Self::Brawl),
            "Commander" => Ok(Self::Commander),
            "Duel" => Ok(Self::Duel),
            "Explorer" => Ok(Self::Explorer),
            "Future" => Ok(Self::Future),
            "Gladiator" => Ok(Self::Gladiator),
            "Historic" => Ok(Self::Historic),
            "HistoricBrawl" => Ok(Self::Historicbrawl),
            "Legacy" => Ok(Self::Legacy),
            "Modern" => Ok(Self::Modern),
            "Oathbreaker" => Ok(Self::Oathbreaker),
            "Pauper" => Ok(Self::Pauper),
            "PauperCommander" => Ok(Self::Paupercommander),
            "Penny" => Ok(Self::Penny),
            "Pionner" => Ok(Self::Pionner),
            "Predh" => Ok(Self::Predh),
            "Premodern" => Ok(Self::Premodern),
            "Standard" => Ok(Self::Standard),
            "Vintage" => Ok(Self::Vintage),
            other => Err(format!("Unknown Format: {}", other.to_string())),
        }
    }
}
impl Format {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Alchemy => "Alchemy",
            Self::Brawl => "Brawl",
            Self::Commander => "Commander",
            Self::Duel => "Duel",
            Self::Explorer => "Explorer",
            Self::Future => "Future",
            Self::Gladiator => "Gladiator",
            Self::Historic => "Historic",
            Self::Historicbrawl => "HistoricBrawl",
            Self::Legacy => "Legacy",
            Self::Modern => "Modern",
            Self::Oathbreaker => "Oathbreaker",
            Self::Pauper => "Pauper",
            Self::Paupercommander => "PauperCommander",
            Self::Penny => "Penny",
            Self::Pionner => "Pionner",
            Self::Predh => "Predh",
            Self::Premodern => "Premodern",
            Self::Standard => "Standard",
            Self::Vintage => "Vintage",
        }
    }
}
impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Format {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Alchemy,
            Self::Brawl,
            Self::Commander,
            Self::Duel,
            Self::Explorer,
            Self::Future,
            Self::Gladiator,
            Self::Historic,
            Self::Historicbrawl,
            Self::Legacy,
            Self::Modern,
            Self::Oathbreaker,
            Self::Pauper,
            Self::Paupercommander,
            Self::Penny,
            Self::Pionner,
            Self::Predh,
            Self::Premodern,
            Self::Standard,
            Self::Vintage,
        ]
        .into_iter()
    }
}
