use bevy::prelude::{shape::Quad, *};
use bevy_xpbd_3d::prelude::*;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let quad = Quad {
        size: Vec2::new(80.0, 5.0),
        ..default()
    };

    let wall_mesh = meshes.add(Mesh::from(shape::Quad::from(quad)));

    let white = materials.add(Color::WHITE.into());

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(80.0))),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(80.0, 0.005, 80.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: wall_mesh.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(2.5, 2.5, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(80.0, 5.0, 0.005),
    ));

    commands.spawn((
        PbrBundle {
            mesh: wall_mesh,
            material: white,
            transform: Transform::from_xyz(-2.5, 2.5, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(80.0, 5.0, 0.005),
    ));
}
