use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*, PhysicsSchedule, PhysicsStepSet};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PhysicsSchedule, movement.before(PhysicsStepSet::BroadPhase));
        app.add_systems(Update, sync_player_camera);
    }
}

#[derive(Component)]
pub struct Player {
    pub health: i32,
}

#[derive(Component)]
struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(1.0, 0.4),
        // Prevent the player from falling over
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        // Cast the player shape downwards to detect when the player is grounded
        ShapeCaster::new(
            Collider::capsule(0.9, 0.35),
            Vector::NEG_Y * 0.05,
            Quaternion::default(),
            Vector::NEG_Y,
        )
        .with_max_time_of_impact(0.2)
        .with_max_hits(1),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        Player { health: 100 },
    ));

    let translation = Vec3::new(0.0, 2.5, 5.0);
    let radius = translation.length();

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius,
            ..default()
        },
    ));
}

fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut PanOrbitCamera, &mut Transform), Without<Player>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut LinearVelocity, &ShapeHits), With<Player>>,
) {
    for (mut linear_velocity, ground_hits) in &mut players {
        // Directional movement
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            linear_velocity.z -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            linear_velocity.z += 1.2;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            linear_velocity.x -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            linear_velocity.x += 1.2;
        }

        // Jump if space pressed and the player is close enough to the ground
        if keyboard_input.just_pressed(KeyCode::Space) && !ground_hits.is_empty() {
            linear_velocity.y += 8.0;
        }

        // Slow player down on the x and y axes
        linear_velocity.x *= 0.8;
        linear_velocity.z *= 0.8;
    }
}
