pub mod board;
pub mod bot;
pub mod bot_algorithm;
pub mod bot_difficulty;
pub mod constants;
pub mod history;
pub mod player;
pub mod reversi;

#[cfg(feature = "terminal")]
pub mod utils;

#[cfg(feature = "game")]
pub(crate) mod bevy_interactions;
#[cfg(feature = "game")]
pub(crate) mod bevy_pieces;
#[cfg(feature = "game")]
pub(crate) mod bevy_scene;
#[cfg(feature = "game")]
pub(crate) mod bevy_structs;
#[cfg(feature = "game")]
pub mod game;
