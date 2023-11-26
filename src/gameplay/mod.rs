use bevy::{app::PluginGroupBuilder, prelude::*};

use self::player::PlayerPlugin;

mod player;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerPlugin)
    }
}
