use std::{str::FromStr, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

use crate::MtgColor;

// const regex to match numbers
lazy_static!{
    static ref NUMBER_REGEX: Regex = Regex::new("[0-9]+").unwrap(); // ! fixme
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManaCost {
    X,
    Any(u8),
    Colored(MtgColor),
    Hybrid(MtgColor, MtgColor),
    MonocoloredHybrid(u8, MtgColor),
    Phyrexian(MtgColor),
    HybridPhyrexian(MtgColor, MtgColor),
    Snow,
}

/// This help building mana costs. 
/// The only real needed thing is the partial phyrexian, to help parse phyrexian mana costs easily.
enum ManaCostBuildingTypes {
    X,
    Any(u8),
    Colored(MtgColor),
    Hybrid(MtgColor, MtgColor),
    MonocoloredHybrid(u8, MtgColor),
    PartialPhyrexian, // only two life
    Phyrexian(MtgColor),
    HybridPhyrexian(MtgColor, MtgColor),
    Snow,
}

impl FromStr for ManaCost {
    type Err = ();
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match Self::internal_parse_manacost(from).ok_or(())? {
            ManaCostBuildingTypes::X => Ok(ManaCost::X),
            ManaCostBuildingTypes::Any(n) => Ok(ManaCost::Any(n)),
            ManaCostBuildingTypes::Colored(c) => Ok(ManaCost::Colored(c)),
            ManaCostBuildingTypes::Hybrid(c1, c2) => Ok(ManaCost::Hybrid(c1, c2)),
            ManaCostBuildingTypes::MonocoloredHybrid(n, c) => Ok(ManaCost::MonocoloredHybrid(n, c)),
            ManaCostBuildingTypes::PartialPhyrexian => Err(()), // not a valid mana cost, only for building
            ManaCostBuildingTypes::Phyrexian(c) => Ok(ManaCost::Phyrexian(c)),
            ManaCostBuildingTypes::HybridPhyrexian(c1, c2) => Ok(ManaCost::HybridPhyrexian(c1, c2)),
            ManaCostBuildingTypes::Snow => Ok(ManaCost::Snow),
        }
    }
}

impl Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManaCost::X => write!(f, "{{X}}"),
            ManaCost::Snow => write!(f, "{{S}}"),
            ManaCost::Any(n) => write!(f, "{{{n}}}"),
            ManaCost::Colored(c) => write!(f, "{{{c}}}"),
            ManaCost::Hybrid(c1, c2) => write!(f, "{{{c1}/{c2}}}"),
            ManaCost::MonocoloredHybrid(n, c) => write!(f, "{{{n}/{c}}}"),
            ManaCost::Phyrexian(c) => write!(f, "{{{c}/P}}"),
            ManaCost::HybridPhyrexian(c1, c2) => write!(f, "{{{c1}/{c2}/P}}"),
        }
    }
}


impl ManaCost {

    /// Parse a mana cost knowing we already have a phyrexian type.
    fn internal_parse_manacost(from: &str) -> Option<ManaCostBuildingTypes> {
        // let's do something fully recursive : find any cost option separator ("/")
        let (first, last) = from.split_at(match from.find("/") {
            Some(index) => index,
            None => {
                // no separator here, parse it as number or letter
                return match Self::internal_parse_numbered(from) {
                    Some(value) => Some(ManaCostBuildingTypes::Any(value)),
                    // this only check first letter, so we may discard info here
                    None => Self::internal_parse_single_letter(from.chars().next()?.to_ascii_uppercase()),
                }
            },
        });
        // remove "/" from last
        let last = last.strip_prefix('/')?;
        // recursive call to get the mana cost of the first and last
        match (Self::internal_parse_manacost(first)?, Self::internal_parse_manacost(last)?) {
            // two colored mana give a hybrid mana cost
            (ManaCostBuildingTypes::Colored(c1), ManaCostBuildingTypes::Colored(c2)) =>
                Some(ManaCostBuildingTypes::Hybrid(c1, c2)),
            // an amount and color give monocolored hybrid mana (usually 2 and color)
            (ManaCostBuildingTypes::Any(amount), ManaCostBuildingTypes::Colored(color)) |
                (ManaCostBuildingTypes::Colored(color), ManaCostBuildingTypes::Any(amount)) =>
                Some(ManaCostBuildingTypes::MonocoloredHybrid(amount, color)),
            // a partial phyrexian and color gives a phyrexian
            (ManaCostBuildingTypes::PartialPhyrexian, ManaCostBuildingTypes::Colored(color)) |
                (ManaCostBuildingTypes::Colored(color), ManaCostBuildingTypes::PartialPhyrexian) =>
                Some(ManaCostBuildingTypes::Phyrexian(color)),
            // a phyrexian and a colored gives a hybrid phyrexian
            (ManaCostBuildingTypes::Phyrexian(c1), ManaCostBuildingTypes::Colored(c2)) |
                (ManaCostBuildingTypes::Colored(c1), ManaCostBuildingTypes::Phyrexian(c2)) =>
                Some(ManaCostBuildingTypes::HybridPhyrexian(c1, c2)),
            _ => None,
        }
    }

    fn internal_parse_numbered(from: &str) -> Option<u8> {
        if NUMBER_REGEX.is_match(from) {
            from.parse().ok()
        }
        else {
            None
        }
    }

    fn internal_parse_single_letter(c: char) -> Option<ManaCostBuildingTypes> {
        return match c.to_ascii_uppercase() {
            'W' => Some(ManaCostBuildingTypes::Colored(MtgColor::White)),
            'B' => Some(ManaCostBuildingTypes::Colored(MtgColor::Black)),
            'R' => Some(ManaCostBuildingTypes::Colored(MtgColor::Red)),
            'U' => Some(ManaCostBuildingTypes::Colored(MtgColor::Blue)),
            'G' => Some(ManaCostBuildingTypes::Colored(MtgColor::Green)),
            'C' => Some(ManaCostBuildingTypes::Colored(MtgColor::Colorless)),
            'X' => Some(ManaCostBuildingTypes::X),
            'S' => Some(ManaCostBuildingTypes::Snow),
            'P' => Some(ManaCostBuildingTypes::PartialPhyrexian),
            _ => None // unknown
        }
    }

}

