mod character;
use character::{Character, Roll};

mod npc;
use npc::{NPCType, NPC};

mod gamestate;
use gamestate::GameState;

fn main() {
    // Initialise the state of the game
    let game_state = GameState::init_game_state();

    // Roll the character reputation
    let player = Character::roll();

    // Create a new Human Guard npc
    let human_guard = NPC::new(
        &game_state,
        character::Races::Human,
        vec![NPCType::Races(character::Races::Human), NPCType::Guard],
    );

    // Determine whether the guard recognises the player
    human_guard.generate_recognition(&player.reputation)
}
