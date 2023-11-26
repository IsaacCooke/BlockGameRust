use bevy::prelude::*;
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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(80.0))),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(80.0, 0.005, 80.0),
    ));
}
