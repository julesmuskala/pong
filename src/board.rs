use bevy::prelude::*;

use crate::{
    ball::BallPlugin, collider::Collider, enemy::EnemyPlugin, player::PlayerPlugin,
    scoreboard::ScoreboardPlugin, velocity::apply_velocity,
};

pub const WALL_THICKNESS: f32 = 10.0;
const GOAL_THICKNESS: f32 = WALL_THICKNESS * 1.5;

// x coordinates
pub const LEFT_WALL: f32 = -450.0;
pub const RIGHT_WALL: f32 = 450.0;
// y coordinates
pub const BOTTOM_WALL: f32 = -250.0;
pub const TOP_WALL: f32 = 250.0;

const GOAL_HEIGH: f32 = 150.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugins((PlayerPlugin, EnemyPlugin, BallPlugin, ScoreboardPlugin))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, apply_velocity);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    commands.spawn(GoalBundle::new(GoalLocation::Left, GoalOwner::Player));
    commands.spawn(GoalBundle::new(GoalLocation::Right, GoalOwner::Enemy));
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Bundle)]
struct GoalBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    goal: Goal,
}

#[derive(Component)]
pub struct Goal {
    pub goal_owner: GoalOwner,
}

enum GoalLocation {
    Left,
    Right,
}

pub enum GoalOwner {
    Player,
    Enemy,
}

impl GoalLocation {
    fn position(&self) -> Vec2 {
        match self {
            GoalLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            GoalLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
        }
    }

    fn size(&self) -> Vec2 {
        Vec2::new(GOAL_THICKNESS, GOAL_HEIGH + GOAL_THICKNESS)
    }
}

impl GoalBundle {
    fn new(location: GoalLocation, goal_owner: GoalOwner) -> GoalBundle {
        GoalBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: BACKGROUND_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
            goal: Goal { goal_owner },
        }
    }
}
