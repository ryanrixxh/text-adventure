// TODO Clean these dependencies up. Gamestate should have races, and possibly knowledge struct
use crate::character::{Races, Saviour, Slaughterer};
use crate::npc::{Knowledge, NPCType};
use std::collections::HashMap;
pub struct GameState {
    pub base_knowledge: HashMap<NPCType, Knowledge>,
}

impl GameState {
    /**
     * The base knowledge of all NPCs throughout the world. NPCs will inherit this knowledge depending on their type.
     */
    fn init_base_knowledge() -> HashMap<NPCType, Knowledge> {
        return HashMap::from([
            (
                NPCType::Races(Races::Human),
                Knowledge {
                    slaughterers: Slaughterer {
                        human: 0.8,
                        elf: 0.5,
                        dwarf: 0.4,
                    },
                    saviours: Saviour {
                        human: 1.0,
                        elf: 0.4,
                        dwarf: 0.2
                    }
                },
            ),
            (
                NPCType::Guard,
                Knowledge {
                    slaughterers: Slaughterer {
                        human: 0.7,
                        elf: 0.7,
                        dwarf: 0.7,
                    },
                    saviours: Saviour {
                        human: 0.3,
                        elf: 0.3,
                        dwarf: 0.3
                    }
                },
            ),
            (
                NPCType::Races(Races::Elf),
                Knowledge {
                    slaughterers: Slaughterer {
                        human: 0.5,
                        elf: 0.9,
                        dwarf: 0.2,
                    },
                    saviours: Saviour {
                        human: 0.4,
                        elf: 1.0,
                        dwarf: 0.1,
                    }
                },
            ),
        ]);
    }

    pub fn init_game_state() -> Self {
        Self {
            base_knowledge: Self::init_base_knowledge(),
        }
    }
}
