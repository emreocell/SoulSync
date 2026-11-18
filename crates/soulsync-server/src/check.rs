//! Plausibility checks for what players send. A run is kept in memory and on
//! disk, so a save or dex far larger than any game can produce is refused
//! before it is stored. The limits leave plenty of room above real games.

use soulsync_core::{Names, Pokemon, RomDex, SaveSnapshot};

/// Party and opponents: six each, twelve in a double battle of two trainers.
const TEAM: usize = 12;
/// 24 boxes of 30 in Black 2 and White 2.
const BOXES: usize = 24 * 30;
/// Wild encounters and fallen Pokémon of one run.
const SEEN: usize = 5000;
/// Characters of a name; games allow far fewer.
const NAME: usize = 64;
/// Characters of a description in a ROM's dex.
const TEXT: usize = 1024;

fn short(s: &str, max: usize) -> bool {
    s.chars().count() <= max
}

fn names(n: &Option<Names>) -> bool {
    n.as_ref()
        .map_or(true, |n| short(&n.en, NAME) && short(&n.de, NAME))
}

fn pokemon(p: &Pokemon) -> bool {
    short(&p.nickname, NAME)
        && names(&p.species_name)
        && names(&p.met_location_name)
        && p.types.len() <= 2
        && p.moves.len() <= 4
}

/// Whether `s` could have come from a game.
pub fn snapshot(s: &SaveSnapshot) -> Result<(), &'static str> {
    let sizes = s.party.len() <= TEAM
        && s.foes.len() <= TEAM
        && s.boxes.len() <= BOXES
        && s.encounters.len() <= SEEN
        && s.fallen.len() <= SEEN
        && s.balls.as_ref().map_or(true, |b| b.len() <= 256)
        && short(&s.trainer.name, NAME);
    let each = s
        .party
        .iter()
        .chain(&s.foes)
        .chain(&s.boxes)
        .chain(&s.encounters)
        .chain(&s.fallen)
        .chain(s.wild.as_deref())
        .all(pokemon);
    if sizes && each {
        Ok(())
    } else {
        Err("this save is larger than any game makes")
    }
}

/// Whether `d` could have come from a ROM.
pub fn dex(d: &RomDex) -> Result<(), &'static str> {
    let entry = |name: &str, text: &str| short(name, NAME) && short(text, TEXT);
    let ok = d.moves.len() <= 1024
        && d.abilities.len() <= 512
        && d.items.len() <= 2048
        && d.learnsets.len() <= 1024
        && d.types.len() <= 1024
        && d.learnsets.iter().all(|l| l.len() <= 256)
        && d.types.iter().all(|t| t.len() <= 2)
        && d.moves.iter().all(|m| entry(&m.name, &m.text))
        && d.abilities
            .iter()
            .chain(&d.items)
            .all(|e| entry(&e.name, &e.text));
    if ok {
        Ok(())
    } else {
        Err("this dex is larger than any ROM has")
    }
}
