use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });
}
