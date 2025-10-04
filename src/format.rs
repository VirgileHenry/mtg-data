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
            "alchemy" => Ok(Self::Alchemy),
            "brawl" => Ok(Self::Brawl),
            "commander" => Ok(Self::Commander),
            "duel" => Ok(Self::Duel),
            "explorer" => Ok(Self::Explorer),
            "future" => Ok(Self::Future),
            "gladiator" => Ok(Self::Gladiator),
            "historic" => Ok(Self::Historic),
            "historicbrawl" => Ok(Self::Historicbrawl),
            "legacy" => Ok(Self::Legacy),
            "modern" => Ok(Self::Modern),
            "oathbreaker" => Ok(Self::Oathbreaker),
            "pauper" => Ok(Self::Pauper),
            "paupercommander" => Ok(Self::Paupercommander),
            "penny" => Ok(Self::Penny),
            "pionner" => Ok(Self::Pionner),
            "predh" => Ok(Self::Predh),
            "premodern" => Ok(Self::Premodern),
            "standard" => Ok(Self::Standard),
            "vintage" => Ok(Self::Vintage),
            other => Err(format!("Unknown Format: {}", other.to_string())),
        }
    }
}
impl Format {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Alchemy => "alchemy",
            Self::Brawl => "brawl",
            Self::Commander => "commander",
            Self::Duel => "duel",
            Self::Explorer => "explorer",
            Self::Future => "future",
            Self::Gladiator => "gladiator",
            Self::Historic => "historic",
            Self::Historicbrawl => "historicbrawl",
            Self::Legacy => "legacy",
            Self::Modern => "modern",
            Self::Oathbreaker => "oathbreaker",
            Self::Pauper => "pauper",
            Self::Paupercommander => "paupercommander",
            Self::Penny => "penny",
            Self::Pionner => "pionner",
            Self::Predh => "predh",
            Self::Premodern => "premodern",
            Self::Standard => "standard",
            Self::Vintage => "vintage",
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
