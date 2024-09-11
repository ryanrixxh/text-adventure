use std::{collections::HashMap, hash::Hash};

use rand::Rng;

pub trait Roll {
    fn generate_rep() -> f32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0.0..1.0);
    }

    fn roll() -> Character;

    fn roll_slaughterer() -> Slaughterer;
    fn roll_saviour() -> Saviour;
}

pub struct Character {
    pub reputation: Reputation,
}

impl Roll for Character {
    fn roll_slaughterer() -> Slaughterer {
        return Slaughterer {
            human: Self::generate_rep(),
            elf: Self::generate_rep(),
            dwarf: Self::generate_rep(),
        };
    }

    fn roll_saviour() -> Saviour {
        return Saviour {
            human: Self::generate_rep(),
            elf: Self::generate_rep(),
            dwarf: Self::generate_rep(),
        };
    }

    fn roll() -> Character {
        return Character {
            reputation: Reputation {
                _slaughterer: Self::roll_slaughterer(),
                _saviour: Self::roll_saviour()
            },
        };
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Races {
    Human,
    Elf,
}

#[derive(Debug, Clone)]
pub struct Slaughterer {
    pub human: f32,
    pub elf: f32,
    pub dwarf: f32,
}

#[derive(Debug, Clone)]
pub struct Saviour {
    pub human: f32,
    pub elf: f32,
    pub dwarf: f32,
}

pub trait Mappable {
    fn get_name(&self) -> String;
    fn to_hashmap(&self) -> HashMap<String, f32>;
    fn from_hashmap(map: HashMap<String, f32>) -> Self where Self: Sized;
}

impl Mappable for Slaughterer {
    fn get_name(&self) -> String {
        return "slaughterers".to_string();
    }

    fn to_hashmap(&self) -> HashMap<String, f32> {
        let mut map: HashMap<String, f32> = HashMap::new();
        map.insert("human".to_string(), self.human);
        map.insert("elf".to_string(), self.elf);
        map.insert("dwarf".to_string(), self.dwarf);
        return map;
    }

    fn from_hashmap(map: HashMap<String, f32>) -> Self {
        return Slaughterer {
            human: map
                .get("human")
                .expect("Failed to get character trait [Human Slaughterer]")
                .clone(),
            elf: map
                .get("elf")
                .expect("Failed to get character trait [Elf Slaughterer]")
                .clone(),
            dwarf: map
                .get("dwarf")
                .expect("Failed to get character trait [Dwarf Slaughterer]")
                .clone(),
        };
    }
}

impl Mappable for Saviour {
    fn get_name(&self) -> String {
        return "saviours".to_string();
    }
    fn to_hashmap(&self) -> HashMap<String, f32> {
        let mut map: HashMap<String, f32> = HashMap::new();
        map.insert("human".to_string(), self.human);
        map.insert("elf".to_string(), self.elf);
        map.insert("dwarf".to_string(), self.dwarf);
        return map;
    }

    fn from_hashmap(map: HashMap<String, f32>) -> Self {
        return Saviour {
            human: map
                .get("human")
                .expect("Failed to get character trait [Human Slaughterer]")
                .clone(),
            elf: map
                .get("elf")
                .expect("Failed to get character trait [Elf Slaughterer]")
                .clone(),
            dwarf: map
                .get("dwarf")
                .expect("Failed to get character trait [Dwarf Slaughterer]")
                .clone(),
        };
    }
}

#[derive(Debug)]
// TODO: This is used to compare the players reputation vs the NPCs knowledge. The result will represent "how well the NPC knows the player". WIP
pub struct Reputation {
    pub _slaughterer: Slaughterer,
    pub _saviour: Saviour
}
