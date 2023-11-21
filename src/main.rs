use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod environment;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .run();
}
