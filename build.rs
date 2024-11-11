
fn sanitize(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());
    let mut start_of_word = true;

    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => if start_of_word {
                result.push(c.to_ascii_uppercase());
                start_of_word = false;
            } else {
                result.push(c.to_ascii_lowercase());
            },
            '\'' => {}, // the 's case, simply skip and let add the s
            ' ' | '-' => start_of_word = true, // space like chars, back to start of word
            '.' => {}, // only one with this is B.O.B. afaik, so lets keep it as "Bob" in the enum (and not BOB)
            other => return Err(format!("Unhandled character in sanitizing process: '{other}'")),
        }
    }

    Ok(result)
}

struct ToGenerateEnum<'a> {
    name: &'a str,
    source_file: &'a str,
    destination_file: &'a str,
}

const TO_GENERATE_ENUMS: [ToGenerateEnum::<'static>; 15] = [
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
        
        let source_file = std::fs::read_to_string(self.source_file)?;

        let mut enum_decl = format!(
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\npub enum {} {{\n",
            self.name
        );
        let mut parse_func = format!(
            "impl std::str::FromStr for {} {{\n    type Err = crate::ParsingError;\n    fn from_str(s: &str) -> Result<Self, Self::Err> {{\n        match s {{\n",
            self.name
        );
        let mut display_func = format!(
            "impl std::fmt::Display for {} {{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        match &self {{\n",
            self.name
        );
        let mut iter_func = format!(
            "impl {} {{\n    pub fn iter() -> impl Iterator<Item = Self> {{\n        [\n",
            self.name
        );


        for line in source_file.split('\n') {
            
            let line = line.trim();
            if line.is_empty() {
                continue; // skip
            }

            match sanitize(line) {
                Ok(variant) => {
                    enum_decl.push_str(&format!("    {},\n", variant));
                    parse_func.push_str(&format!("            \"{}\" => Ok(Self::{}),\n", line, variant));
                    display_func.push_str(&format!("            Self::{} => write!(f, \"{}\"),\n", variant, line));
                    iter_func.push_str(&format!("            Self::{},\n", variant));
                },
                Err(e) => {
                    // maybe we could simply skip ? But I prefer to fail the build with a hard error, to force us to handle all data
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to sanitize input line: {e}")));
                },
            }
        }

        enum_decl.push_str("}\n");
        parse_func.push_str("            _ => Err(crate::ParsingError::UnknownInput { input: s.to_string() }),\n        }\n    }\n}\n");
        display_func.push_str("        }\n    }\n}\n");
        iter_func.push_str("        ].into_iter()\n    }\n}\n");

        use std::io::Write;
        let mut destination_file = std::fs::OpenOptions::new().create(true).write(true).open(self.destination_file)?;
        
        write!(destination_file, "{}\n{}\n{}\n{}", enum_decl, parse_func, display_func, iter_func)?;
        
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





