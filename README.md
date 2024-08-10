# Rust Text Adventure

This text adventure is currently attempting to implement a dynamic NPC <-> Player to dialogue system based on the reputation of the player vs the knowledge and world view of the NPC 
they are talking to.

## How it works

Game state is stored as a data structure containing the details of base knowledge that NPCs of a certain type would have without any player interaction. 
When a new NPC is initialised, it will read from game state to develop its own set of knowledge based on combination of base knowledge dependant on the type of NPC it is. 

When player is initialised, it will be rolled a random reputation.

Upon interaction with an player, the NPC will determine its Recognition (how well it knows the player) based on its Knowledge and the reputation of the player, and its Opinion (how it feels about the player) based on its WorldView and the Reputation of the player. (**WIP**)

These two data structures will be used to determine how the NPC responds to interaction with the player (**WIP**)

**This repository is very early and experimental. It is just as much a learning experience for the Rust language, as it is a project to make a small text-based game**