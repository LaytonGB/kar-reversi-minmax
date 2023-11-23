use bevy::prelude::*;

use crate::bevy_structs::{Board, Position};

pub fn draw_board(mut commands: Commands) {
    commands.spawn((Board, Position { x: 0, y: 0, z: 0 }));
}
