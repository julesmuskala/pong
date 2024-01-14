use bevy::prelude::*;

use crate::board::{BOTTOM_WALL, TOP_WALL, WALL_THICKNESS};
use crate::paddle::PADDLE_HEIGHT;
use crate::paddle::{PaddleBundle, PaddleColor, PaddleLocation};

const MOVEMENT_SPEED: f32 = 700.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PlayerBundle::new());
}

#[derive(Component)]
struct PlayerController;

#[derive(Bundle)]
struct PlayerBundle {
    paddle_bundle: PaddleBundle,
    player_controller: PlayerController,
}

impl PlayerBundle {
    fn new() -> PlayerBundle {
        PlayerBundle {
            paddle_bundle: PaddleBundle::new(PaddleLocation::Left, PaddleColor::Player),
            player_controller: PlayerController,
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerController>>,
    time: Res<Time>,
) {
    let mut paddle = query.single_mut();

    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_HEIGHT / 2.0;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_HEIGHT / 2.0;

    let direction = if keyboard_input.pressed(KeyCode::Up) {
        1.0
    } else if keyboard_input.pressed(KeyCode::Down) {
        -1.0
    } else {
        0.0
    };

    let new_paddle_position =
        paddle.translation.y + direction * MOVEMENT_SPEED * time.delta_seconds();

    paddle.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}
