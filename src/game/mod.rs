use crate::game::primary_logic::GameLogicPlugin;
use crate::game::textcolor_smoothing::TextColorSmoothingPlugin;
use crate::utils::buttoning::ButtoningPlugin;
use crate::utils::position_smoothing::PositionSmoothingPlugin;
use crate::utils::responsive::ResponsivePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::{Plugin, PluginGroup};

pub(self) mod internal;
pub(self) mod primary_logic;
pub(self) mod textcolor_smoothing;

#[derive(Default)]
pub struct GamePlugins;

#[derive(Default)]
struct GameDependencyPlugins;

impl PluginGroup for GameDependencyPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<GameDependencyPlugins>()
            .add(PositionSmoothingPlugin)
            .add(ResponsivePlugin)
            .add(ButtoningPlugin)
    }
}

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<GamePlugins>()
            .add_group(GameDependencyPlugins)
            .add(GameLogicPlugin)
            .add(TextColorSmoothingPlugin)
    }
}
