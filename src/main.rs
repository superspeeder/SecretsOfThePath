mod game;
mod utils;

use crate::game::GamePlugins;
use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugins)).run();
}
