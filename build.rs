fn sanitize(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());
    let mut start_of_word = true;

    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                result.push(if start_of_word {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                });
                start_of_word = false;
            }
            '\'' | '.' | '!' => {}
            ' ' | '-' => start_of_word = true,
            other => {
                return Err(format!(
                    "Unhandled character in sanitizing process: '{other}'"
                ))
            }
        }
    }

    Ok(result)
}

struct ToGenerateEnum<'a> {
    name: &'a str,
    source_file: &'a str,
    destination_file: &'a str,
}

const TO_GENERATE_ENUMS: [ToGenerateEnum<'static>; 15] = [
    ToGenerateEnum {
        name: "AbilityWord",
        source_file: "data/ability_word.txt",
        destination_file: "src/ability_word.rs",
    },
    ToGenerateEnum {
        name: "ArtifactType",
        source_file: "data/artifact_type.txt",
        destination_file: "src/artifact_type.rs",
    },
    ToGenerateEnum {
        name: "BattleType",
        source_file: "data/battle_type.txt",
        destination_file: "src/battle_type.rs",
    },
    ToGenerateEnum {
        name: "CardType",
        source_file: "data/card_type.txt",
        destination_file: "src/card_type.rs",
    },
    ToGenerateEnum {
        name: "Color",
        source_file: "data/color.txt",
        destination_file: "src/color.rs",
    },
    ToGenerateEnum {
        name: "CreatureType",
        source_file: "data/creature_type.txt",
        destination_file: "src/creature_type.rs",
    },
    ToGenerateEnum {
        name: "EnchantmentType",
        source_file: "data/enchantment_type.txt",
        destination_file: "src/enchantment_type.rs",
    },
    ToGenerateEnum {
        name: "Format",
        source_file: "data/format.txt",
        destination_file: "src/format.rs",
    },
    ToGenerateEnum {
        name: "KeywordAbility",
        source_file: "data/keyword_ability.txt",
        destination_file: "src/keyword_ability.rs",
    },
    ToGenerateEnum {
        name: "KeywordAction",
        source_file: "data/keyword_action.txt",
        destination_file: "src/keyword_action.rs",
    },
    ToGenerateEnum {
        name: "LandType",
        source_file: "data/land_type.txt",
        destination_file: "src/land_type.rs",
    },
    ToGenerateEnum {
        name: "Legality",
        source_file: "data/legality.txt",
        destination_file: "src/legality.rs",
    },
    ToGenerateEnum {
        name: "PlaneswalkerType",
        source_file: "data/planeswalker_type.txt",
        destination_file: "src/planeswalker_type.rs",
    },
    ToGenerateEnum {
        name: "SpellType",
        source_file: "data/spell_type.txt",
        destination_file: "src/spell_type.rs",
    },
    ToGenerateEnum {
        name: "Supertype",
        source_file: "data/supertype.txt",
        destination_file: "src/supertype.rs",
    },
];

impl<'a> ToGenerateEnum<'a> {
    fn generate(&self) -> Result<(), std::io::Error> {
        use std::io::Write;

        let source = std::fs::read_to_string(self.source_file)?;
        let mut destination = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.destination_file)?;

        // sanitize all lines in input file as enum ready tokens
        let variants = source
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| sanitize(line).map(|s| (line, s)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| std::io::Error::other(format!("Failed to sanitize input line: {e}")))?;

        // Write out the enum
        writeln!(
            destination,
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]"
        )?;
        writeln!(destination, "pub enum {} {{", self.name)?;
        for (_, variant) in variants.iter() {
            writeln!(destination, "{},", variant)?;
        }
        writeln!(destination, "}}")?;

        // write out the parse func
        writeln!(destination, "impl std::str::FromStr for {} {{", self.name)?;
        writeln!(destination, "type Err = String;")?;
        writeln!(
            destination,
            "fn from_str(s: &str) -> Result<Self, Self::Err> {{"
        )?;
        writeln!(destination, "match s {{")?;
        for (line, variant) in variants.iter() {
            writeln!(destination, "\"{}\" => Ok(Self::{}),", line, variant)?;
        }
        writeln!(
            destination,
            "other => Err(format!(\"Unknown {}: {{}}\", other.to_string())),",
            self.name
        )?;
        writeln!(destination, "}} }} }}")?;

        // Write out the display funcs
        writeln!(destination, "impl {} {{", self.name)?;
        writeln!(destination, "fn as_str(&self) -> &'static str {{")?;
        writeln!(destination, "match self {{")?;
        for (line, variant) in variants.iter() {
            writeln!(destination, "Self::{} =>\"{}\",", variant, line)?;
        }
        writeln!(destination, "}} }} }}")?;

        writeln!(destination, "impl std::fmt::Display for {} {{", self.name)?;
        writeln!(
            destination,
            "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
        )?;
        writeln!(destination, "write!(f, \"{{}}\", self.as_str())")?;
        writeln!(destination, "}} }}")?;

        // write the iter func
        writeln!(destination, "impl {} {{", self.name)?;
        writeln!(destination, "pub fn all() -> impl Iterator<Item = Self> {{")?;
        writeln!(destination, "[")?;
        for (_, variant) in variants.iter() {
            writeln!(destination, "Self::{},", variant)?;
        }
        writeln!(destination, "].into_iter()")?;
        writeln!(destination, "}} }} ")?;

        std::process::Command::new("rustfmt")
            .arg(&self.destination_file)
            .output()?;

        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    // if any of the source files used changed, rerun generation
    for to_gen in TO_GENERATE_ENUMS.iter() {
        println!("cargo::rerun-if-changed={}", to_gen.source_file);
        to_gen.generate()?;
    }

    Ok(())
}
