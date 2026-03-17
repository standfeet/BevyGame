mod block;
mod components;
mod constants;
mod systems;

use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use components::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "積み木ゲーム".to_string(),
                        canvas: Some("#bevy-canvas".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(PhysicsPlugins::new(FixedUpdate).with_length_unit(50.0))
        .add_plugins(PhysicsDebugPlugin)
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        .insert_resource(Gravity(Vec2::NEG_Y * 490.0))
        .insert_resource(SubstepCount(50))
        .insert_resource(BlockColors(vec![
            Color::srgb(0.93, 0.35, 0.35),
            Color::srgb(0.35, 0.75, 0.93),
            Color::srgb(0.45, 0.90, 0.45),
            Color::srgb(0.95, 0.80, 0.25),
            Color::srgb(0.85, 0.50, 0.85),
            Color::srgb(1.00, 0.60, 0.30),
        ]))
        .init_resource::<GameData>()
        .init_state::<GameState>()
        // 起動
        .add_systems(Startup, (setup_camera, hide_loading_screen))
        // タイトル
        .add_systems(OnEnter(GameState::Title), setup_title)
        .add_systems(Update, title_input.run_if(in_state(GameState::Title)))
        .add_systems(OnExit(GameState::Title), cleanup_ui)
        // プレイ
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update,
            (
                move_active_block,
                drop_block,
                check_block_settled,
                spawn_next_block,
                check_death,
                update_score_text,
                update_preview,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        // ゲームオーバー
        .add_systems(OnEnter(GameState::GameOver), setup_gameover)
        .add_systems(Update, gameover_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_ui)
        .run();
}
