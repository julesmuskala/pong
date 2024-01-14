use bevy::prelude::*;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const PLAYER_SCORE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const ENEMY_SCORE_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard {
            player_score: 0,
            enemy_score: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_scoreboard);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(ScoreTextBundle::new(ScoreOwner::Player));
    commands.spawn(ScoreTextBundle::new(ScoreOwner::Enemy));
}

#[derive(Bundle)]
struct ScoreTextBundle {
    text_bundle: TextBundle,
    score_owner: ScoreOwner,
}

impl ScoreTextBundle {
    fn new(score_owner: ScoreOwner) -> ScoreTextBundle {
        ScoreTextBundle {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    match score_owner {
                        ScoreOwner::Player => "Player: ",
                        ScoreOwner::Enemy => "Enemy: ",
                    },
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: match score_owner {
                            ScoreOwner::Player => PLAYER_SCORE_COLOR,
                            ScoreOwner::Enemy => ENEMY_SCORE_COLOR,
                        },
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: match score_owner {
                        ScoreOwner::Player => PLAYER_SCORE_COLOR,
                        ScoreOwner::Enemy => ENEMY_SCORE_COLOR,
                    },
                    ..default()
                }),
            ])
            .with_style(match score_owner {
                ScoreOwner::Player => Style {
                    position_type: PositionType::Absolute,
                    top: SCOREBOARD_TEXT_PADDING,
                    left: SCOREBOARD_TEXT_PADDING,
                    ..default()
                },
                ScoreOwner::Enemy => Style {
                    position_type: PositionType::Absolute,
                    top: SCOREBOARD_TEXT_PADDING,
                    right: SCOREBOARD_TEXT_PADDING,
                    ..default()
                },
            }),
            score_owner,
        }
    }
}

#[derive(Component)]
enum ScoreOwner {
    Player,
    Enemy,
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<(&mut Text, &ScoreOwner)>) {
    for (mut text, score_owner) in &mut query {
        text.sections[1].value = match score_owner {
            ScoreOwner::Player => scoreboard.player_score.to_string(),
            ScoreOwner::Enemy => scoreboard.enemy_score.to_string(),
        }
    }
}

#[derive(Resource)]
pub struct Scoreboard {
    pub player_score: usize,
    pub enemy_score: usize,
}
