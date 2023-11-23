use bevy::prelude::*;

#[derive(Component)]
pub struct Coord {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Board;
