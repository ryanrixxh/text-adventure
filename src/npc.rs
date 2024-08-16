use crate::character::{Mappable, Races, Saviour, Slaughterer};
use crate::gamestate::GameState;
use std::collections::HashMap;

// At runtime need to inject knowledge from preset values i.e Humans and Guards
// How well do guards know slaughterers, how well do humans know them

#[derive(Debug)]
pub struct Knowledge {
    pub slaughterers: Slaughterer,
    pub saviours: Saviour, // TODO: This probably needs to be a RefCel so that it can be read properly
}

impl Knowledge {
    fn knowledge_to_hashmap(&self) -> HashMap<String, Box<dyn Mappable>> {
        let mut map: HashMap<String, Box<dyn Mappable>> = HashMap::new();
        map.insert("slaughterers".to_string(), Box::new(self.slaughterers.clone()));
        map.insert("saviours".to_string(), Box::new(self.saviours.clone()));
        return map;
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum NPCType {
    Races(Races),
    Guard,
}

pub struct NPC {
    pub _race: Races,
    pub knowledge: Knowledge,
}

impl NPC {
    pub fn new(game_state: &GameState, race: Races, types: Vec<NPCType>) -> Self {
        let mut knowledge_vec: Vec<&Knowledge> = Vec::new();

        for t in &types {
            let base = game_state.base_knowledge.get(t);

            // Pattern matching required here because we dont want Rust to panic if it doesn't get base state
            match base {
                Some(knowledge) => knowledge_vec.push(knowledge),
                None => {
                    println!("No base knowledge found for {:?}", t);
                    continue;
                }
            }
        }

        return Self {
            _race: race, // TODO: Change this once it gets read
            knowledge: Self::generate_final_knowledge(knowledge_vec),
        };
    }

    fn generate_final_knowledge(knowledge_vec: Vec<&Knowledge>) -> Knowledge {
        let first = knowledge_vec[0]; 

        // Takes the first bit of knowledge, and compares its different catagories against all other sets of knowledge
        let slaughterer_total = Self::generate_knowledge_for_type::<Slaughterer>(knowledge_vec.clone(), &first.slaughterers);
        let slaughterer: Slaughterer = Slaughterer::from_hashmap(slaughterer_total);

        let saviour_total = Self::generate_knowledge_for_type::<Saviour>(knowledge_vec.clone(), &first.saviours);
        let saviour: Saviour = Saviour::from_hashmap(saviour_total);

        return Knowledge {
            slaughterers: slaughterer,
            saviours: saviour
        };
    }
    
    /// Takes a given vector knowledge and a type of sub-knowledge. For each property in that bit of sub-knowledge, compare that same 
    /// value of sub-knowledge against all other sets of knowledge. 
    /// **Returns** - A hashmap of values representing the average of all the combined knowledge for that particular subtype
    fn generate_knowledge_for_type<T: Mappable>(knowledge_vec: Vec<&Knowledge>, knowledge_type: &T) -> HashMap<String, f32> {
        let mut sub_type_total: HashMap<String, f32> = HashMap::new();
        for (key, _value) in &knowledge_type.to_hashmap() {
        let mapped: Vec<f32> = knowledge_vec
            .clone()
            .into_iter()
            .map(|knowledge| -> f32 {
                let knowledge_map = knowledge.knowledge_to_hashmap();
                let knowledge_of_type: &Box<dyn Mappable>  = knowledge_map.get::<String>(&knowledge_type.get_name()).expect("error getting key value from knowledge_map");
                return knowledge_of_type.to_hashmap().get(key).unwrap().clone();
            })
            .collect();
        let total: f32 = mapped.iter().sum();
        let mean = total / mapped.capacity() as f32;
        sub_type_total.insert(key.to_owned(), mean);
        }
        return sub_type_total
    }
}
