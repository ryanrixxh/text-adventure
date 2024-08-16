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
    let _player = Character::roll();

    let human_guard = NPC::new(
        &game_state,
        character::Races::Human,
        vec![NPCType::Races(character::Races::Human), NPCType::Guard],
    );

    let elf_guard = NPC::new(
        &game_state,
        character::Races::Elf,
        vec![NPCType::Races(character::Races::Elf), NPCType::Guard],
    );

    println!(
        "This human guards knowledge of the world is: \n{:?}",
        human_guard.knowledge
    );

    println!(
        "This elf guards knowledge of the world is: \n{:?}",
        elf_guard.knowledge
    );
}
