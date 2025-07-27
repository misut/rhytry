use std::collections::VecDeque;

use bevy::{ecs::system::Query, prelude::*, render::camera::Viewport};

use crate::{infrastructure::bevy::app::AppState, infrastructure::bevy::assets::TextureAssets};

pub struct TaikoPlugin;

#[derive(Component)]
enum Onpu {
    Don,
    Kat,
}

#[derive(Resource, Default)]
pub struct OnpuQueue(pub VecDeque<Entity>);

impl Plugin for TaikoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnpuQueue>()
            .add_systems(OnEnter(AppState::PlayingTaiko), (setup_taiko, spawn_onpu))
            .add_systems(Update, (hit_onpu, move_onpu, push_onpu_to_queue));
    }
}

fn setup_taiko(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    // Initialize centered, non-window-filling viewport
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: window_size.as_uvec2(),
                ..default()
            }),
            ..default()
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50000.0, window_size.y * 0.3))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.1))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn push_onpu_to_queue(query: Query<Entity, Added<Onpu>>, mut queue: ResMut<OnpuQueue>) {
    for onpu in query.iter() {
        queue.0.push_back(onpu);
    }
}

fn spawn_onpu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.don.clone()),
        Transform::from_translation(Vec3::new(600., 0., 0.)),
        Onpu::Don,
    ));

    commands.spawn((
        Sprite::from_image(textures.kat.clone()),
        Transform::from_translation(Vec3::new(900., 0., 0.)),
        Onpu::Kat,
    ));

    commands.spawn((
        Sprite::from_image(textures.don.clone()),
        Transform::from_translation(Vec3::new(1200., 0., 0.)),
        Onpu::Don,
    ));
}

fn move_onpu(time: Res<Time>, mut positions: Query<(&mut Onpu, &mut Transform)>) {
    for (mut _onpu, mut transform) in &mut positions {
        transform.translation.x -= 200.0 * time.delta_secs();
    }
}

fn hit_onpu(
    mut commands: Commands,
    mut onpu_queue: ResMut<OnpuQueue>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(onpu) = onpu_queue.0.pop_front() {
            commands.entity(onpu).despawn();
        } else {
            println!("No more onpus to hit!");
        }
    }
}
