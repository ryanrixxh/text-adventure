mod character;
use character::{Character, Roll};
mod npc;
use npc::{GameState, NPCType, NPC};

fn main() {
    // Initialise the state of the game
    let game_state = GameState::init_game_state();
    
    // Roll the character reputation
    let _player = Character::roll();

    let _npc = NPC::new(&game_state, vec![NPCType::Races(character::Races::Human), NPCType::Guard]);
}
