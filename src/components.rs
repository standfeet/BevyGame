use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Title,
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct ActiveBlock;

#[derive(Component)]
pub struct SettledBlock;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct PreviewIndicator;

#[derive(Resource, Default)]
pub struct GameData {
    pub score: u32,
    pub has_active_block: bool,
    pub settle_timer: f32,
}

#[derive(Resource)]
pub struct BlockColors(pub Vec<Color>);
