use bevy::{app::PluginGroupBuilder, prelude::*};

use self::lighting::LightingPlugin;
use self::track::TrackPlugin;

mod lighting;
mod track;

pub struct EnvironmentPlugins;

impl PluginGroup for EnvironmentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LightingPlugin)
            .add(TrackPlugin)
    }
}
