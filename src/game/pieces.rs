use bevy::{prelude::*, utils::HashSet};
use strum::IntoEnumIterator;

use crate::player::Player;

use crate::game::{
    structs::{BevyPiece, BevyReversi},
    utils::*,
};

pub(crate) fn draw_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut queries: Query<(Entity, &Transform, &mut Handle<StandardMaterial>), With<BevyPiece>>,
    game: Res<BevyReversi>,
) {
    let mut drawn_pieces = HashSet::new();
    for query in queries.iter_mut() {
        let (mut piece, transform, mut material) = (commands.entity(query.0), query.1, query.2);
        let Vec3 { x, z, .. } = transform.translation;
        let coord = game_coord_to_reversi_coord((x, z));
        let player = game.0.board().get(coord);
        if let Some(player) = player {
            let color = get_color_for_player(player);
            material.apply(materials.add(color.into()).as_reflect());
            drawn_pieces.insert(coord);
        } else {
            piece.despawn();
        }
    }
    for player in Player::iter() {
        for coord in game.0.board().pieces_for_player(player) {
            if drawn_pieces.contains(&coord) {
                continue;
            }

            // better to have this here since only 1 player per round will be placing a new piece (except first round)
            let color = get_color_for_player(player);
            let (x, z) = reversi_coord_to_game_coord(coord);
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::UVSphere::default().into()),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(x, 1.0, z).with_scale(Vec3::new(0.5, 0.5, 0.5)),
                    ..default()
                },
                BevyPiece,
            ));
        }
    }
}

fn get_color_for_player(player: Player) -> Color {
    match player {
        Player::Green => Color::GREEN,
        Player::Red => Color::RED,
    }
}
