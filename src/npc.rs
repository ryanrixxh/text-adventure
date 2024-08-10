use std::collections::HashMap;
use crate::character::Races;

// At runtime need to inject knowledge from preset values i.e Humans and Guards
// How well do guards know slaughterers, how well do humans know them 

#[derive(Debug)]
pub struct Knowledge {
    pub slaughterers: HashMap<Races, f32> // TODO: This probably needs to be a RefCel so that it can be read properly
}

impl Knowledge {
    pub fn new() -> Self {
        Knowledge {
            slaughterers: HashMap::new(),
        }
    }
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
                slaughterers: HashMap::from([
                    (Races::Human, 0.8), // A human is most likely to know about someone who has a reputation for slaughtering humans
                    (Races::Elf, 0.3),
                    (Races::Dwarf, 0.1)
                ])
            }),
            (NPCType::Guard, Knowledge {
                slaughterers: HashMap::from([
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

pub struct NPC {
    pub _race: Races,
    pub _knowledge: Knowledge,
}

// TODO Need to know what base_knowledge to grab from gamestate in order to fill out the actual knowledge.
// How the hell are we gonna use the NPCType on the tuple for each of the properties with ANOTHER hashmap. Yikes.
impl NPC {
    pub fn new(game_state: &GameState, race: Races, types: Vec<NPCType>) -> Self {
        let mut knowledge_vec: Vec<&Knowledge> = Vec::new();

        for t in &types {
            let base = game_state.base_knowledge.get(t);

            // Pattern matching required here because we dont want Rust to panic if it doesn't get base state
            match base {
                Some(knowledge) => {
                    knowledge_vec.push(knowledge)
                },
                None => {
                    println!("No base knowledge found for {:?}", t);
                    continue
                }
            }
        }

        return Self {
            _race: race, // TODO: Change this once it gets read
            _knowledge: Self::generate_final_knowledge(knowledge_vec)
        }
    }
    
    // TODO: This needs to change to iterate through all of the bits of knowledge that gets grabbed from base not just first two.
    //       Will require a bit of a major rethink of this function
    fn generate_final_knowledge(knowledge_vec: Vec<&Knowledge>) -> Knowledge {
        let first = knowledge_vec[0];
        let second = knowledge_vec[1];
        let mut final_knowledge = Knowledge::new();
        
        for (key, value) in &first.slaughterers {
            let second_at_key = second.slaughterers.get(key).unwrap_or(&0.0);
            let mean = (value + second_at_key) / 2.0;
            final_knowledge.slaughterers.insert(key.clone(), mean);
        }

        return final_knowledge;
    } 
}