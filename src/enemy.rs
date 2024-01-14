use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    board::{BOTTOM_WALL, TOP_WALL, WALL_THICKNESS},
    paddle::{PaddleBundle, PaddleColor, PaddleLocation, PADDLE_HEIGHT},
};

const MOVEMENT_SPEED: f32 = 330.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_enemy);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(EnemyBundle::new());
}

#[derive(Component)]
struct EnemyController {
    direction: EnemyDirection,
}

enum EnemyDirection {
    Up,
    Down,
}

impl Distribution<EnemyDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyDirection {
        match rng.gen_range(0..=1) {
            0 => EnemyDirection::Up,
            _ => EnemyDirection::Down,
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    paddle_bundle: PaddleBundle,
    enemy_controller: EnemyController,
}

impl EnemyBundle {
    fn new() -> EnemyBundle {
        EnemyBundle {
            paddle_bundle: PaddleBundle::new(PaddleLocation::Right, PaddleColor::Enemy),
            enemy_controller: EnemyController {
                direction: rand::random(),
            },
        }
    }
}

fn move_enemy(mut query: Query<(&mut Transform, &mut EnemyController)>, time: Res<Time>) {
    let mut paddle = query.single_mut();

    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_HEIGHT / 2.0;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_HEIGHT / 2.0;

    let direction = match paddle.1.direction {
        EnemyDirection::Up => {
            if paddle.0.translation.y == top_bound {
                paddle.1.direction = EnemyDirection::Down;
            }

            1.0
        }
        EnemyDirection::Down => {
            if paddle.0.translation.y == bottom_bound {
                paddle.1.direction = EnemyDirection::Up;
            }

            -1.0
        }
    };

    let new_paddle_position =
        paddle.0.translation.y + direction * MOVEMENT_SPEED * time.delta_seconds();

    paddle.0.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}
