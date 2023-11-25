use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
};

use crate::{
    bevy_highlight_constants::GRID_HIGHLIGHT, bevy_interactions::click_grid_square,
    bevy_structs::BevySquare,
};

/// set up a simple 3D scene
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // game board base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 2.0, 10.0).into()),
        material: materials.add(Color::DARK_GRAY.into()),
        ..default()
    });
    // game board grid
    for x in -4..4 {
        for z in -4..4 {
            let color = if (x % 2_i32).abs() == (z % 2_i32).abs() {
                Color::WHITE
            } else {
                Color::BLACK
            };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Box::new(1.0, 2.0, 1.0).into()),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.2, z as f32 + 0.5),
                    ..default()
                },
                BevySquare,
                GRID_HIGHLIGHT,
                On::<Pointer<Click>>::target_commands_mut(click_grid_square),
            ));
        }
    }
    // background
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(30.0, 1.0, 30.0).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, -4.0),
        ..default()
    });
}
