mod block;
mod components;
mod constants;
mod systems;

use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use components::*;
use constants::*;
use systems::*;

fn main() {
    App::new()
        // --- Bevy プラグイン ---
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
        // --- 物理演算 ---
        .add_plugins(PhysicsPlugins::new(FixedUpdate).with_length_unit(PHYSICS_LENGTH_UNIT))
        .add_plugins(PhysicsDebugPlugin)
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_HZ))
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_STRENGTH))
        .insert_resource(SubstepCount(PHYSICS_SUBSTEPS))
        // --- ゲームリソース ---
        .init_resource::<GameData>()
        .init_state::<GameState>()
        // --- システム登録 ---
        .add_systems(Startup, (setup_camera, hide_loading_screen))
        .add_systems(OnEnter(GameState::Title), setup_title)
        .add_systems(Update, title_input.run_if(in_state(GameState::Title)))
        .add_systems(OnExit(GameState::Title), cleanup_ui)
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
        .add_systems(OnEnter(GameState::GameOver), setup_gameover)
        .add_systems(Update, gameover_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_ui)
        .run();
}
