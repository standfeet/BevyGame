use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use web_sys::wasm_bindgen::JsCast;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy App".to_string(),
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
        .add_systems(Startup, (setup, hide_loading_screen))
        .add_systems(Update, update_viewport)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // サンプル: 画面中央に円を表示
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
    ));

    let font = asset_server.load("fonts/NotoSansJP-Regular.otf");

    // 日本語テキスト表示
    commands.spawn((
        Text::new("こんにちは、Bevy！"),
        TextFont {
            font,
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn update_viewport(windows: Query<&Window>, mut cameras: Query<&mut Camera, With<Camera2d>>) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    let window_width = window.physical_width();
    let window_height = window.physical_height();
    if window_width == 0 || window_height == 0 {
        return;
    }

    let window_aspect = window_width as f32 / window_height as f32;

    let (vp_width, vp_height) = if window_aspect > ASPECT_RATIO {
        let h = window_height;
        let w = (h as f32 * ASPECT_RATIO) as u32;
        (w, h)
    } else {
        let w = window_width;
        let h = (w as f32 / ASPECT_RATIO) as u32;
        (w, h)
    };

    let offset_x = (window_width - vp_width) / 2;
    let offset_y = (window_height - vp_height) / 2;

    camera.viewport = Some(bevy::camera::Viewport {
        physical_position: UVec2::new(offset_x, offset_y),
        physical_size: UVec2::new(vp_width, vp_height),
        ..default()
    });
}

fn hide_loading_screen() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    if let Some(el) = document.get_element_by_id("loading-screen") {
        let html_el: web_sys::HtmlElement = el.unchecked_into();
        let _ = html_el.class_list().add_1("fade-out");
    }
}
