pub const FONT_PATH: &str = "fonts/NotoSansJP-Regular.otf";

pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 640.0;
pub const GROUND_Y: f32 = -GAME_HEIGHT / 2.0 + 30.0;
pub const WALL_THICKNESS: f32 = 18.0;
pub const WALL_HEIGHT: f32 = 180.0;
pub const SPAWN_Y: f32 = GAME_HEIGHT / 2.0 - 50.0;
pub const DROP_ZONE_WIDTH: f32 = GAME_WIDTH - WALL_THICKNESS * 2.0;
pub const DEATH_Y: f32 = GROUND_Y - 80.0;
pub const BLOCK_SETTLE_TIME: f32 = 1.2;
pub const BLOCK_MIN_SIZE: f32 = 45.0;
pub const BLOCK_MAX_SIZE: f32 = 90.0;
