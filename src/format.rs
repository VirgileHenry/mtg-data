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
    type Err = crate::ParsingError;
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
            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Alchemy => write!(f, "Alchemy"),
            Self::Brawl => write!(f, "Brawl"),
            Self::Commander => write!(f, "Commander"),
            Self::Duel => write!(f, "Duel"),
            Self::Explorer => write!(f, "Explorer"),
            Self::Future => write!(f, "Future"),
            Self::Gladiator => write!(f, "Gladiator"),
            Self::Historic => write!(f, "Historic"),
            Self::Historicbrawl => write!(f, "HistoricBrawl"),
            Self::Legacy => write!(f, "Legacy"),
            Self::Modern => write!(f, "Modern"),
            Self::Oathbreaker => write!(f, "Oathbreaker"),
            Self::Pauper => write!(f, "Pauper"),
            Self::Paupercommander => write!(f, "PauperCommander"),
            Self::Penny => write!(f, "Penny"),
            Self::Pionner => write!(f, "Pionner"),
            Self::Predh => write!(f, "Predh"),
            Self::Premodern => write!(f, "Premodern"),
            Self::Standard => write!(f, "Standard"),
            Self::Vintage => write!(f, "Vintage"),
        }
    }
}

impl Format {
    pub fn iter() -> impl Iterator<Item = Self> {
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
        ].into_iter()
    }
}
