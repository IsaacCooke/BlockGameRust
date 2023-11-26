use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::environment::EnvironmentPlugins;
use crate::gameplay::GameplayPlugins;

mod environment;
mod gameplay;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            EnvironmentPlugins,
            GameplayPlugins,
        ))
        .run();
}
