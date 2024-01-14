use bevy::prelude::*;

use crate::board::{LEFT_WALL, RIGHT_WALL};
use crate::collider::Collider;

const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 60.0;

pub const PADDLE_HEIGHT: f32 = 120.0;
pub const PADDLE_WIDTH: f32 = 40.0;

const PADDLE_PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_ENEMY_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);

#[derive(Bundle)]
pub struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

pub enum PaddleLocation {
    Left,
    Right,
}

pub enum PaddleColor {
    Player,
    Enemy,
}

impl PaddleBundle {
    pub fn new(location: PaddleLocation, color: PaddleColor) -> PaddleBundle {
        PaddleBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: color.color(),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

impl PaddleLocation {
    fn position(&self) -> Vec2 {
        match self {
            PaddleLocation::Left => Vec2::new(LEFT_WALL + GAP_BETWEEN_PADDLE_AND_WALL, 0.0),
            PaddleLocation::Right => Vec2::new(RIGHT_WALL - GAP_BETWEEN_PADDLE_AND_WALL, 0.0),
        }
    }

    fn size(&self) -> Vec2 {
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
    }
}

impl PaddleColor {
    fn color(&self) -> Color {
        match self {
            PaddleColor::Player => PADDLE_PLAYER_COLOR,
            PaddleColor::Enemy => PADDLE_ENEMY_COLOR,
        }
    }
}
