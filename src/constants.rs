// ゲームエリア
pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 640.0;

// フォント
pub const FONT_PATH: &str = "fonts/NotoSansJP-Regular.otf";

// 地形
pub const GROUND_Y: f32 = -GAME_HEIGHT / 2.0 + 30.0;
pub const WALL_THICKNESS: f32 = 18.0;
pub const WALL_HEIGHT: f32 = 180.0;

// ブロック
pub const SPAWN_Y: f32 = GAME_HEIGHT / 2.0 - 50.0;
pub const DROP_ZONE_WIDTH: f32 = GAME_WIDTH - WALL_THICKNESS * 2.0;
pub const BLOCK_MIN_SIZE: f32 = 45.0;
pub const BLOCK_MAX_SIZE: f32 = 90.0;
pub const TRAPEZOID_TOP_RATIO: f32 = 0.6;

// ゲームロジック
pub const DEATH_Y: f32 = GROUND_Y - 80.0;
pub const SETTLE_VELOCITY_THRESHOLD: f32 = 5.0;
pub const SETTLE_WAIT_SECS: f32 = 1.2;

// 物理演算
pub const PHYSICS_HZ: f64 = 120.0;
pub const PHYSICS_LENGTH_UNIT: f32 = 50.0;
pub const PHYSICS_SUBSTEPS: u32 = 20;
pub const GRAVITY_STRENGTH: f32 = 490.0;
pub const GROUND_FRICTION: f32 = 0.9;
pub const WALL_FRICTION: f32 = 0.5;
pub const BLOCK_FRICTION: f32 = 0.8;

// カラーパレット
pub const BLOCK_COLORS: &[[f32; 3]] = &[
    [0.93, 0.35, 0.35], // 赤
    [0.35, 0.75, 0.93], // 青
    [0.45, 0.90, 0.45], // 緑
    [0.95, 0.80, 0.25], // 黄
    [0.85, 0.50, 0.85], // 紫
    [1.00, 0.60, 0.30], // オレンジ
];
