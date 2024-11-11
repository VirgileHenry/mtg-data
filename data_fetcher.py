# This python script will fetch the latest data on scryfall,
# a MTG card database with a suitable API to update the various
# files in the data/ directory.
# 
# Those files are then used by the rust build script to generate the required enums for the game.
# Please note that not all files are automatically fetched and generated, so it is not 
# recommanded to remove all the data/ folder and rerun the script.
# Instead, simply run this python script and it will override the outdated files.
#
# This script shall be run every once in a while (with every new extension)
# to keep up with the new sets. 


import requests


def fetch_and_save(target_file, url):
    print("Fetching", url, "->", target_file)
    with open(target_file, "w") as output:
        req = requests.get(url).json()
        if req["object"] == "error":
            raise Exception(f"Failed to fetch data, req returned {req}")
        lines = sorted(req["data"])
        if req["total_values"] != len(lines):
            raise Exception(f"Invalid value count, expected {req['total_values']} but found {len(lines)}")
        for line in lines:
            output.write(f"{line}\n")


fetch_and_save("data/ability_word.txt",         "https://api.scryfall.com/catalog/ability-words")
fetch_and_save("data/artifact_type.txt",        "https://api.scryfall.com/catalog/artifact-types")
fetch_and_save("data/battle_type.txt",          "https://api.scryfall.com/catalog/battle-types")
fetch_and_save("data/card_type.txt",            "https://api.scryfall.com/catalog/card-types")
fetch_and_save("data/creature_type.txt",        "https://api.scryfall.com/catalog/creature-types")
fetch_and_save("data/enchantment_type.txt",     "https://api.scryfall.com/catalog/enchantment-types")
fetch_and_save("data/keyword_ability.txt",      "https://api.scryfall.com/catalog/keyword-abilities")
fetch_and_save("data/keyword_action.txt",       "https://api.scryfall.com/catalog/keyword-actions")
fetch_and_save("data/land_type.txt",            "https://api.scryfall.com/catalog/land-types")
fetch_and_save("data/planeswalker_type.txt",    "https://api.scryfall.com/catalog/planeswalker-types")
fetch_and_save("data/spell_type.txt",           "https://api.scryfall.com/catalog/spell-types")
fetch_and_save("data/supertype.txt",            "https://api.scryfall.com/catalog/supertypes")

