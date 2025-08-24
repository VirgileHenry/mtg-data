use std::{fmt::Display, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

// const regex to match numbers
lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new("[0-9]+").unwrap(); // ! fixme
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mana {
    X,
    Any(usize),
    Colored(crate::Color),
    Hybrid(crate::Color, crate::Color),
    MonocoloredHybrid(usize, crate::Color),
    Phyrexian(crate::Color),
    HybridPhyrexian(crate::Color, crate::Color),
    Snow,
}

/*
VHY Todo: this whole parsing could be rewritten more nicely
*/

/// This help building mana costs.
/// The only real needed thing is the partial phyrexian, to help parse phyrexian mana costs easily.
enum ManaCostBuildingTypes {
    X,
    Any(usize),
    Colored(crate::Color),
    Hybrid(crate::Color, crate::Color),
    MonocoloredHybrid(usize, crate::Color),
    PartialPhyrexian, // only two life
    Phyrexian(crate::Color),
    HybridPhyrexian(crate::Color, crate::Color),
    Snow,
}

impl FromStr for Mana {
    type Err = String;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match Self::internal_parse_manacost(from).ok_or(format!("Unknown Mana Cost: {}", from))? {
            ManaCostBuildingTypes::X => Ok(Mana::X),
            ManaCostBuildingTypes::Any(n) => Ok(Mana::Any(n)),
            ManaCostBuildingTypes::Colored(c) => Ok(Mana::Colored(c)),
            ManaCostBuildingTypes::Hybrid(c1, c2) => Ok(Mana::Hybrid(c1, c2)),
            ManaCostBuildingTypes::MonocoloredHybrid(n, c) => Ok(Mana::MonocoloredHybrid(n, c)),
            ManaCostBuildingTypes::PartialPhyrexian => Err(format!("Unknown Mana Cost: {}", from)), // not a valid mana cost, only for building
            ManaCostBuildingTypes::Phyrexian(c) => Ok(Mana::Phyrexian(c)),
            ManaCostBuildingTypes::HybridPhyrexian(c1, c2) => Ok(Mana::HybridPhyrexian(c1, c2)),
            ManaCostBuildingTypes::Snow => Ok(Mana::Snow),
        }
    }
}

impl Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mana::X => write!(f, "{{X}}"),
            Mana::Snow => write!(f, "{{S}}"),
            Mana::Any(n) => write!(f, "{{{n}}}"),
            Mana::Colored(c) => write!(f, "{{{c:?}}}"),
            Mana::Hybrid(c1, c2) => write!(f, "{{{c1:?}/{c2:?}}}"),
            Mana::MonocoloredHybrid(n, c) => write!(f, "{{{n}/{c:?}}}"),
            Mana::Phyrexian(c) => write!(f, "{{{c:?}/P}}"),
            Mana::HybridPhyrexian(c1, c2) => write!(f, "{{{c1:?}/{c2:?}/P}}"),
        }
    }
}

impl Mana {
    /// Parse a mana cost knowing we already have a phyrexian type.
    fn internal_parse_manacost(from: &str) -> Option<ManaCostBuildingTypes> {
        // let's do something fully recursive : find any cost option separator ("/")
        let split_index = match from.find("/") {
            Some(index) => index,
            None => {
                // no separator here, parse it as number or letter
                return match Self::internal_parse_numbered(from) {
                    Some(value) => Some(ManaCostBuildingTypes::Any(value)),
                    // this only check first letter, so we may discard info here
                    None => Self::internal_parse_single_letter(
                        from.chars().next()?.to_ascii_uppercase(),
                    ),
                };
            }
        };
        let (first, last) = from.split_at(split_index);
        // remove "/" from last
        let last = last.strip_prefix('/')?;
        // recursive call to get the mana cost of the first and last
        match (
            Self::internal_parse_manacost(first)?,
            Self::internal_parse_manacost(last)?,
        ) {
            // two colored mana give a hybrid mana cost
            (ManaCostBuildingTypes::Colored(c1), ManaCostBuildingTypes::Colored(c2)) => {
                Some(ManaCostBuildingTypes::Hybrid(c1, c2))
            }
            // an amount and color give monocolored hybrid mana (usually 2 and color)
            (ManaCostBuildingTypes::Any(amount), ManaCostBuildingTypes::Colored(color))
            | (ManaCostBuildingTypes::Colored(color), ManaCostBuildingTypes::Any(amount)) => {
                Some(ManaCostBuildingTypes::MonocoloredHybrid(amount, color))
            }
            // a partial phyrexian and color gives a phyrexian
            (ManaCostBuildingTypes::PartialPhyrexian, ManaCostBuildingTypes::Colored(color))
            | (ManaCostBuildingTypes::Colored(color), ManaCostBuildingTypes::PartialPhyrexian) => {
                Some(ManaCostBuildingTypes::Phyrexian(color))
            }
            // a phyrexian and a colored gives a hybrid phyrexian
            (ManaCostBuildingTypes::Phyrexian(c1), ManaCostBuildingTypes::Colored(c2))
            | (ManaCostBuildingTypes::Colored(c1), ManaCostBuildingTypes::Phyrexian(c2)) => {
                Some(ManaCostBuildingTypes::HybridPhyrexian(c1, c2))
            }
            _ => None,
        }
    }

    fn internal_parse_numbered(from: &str) -> Option<usize> {
        if NUMBER_REGEX.is_match(from) {
            from.parse().ok()
        } else {
            None
        }
    }

    fn internal_parse_single_letter(c: char) -> Option<ManaCostBuildingTypes> {
        return match c.to_ascii_uppercase() {
            'W' => Some(ManaCostBuildingTypes::Colored(crate::Color::White)),
            'B' => Some(ManaCostBuildingTypes::Colored(crate::Color::Black)),
            'R' => Some(ManaCostBuildingTypes::Colored(crate::Color::Red)),
            'U' => Some(ManaCostBuildingTypes::Colored(crate::Color::Blue)),
            'G' => Some(ManaCostBuildingTypes::Colored(crate::Color::Green)),
            'C' => Some(ManaCostBuildingTypes::Colored(crate::Color::Colorless)),
            'X' => Some(ManaCostBuildingTypes::X),
            'S' => Some(ManaCostBuildingTypes::Snow),
            'P' => Some(ManaCostBuildingTypes::PartialPhyrexian),
            _ => None, // unknown
        };
    }
}
