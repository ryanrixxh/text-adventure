use std::collections::HashMap;
use crate::character::Races;

// At runtime need to inject knowledge from preset values i.e Humans and Guards
// How well do guards know slaughterers, how well do humans know them 

#[derive(Debug)]
pub struct Knowledge {
    pub _slaughterers: HashMap<Races, f32> // TODO: This probably needs to be a RefCel so that it can be read properly
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum NPCType {
    Races(Races),
    Guard,
}


pub struct GameState {
    pub base_knowledge: HashMap<NPCType, Knowledge>,
}

impl GameState {
    fn init_base_knowledge() -> HashMap<NPCType, Knowledge> {
        return HashMap::from([
            (NPCType::Races(Races::Human), Knowledge {
                _slaughterers: HashMap::from([
                    (Races::Human, 0.8), // A human is most likely to know about someone how has a reputation for slaughtering humans
                    (Races::Elf, 0.3),
                    (Races::Dwarf, 0.1)
                ])
            }),
            (NPCType::Guard, Knowledge {
                _slaughterers: HashMap::from([
                    (Races::Human, 0.7),
                    (Races::Elf, 0.7),
                    (Races::Dwarf, 0.7), // Guards are fairly likely to know about slaughterers
                ])
            }),
        ])
    }

    pub fn init_game_state() -> Self {
        Self {
            base_knowledge: Self::init_base_knowledge()
        }
    }
}


// pub trait Evaluate {
//     fn form_recognition(reputation: character::Reputation, _knowledge: Knowledge) -> i32 {
//         let slaughterer: &character::Slaughterer = &reputation._slaughterer;
//         let _total = 0;
//         for (key, value) in slaughterer {
//             let _key_knowledge = _knowledge.slaughterers.get(key);

//         }
//         let mean = _total / slaughterer.capacity() as i32;
//         return mean;
//     }
//     fn form_opinion(reputation: character::Reputation) -> Opinion;
// }

// pub struct Opinion {
//     pub like: i32,
//     pub dislike: i32,
// }

pub struct NPC {
    pub _race: Races,
    pub _knowledge: Knowledge,
}

// TODO Need to know what base_knowledge to grab from gamestate in order to fill out the actual knowledge.
// How the hell are we gonna use the NPCType on the tuple for each of the properties with ANOTHER hashmap. Yikes.
impl NPC {
    pub fn new(game_state: &GameState, types: Vec<NPCType>) {
        for t in &types {
            let base = game_state.base_knowledge.get(t);
            println!("{:?}", base);
        }
    }
}