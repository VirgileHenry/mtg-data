use strum_macros::EnumString;

/// All game formats in Mtg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    Alchemy,
    Brawl,
    Commander,
    Duel,
    Explorer,
    Future,
    Gladiator,
    Historic,
    HistoricBrawl,
    Legacy,
    Modern,
    Oathbreaker,
    Pauper,
    PauperCommander,
    Penny,
    Pionner,
    Predh,
    Premodern,
    Standard,
    Vintage
}