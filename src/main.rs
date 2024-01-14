use bevy::prelude::*;

use pong::{board::BoardPlugin, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BoardPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
