use bevy::{app::PluginGroupBuilder, prelude::*};

use self::health_bar::HealthBarPlugin;

mod health_bar;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HealthBarPlugin)
    }
}
