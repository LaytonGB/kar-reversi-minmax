use bevy::prelude::*;

use crate::reversi::Reversi;

#[derive(Component)]
pub(crate) struct BevyPiece;

#[derive(Resource, Default, Debug)]
pub(crate) struct BevyReversi(pub Reversi);
