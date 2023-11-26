use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, States, Debug)]
pub enum GameState {
    #[default]
    Menu,
    PlayerTurn,
    AiTurn,
    End,
}
