use bevy::prelude::*;

mod ball;
pub mod board;
mod collider;
mod enemy;
mod paddle;
mod player;
mod scoreboard;
mod velocity;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
