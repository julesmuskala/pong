use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

use crate::{
    board::{Goal, GoalOwner},
    collider::Collider,
    scoreboard::Scoreboard,
    velocity::Velocity,
};

const BALL_STARTING_POSITION: Vec3 = Vec3::ZERO;

const BALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

const BALL_SPEED: f32 = 400.0;

const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, check_for_collisions);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<(&Transform, Option<&Goal>, Without<Ball>), With<Collider>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let (mut ball_transform, mut ball_velocity) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (transform, maybe_goal, _) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            if let Some(goal) = maybe_goal {
                ball_velocity.0 = INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED;
                ball_transform.translation.x = BALL_STARTING_POSITION.x;
                ball_transform.translation.y = BALL_STARTING_POSITION.y;

                match goal {
                    Goal {
                        goal_owner: GoalOwner::Player,
                    } => {
                        scoreboard.enemy_score += 1;
                    }
                    Goal {
                        goal_owner: GoalOwner::Enemy,
                    } => {
                        scoreboard.player_score += 1;
                    }
                }

                continue;
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

#[derive(Component)]
struct Ball;
