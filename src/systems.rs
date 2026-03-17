use avian2d::prelude::*;
use bevy::prelude::*;

use crate::block::spawn_block;
use crate::components::*;
use crate::constants::*;

// === 起動 ===

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn hide_loading_screen() {
    use web_sys::wasm_bindgen::JsCast;

    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    if let Some(el) = document.get_element_by_id("loading-screen") {
        let html_el: web_sys::HtmlElement = el.unchecked_into();
        let _ = html_el.class_list().add_1("fade-out");
    }
}

// === タイトル画面 ===

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("積み木ゲーム"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.85, 0.3)),
            ));
            parent.spawn((
                Text::new("クリック or タップ でスタート"),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn title_input(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse.just_released(MouseButton::Left) || touches.any_just_released() {
        next_state.set(GameState::Playing);
    }
}

// === ゲームプレイ ===

pub fn setup_game(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    asset_server: Res<AssetServer>,
) {
    *game_data = GameData::default();
    let font = asset_server.load(FONT_PATH);

    // 地面
    commands.spawn((
        GameEntity,
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.35),
            custom_size: Some(Vec2::new(GAME_WIDTH, WALL_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
        RigidBody::Static,
        Collider::rectangle(GAME_WIDTH, WALL_THICKNESS),
        Friction::new(0.9),
        Restitution::new(0.0),
    ));

    let ground_top = GROUND_Y + WALL_THICKNESS / 2.0;
    let wall_center_y = ground_top + WALL_HEIGHT / 2.0;

    // 左壁
    commands.spawn((
        GameEntity,
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.3),
            custom_size: Some(Vec2::new(WALL_THICKNESS, WALL_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-GAME_WIDTH / 2.0 + WALL_THICKNESS / 2.0, wall_center_y, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_HEIGHT),
        Friction::new(0.5),
        Restitution::new(0.0),
    ));

    // 右壁
    commands.spawn((
        GameEntity,
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.3),
            custom_size: Some(Vec2::new(WALL_THICKNESS, WALL_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(GAME_WIDTH / 2.0 - WALL_THICKNESS / 2.0, wall_center_y, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_HEIGHT),
        Friction::new(0.5),
        Restitution::new(0.0),
    ));

    // 背景
    commands.spawn((
        GameEntity,
        Sprite {
            color: Color::srgb(0.08, 0.08, 0.12),
            custom_size: Some(Vec2::new(GAME_WIDTH, GAME_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // スコアUI
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                Text::new("スコア: 0"),
                TextFont {
                    font,
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    // プレビューライン
    commands.spawn((
        GameEntity,
        PreviewIndicator,
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.15),
            custom_size: Some(Vec2::new(2.0, GAME_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -5.0),
        Visibility::Hidden,
    ));
}

pub fn move_active_block(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    touches: Res<Touches>,
    mut active_blocks: Query<&mut Transform, With<ActiveBlock>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, cam_transform)) = cameras.single() else { return };

    // タッチ中の指の位置 or マウス位置を取得
    let screen_pos = touches
        .iter()
        .next()
        .map(|t| t.position())
        .or_else(|| window.cursor_position());

    let Some(pos) = screen_pos else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, pos) else {
        return;
    };

    let half_zone = DROP_ZONE_WIDTH / 2.0 - 10.0;
    let clamped_x = world_pos.x.clamp(-half_zone, half_zone);

    for mut transform in &mut active_blocks {
        transform.translation.x = clamped_x;
    }
}

pub fn drop_block(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    active_blocks: Query<Entity, With<ActiveBlock>>,
    mut game_data: ResMut<GameData>,
) {
    // マウス: クリックで落下
    // タッチ: 指を離した時に落下（ドラッグで位置調整してからリリース）
    if !mouse.just_pressed(MouseButton::Left) && !touches.any_just_released() {
        return;
    }

    for entity in &active_blocks {
        commands
            .entity(entity)
            .remove::<ActiveBlock>()
            .remove::<RigidBody>()
            .insert(RigidBody::Dynamic);
        game_data.has_active_block = false;
        game_data.settle_timer = 0.0;
    }
}

#[allow(clippy::type_complexity)]
pub fn check_block_settled(
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
    blocks: Query<&LinearVelocity, (With<Block>, Without<ActiveBlock>, Without<SettledBlock>)>,
    mut commands: Commands,
    unsettled: Query<Entity, (With<Block>, Without<ActiveBlock>, Without<SettledBlock>)>,
) {
    if game_data.has_active_block {
        return;
    }

    let all_slow = blocks.iter().all(|vel| vel.length() < 5.0);

    if all_slow && !blocks.is_empty() {
        game_data.settle_timer += time.delta_secs();
        if game_data.settle_timer > BLOCK_SETTLE_TIME {
            for entity in &unsettled {
                commands.entity(entity).insert(SettledBlock);
            }
            game_data.score += 1;
            game_data.settle_timer = 0.0;
        }
    } else {
        game_data.settle_timer = 0.0;
    }
}

#[allow(clippy::type_complexity)]
pub fn spawn_next_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    colors: Res<BlockColors>,
    mut game_data: ResMut<GameData>,
    active_blocks: Query<&ActiveBlock>,
    unsettled: Query<Entity, (With<Block>, Without<ActiveBlock>, Without<SettledBlock>)>,
) {
    if game_data.has_active_block || !active_blocks.is_empty() {
        return;
    }
    if !unsettled.is_empty() {
        return;
    }

    spawn_block(&mut commands, &mut meshes, &mut materials, &colors, 0.0);
    game_data.has_active_block = true;
}

pub fn check_death(
    blocks: Query<&Transform, (With<Block>, Without<ActiveBlock>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for transform in &blocks {
        if transform.translation.y < DEATH_Y {
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

pub fn update_score_text(game_data: Res<GameData>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        **text = format!("スコア: {}", game_data.score);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_preview(
    active_blocks: Query<&Transform, With<ActiveBlock>>,
    mut preview: Query<
        (&mut Transform, &mut Visibility),
        (With<PreviewIndicator>, Without<ActiveBlock>),
    >,
) {
    let Ok((mut preview_transform, mut visibility)) = preview.single_mut() else {
        return;
    };

    if let Some(block_transform) = active_blocks.iter().next() {
        *visibility = Visibility::Visible;
        preview_transform.translation.x = block_transform.translation.x;
    } else {
        *visibility = Visibility::Hidden;
    }
}

pub fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
    ui: Query<Entity, With<UiRoot>>,
) {
    for entity in entities.iter().chain(ui.iter()) {
        commands.entity(entity).despawn();
    }
}

// === ゲームオーバー画面 ===

pub fn setup_gameover(
    mut commands: Commands,
    game_data: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(FONT_PATH);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("ゲームオーバー"),
                TextFont {
                    font: font.clone(),
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.3, 0.3)),
            ));
            parent.spawn((
                Text::new(format!("スコア: {}", game_data.score)),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("クリック でリトライ"),
                TextFont {
                    font,
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

pub fn gameover_input(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse.just_released(MouseButton::Left) || touches.any_just_released() {
        next_state.set(GameState::Playing);
    }
}
