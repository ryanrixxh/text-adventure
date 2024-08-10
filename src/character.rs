use std::{collections::HashMap, hash::Hash};

use rand::Rng;

pub trait Roll {
    fn generate_rep() -> i32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..10);
    }

    fn roll() -> Character;

    fn roll_slaughterer() -> Slaughterer;
}

pub struct Character {
    pub _reputation: Reputation,
}

impl Roll for Character {
    fn roll_slaughterer() -> Slaughterer {
        return HashMap::from([
            (Races::Human, Self::generate_rep()),
            (Races::Elf, Self::generate_rep()),
            (Races::Dwarf, Self::generate_rep()),
        ]);
    }

    fn roll() -> Character {
        return Character {
            _reputation: Reputation {
                _slaughterer: Self::roll_slaughterer(),
            }
        } 
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Races {
    Human,
    Elf,
    Dwarf
}

pub type Slaughterer = HashMap<Races, i32>;

pub struct Reputation {
    pub _slaughterer: HashMap<Races, i32>
}

