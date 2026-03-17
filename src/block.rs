use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;

#[derive(Clone, Copy)]
pub enum BlockShape {
    Rectangle,
    Triangle,
    Trapezoid,
    Circle,
}

fn random_block_shape() -> (BlockShape, f32, f32) {
    let mut rng = rand::thread_rng();
    let shape_idx = rng.gen_range(0..4);
    let width = rng.gen_range(BLOCK_MIN_SIZE..=BLOCK_MAX_SIZE);
    let height = rng.gen_range(BLOCK_MIN_SIZE..=BLOCK_MAX_SIZE);

    match shape_idx {
        0 => (BlockShape::Rectangle, width, height),
        1 => (BlockShape::Triangle, width, height),
        2 => (BlockShape::Trapezoid, width, height),
        _ => (BlockShape::Circle, width, width),
    }
}

fn create_collider(shape: BlockShape, width: f32, height: f32) -> Collider {
    match shape {
        BlockShape::Rectangle => Collider::rectangle(width, height),
        BlockShape::Triangle => {
            let verts = vec![
                Vec2::new(-width / 2.0, -height / 2.0),
                Vec2::new(width / 2.0, -height / 2.0),
                Vec2::new(0.0, height / 2.0),
            ];
            Collider::convex_hull(verts).unwrap_or(Collider::rectangle(width, height))
        }
        BlockShape::Trapezoid => {
            let top_w = width * 0.6;
            let (hw, thw, hh) = (width / 2.0, top_w / 2.0, height / 2.0);
            let verts = vec![
                Vec2::new(-hw, -hh),
                Vec2::new(hw, -hh),
                Vec2::new(thw, hh),
                Vec2::new(-thw, hh),
            ];
            Collider::convex_hull(verts).unwrap_or(Collider::rectangle(width, height))
        }
        BlockShape::Circle => Collider::circle(width / 2.0),
    }
}

fn create_mesh(shape: BlockShape, width: f32, height: f32) -> Mesh {
    match shape {
        BlockShape::Rectangle => Rectangle::new(width, height).into(),
        BlockShape::Triangle => Triangle2d::new(
            Vec2::new(-width / 2.0, -height / 2.0),
            Vec2::new(width / 2.0, -height / 2.0),
            Vec2::new(0.0, height / 2.0),
        )
        .into(),
        BlockShape::Trapezoid => {
            let top_w = width * 0.6;
            let (hw, thw, hh) = (width / 2.0, top_w / 2.0, height / 2.0);
            Mesh::new(
                bevy::mesh::PrimitiveTopology::TriangleList,
                bevy::asset::RenderAssetUsages::default(),
            )
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![[-hw, -hh, 0.0], [hw, -hh, 0.0], [thw, hh, 0.0], [-thw, hh, 0.0]],
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

pub fn spawn_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    colors: &BlockColors,
    x: f32,
) {
    let mut rng = rand::thread_rng();
    let (shape, width, height) = random_block_shape();
    let color = colors.0[rng.gen_range(0..colors.0.len())];

    commands.spawn((
        GameEntity,
        Block,
        ActiveBlock,
        Mesh2d(meshes.add(create_mesh(shape, width, height))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, SPAWN_Y, 0.0),
        RigidBody::Kinematic,
        create_collider(shape, width, height),
        Friction::new(0.8),
        Restitution::new(0.0),
        SleepingDisabled,
    ));
}
