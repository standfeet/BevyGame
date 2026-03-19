use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;

/// ブロックの形状
#[derive(Clone, Copy)]
enum BlockShape {
    Rectangle,
    Triangle,
    Trapezoid,
    Circle,
}

/// ランダムな形状とサイズを生成
fn random_shape_and_size() -> (BlockShape, f32, f32) {
    let mut rng = rand::thread_rng();
    let width = rng.gen_range(BLOCK_MIN_SIZE..=BLOCK_MAX_SIZE);
    let height = rng.gen_range(BLOCK_MIN_SIZE..=BLOCK_MAX_SIZE);

    let shape = match rng.gen_range(0..4u8) {
        0 => BlockShape::Rectangle,
        1 => BlockShape::Triangle,
        2 => BlockShape::Trapezoid,
        _ => BlockShape::Circle,
    };

    match shape {
        BlockShape::Circle => (shape, width, width),
        _ => (shape, width, height),
    }
}

/// 台形の頂点座標を計算（コライダーとメッシュで共有）
fn trapezoid_vertices(width: f32, height: f32) -> [(f32, f32); 4] {
    let hw = width / 2.0;
    let thw = width * TRAPEZOID_TOP_RATIO / 2.0;
    let hh = height / 2.0;
    [(-hw, -hh), (hw, -hh), (thw, hh), (-thw, hh)]
}

/// 三角形の頂点座標を計算（コライダーとメッシュで共有）
fn triangle_vertices(width: f32, height: f32) -> [Vec2; 3] {
    [
        Vec2::new(-width / 2.0, -height / 2.0),
        Vec2::new(width / 2.0, -height / 2.0),
        Vec2::new(0.0, height / 2.0),
    ]
}

fn create_collider(shape: BlockShape, width: f32, height: f32) -> Collider {
    match shape {
        BlockShape::Rectangle => Collider::rectangle(width, height),
        BlockShape::Triangle => {
            let verts = triangle_vertices(width, height);
            Collider::convex_hull(verts.to_vec())
                .unwrap_or_else(|| Collider::rectangle(width, height))
        }
        BlockShape::Trapezoid => {
            let verts = trapezoid_vertices(width, height);
            let vec2s: Vec<Vec2> = verts.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
            Collider::convex_hull(vec2s)
                .unwrap_or_else(|| Collider::rectangle(width, height))
        }
        BlockShape::Circle => Collider::circle(width / 2.0),
    }
}

fn create_mesh(shape: BlockShape, width: f32, height: f32) -> Mesh {
    match shape {
        BlockShape::Rectangle => Rectangle::new(width, height).into(),
        BlockShape::Triangle => {
            let [a, b, c] = triangle_vertices(width, height);
            Triangle2d::new(a, b, c).into()
        }
        BlockShape::Trapezoid => {
            let verts = trapezoid_vertices(width, height);
            Mesh::new(
                bevy::mesh::PrimitiveTopology::TriangleList,
                bevy::asset::RenderAssetUsages::default(),
            )
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                verts.map(|(x, y)| [x, y, 0.0]).to_vec(),
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![[0.0, 1.0], [1.0, 1.0], [0.8, 0.0], [0.2, 0.0]],
            )
            .with_inserted_indices(bevy::mesh::Indices::U32(vec![0, 1, 2, 0, 2, 3]))
        }
        BlockShape::Circle => Circle::new(width / 2.0).into(),
    }
}

/// ランダムなカラーを選択
fn random_block_color() -> Color {
    let mut rng = rand::thread_rng();
    let [r, g, b] = BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS.len())];
    Color::srgb(r, g, b)
}

/// 新しいブロックをスポーンする
pub fn spawn_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
) {
    let (shape, width, height) = random_shape_and_size();

    commands.spawn((
        GameEntity,
        Block,
        ActiveBlock,
        Mesh2d(meshes.add(create_mesh(shape, width, height))),
        MeshMaterial2d(materials.add(random_block_color())),
        Transform::from_xyz(x, SPAWN_Y, 0.0),
        RigidBody::Kinematic,
        create_collider(shape, width, height),
        Friction::new(BLOCK_FRICTION),
        Restitution::new(0.0),
        SleepingDisabled,
    ));
}
