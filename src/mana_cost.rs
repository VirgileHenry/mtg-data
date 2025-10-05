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

impl std::str::FromStr for Mana {
    type Err = String;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        if from.starts_with('{') && from.ends_with('}') {
            let symbols = from[1..from.len() - 1]
                .split('/')
                .map(ManaSymbol::parse_symbol)
                .collect::<Vec<_>>();
            match symbols.as_slice() {
                [Some(ManaSymbol::X)] => Ok(Mana::X),
                [Some(ManaSymbol::Any(num))] => Ok(Mana::Any(*num)),
                [Some(ManaSymbol::Colored(color))] => Ok(Mana::Colored(*color)),
                [Some(ManaSymbol::Colored(c1)), Some(ManaSymbol::Colored(c2))] => {
                    Ok(Mana::Hybrid(*c1, *c2))
                }
                [Some(ManaSymbol::Any(num)), Some(ManaSymbol::Colored(color))] => {
                    Ok(Mana::MonocoloredHybrid(*num, *color))
                }
                [Some(ManaSymbol::Colored(color)), Some(ManaSymbol::Phyrexian)] => {
                    Ok(Mana::Phyrexian(*color))
                }
                [Some(ManaSymbol::Colored(c1)), Some(ManaSymbol::Colored(c2)), Some(ManaSymbol::Phyrexian)] => {
                    Ok(Mana::HybridPhyrexian(*c1, *c2))
                }
                [Some(ManaSymbol::Snow)] => Ok(Mana::Snow),
                _ => Err(format!("Invalid symbol combination: {symbols:?}")),
            }
        } else {
            Err(format!("Mana cost shall be between curly braces"))
        }
    }
}

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mana::X => write!(f, "{{x}}"),
            Mana::Snow => write!(f, "{{s}}"),
            Mana::Any(n) => write!(f, "{{{n}}}"),
            Mana::Colored(c) => write!(f, "{{{}}}", c.as_char()),
            Mana::Hybrid(c1, c2) => write!(f, "{{{}/{}}}", c1.as_char(), c2.as_char()),
            Mana::MonocoloredHybrid(n, c) => write!(f, "{{{n}/{}}}", c.as_char()),
            Mana::Phyrexian(c) => write!(f, "{{{}/p}}", c.as_char()),
            Mana::HybridPhyrexian(c1, c2) => write!(f, "{{{}/{}/p}}", c1.as_char(), c2.as_char()),
        }
    }
}

/// Inner type used to parse mana costs.
enum ManaSymbol {
    X,
    Any(usize),
    Colored(crate::Color),
    Phyrexian,
    Snow,
}

impl std::fmt::Debug for ManaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManaSymbol::X => write!(f, "x"),
            ManaSymbol::Any(num) => write!(f, "{num}"),
            ManaSymbol::Colored(color) => write!(f, "{}", color.as_char()),
            ManaSymbol::Phyrexian => write!(f, "p"),
            ManaSymbol::Snow => write!(f, "s"),
        }
    }
}

impl ManaSymbol {
    fn parse_symbol(input: &str) -> Option<ManaSymbol> {
        return match input {
            "w" => Some(ManaSymbol::Colored(crate::Color::White)),
            "b" => Some(ManaSymbol::Colored(crate::Color::Black)),
            "r" => Some(ManaSymbol::Colored(crate::Color::Red)),
            "u" => Some(ManaSymbol::Colored(crate::Color::Blue)),
            "g" => Some(ManaSymbol::Colored(crate::Color::Green)),
            "c" => Some(ManaSymbol::Colored(crate::Color::Colorless)),
            "x" => Some(ManaSymbol::X),
            "s" => Some(ManaSymbol::Snow),
            "p" => Some(ManaSymbol::Phyrexian),
            other => Some(ManaSymbol::Any(other.parse().ok()?)),
        };
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::Color;

    use super::*;

    #[test]
    fn test_mana_cost_parsing() {
        assert_eq!(Mana::from_str("{r}"), Ok(Mana::Colored(Color::Red)));
        assert_eq!(Mana::from_str("{x}"), Ok(Mana::X));
        assert_eq!(Mana::from_str("{12}"), Ok(Mana::Any(12)));
        assert_eq!(Mana::from_str("{s}"), Ok(Mana::Snow));
        assert_eq!(
            Mana::from_str("{r/g}"),
            Ok(Mana::Hybrid(Color::Red, Color::Green))
        );
        assert_eq!(Mana::from_str("{r/p}"), Ok(Mana::Phyrexian(Color::Red)));
        assert_eq!(
            Mana::from_str("{3/r}"),
            Ok(Mana::MonocoloredHybrid(3, Color::Red))
        );
        assert_eq!(
            Mana::from_str("{r/g/p}"),
            Ok(Mana::HybridPhyrexian(Color::Red, Color::Green))
        );
    }
}
