use bevy::prelude::*;

/// ゲームの画面遷移ステート
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Title,
    Playing,
    GameOver,
}

// --- マーカーコンポーネント ---

/// 物理演算で動くブロック
#[derive(Component)]
pub struct Block;

/// プレイヤーが操作中のブロック（まだ落下していない）
#[derive(Component)]
pub struct ActiveBlock;

/// 安定して積み上がったブロック
#[derive(Component)]
pub struct SettledBlock;

/// ゲームプレイ中に生成されるエンティティ（cleanup用）
#[derive(Component)]
pub struct GameEntity;

/// UI ルートノード（cleanup用）
#[derive(Component)]
pub struct UiRoot;

/// スコア表示テキスト
#[derive(Component)]
pub struct ScoreText;

/// 落下位置のプレビューライン
#[derive(Component)]
pub struct PreviewIndicator;

// --- リソース ---

/// ゲーム進行状態
#[derive(Resource, Default)]
pub struct GameData {
    pub score: u32,
    pub has_active_block: bool,
    pub settle_timer: f32,
}
