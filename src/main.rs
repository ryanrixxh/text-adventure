mod character;
use character::{Character, Roll};
mod npc;
use npc::{GameState, NPCType, NPC};

fn main() {
    // Initialise the state of the game
    let game_state = GameState::init_game_state();

    // Roll the character reputation
    let _player = Character::roll();

    let npc = NPC::new(
        &game_state,
        character::Races::Human,
        vec![NPCType::Races(character::Races::Human), NPCType::Guard],
    );

    println!(
        "This npcs knowledge of the world is: \n{:?}",
        npc._knowledge
    )
}
