use std::collections::VecDeque;

use bevy::{
    ecs::{
        component::{Mutable, StorageType},
        system::Query,
    },
    prelude::*,
    render::camera::Viewport,
};

use crate::{
    domain::taiko::{Beat, Fraction, Onpu, TaikoState},
    infrastructure::bevy::{app::AppState, assets::TextureAssets},
};

const FETCHED_ONPU: [&'static Onpu; 10] = [
    &Onpu::Don(Fraction { denominator: 8, numerator: 0 }),
    &Onpu::Kat(Fraction { denominator: 8, numerator: 2 }),
    &Onpu::Don(Fraction { denominator: 8, numerator: 4 }),
    &Onpu::Don(Fraction { denominator: 8, numerator: 5 }),
    &Onpu::Kat(Fraction { denominator: 8, numerator: 6 }),
    &Onpu::Kat(Fraction { denominator: 8, numerator: 8 }),
    &Onpu::Don(Fraction { denominator: 8, numerator: 10 }),
    &Onpu::Kat(Fraction { denominator: 8, numerator: 12 }),
    &Onpu::Kat(Fraction { denominator: 8, numerator: 13 }),
    &Onpu::Don(Fraction { denominator: 8, numerator: 14 }),
];

pub struct TaikoPlugin;

impl Component for Onpu {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    type Mutability = Mutable;
}

#[derive(Resource, Default)]
pub struct OnpuQueue(pub VecDeque<Entity>);

#[derive(Resource, Default)]
pub struct CurrentTaikoState(pub TaikoState);

#[derive(Resource)]
pub struct TaikoConfig {
    pub scroll_speed: f32,
    pub start_offset: f32,
}

impl Default for TaikoConfig {
    fn default() -> Self {
        Self {
            scroll_speed: 400.0, // Pixels per second
            start_offset: 100.0, // Initial offset for onpus
        }
    }
}

impl Plugin for TaikoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnpuQueue>()
            .init_resource::<CurrentTaikoState>()
            .init_resource::<TaikoConfig>()
            .add_systems(OnEnter(AppState::PlayingTaiko), (setup_taiko, spawn_onpu))
            .add_systems(Update, (hit_onpu, move_onpu, push_onpu_to_queue, update_taiko_state));
    }
}

fn update_taiko_state(time: Res<Time>, mut taiko_state: ResMut<CurrentTaikoState>) {
    const BPM: f32 = 100.0;
    const BEATS_PER_SHOUSETSU: f32 = 4.0; // Assuming 4/4 time signature

    let beat_duration = 60.0 / BPM;
    let shousetsu_duration = beat_duration * BEATS_PER_SHOUSETSU;

    let elapsed_time = time.elapsed_secs();

    let current_shousetsu_index = (elapsed_time / shousetsu_duration) as u32;
    let time_in_current_shousetsu = elapsed_time % shousetsu_duration;
    let current_beat_index = time_in_current_shousetsu / beat_duration;

    taiko_state.0 =
        TaikoState { shousetsu_index: current_shousetsu_index, beat_index: current_beat_index };
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

const BPS: f32 = 100.0 / 60.0; // Beats per second
const BEATS_PER_SHOUSETSU: f32 = 4.0; // Assuming 4/4 time signature

fn spawn_onpu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    taiko_config: Res<TaikoConfig>,
) {
    for &onpu in FETCHED_ONPU.iter() {
        match *onpu {
            Onpu::Don(fraction) => {
                commands.spawn((
                    Sprite::from_image(textures.don.clone()),
                    Transform::from_translation(Vec3::new(
                        taiko_config.start_offset
                            + fraction.at() * taiko_config.scroll_speed / BPS * BEATS_PER_SHOUSETSU,
                        0.,
                        0.,
                    )),
                    onpu.clone(),
                ));
            }
            Onpu::Kat(fraction) => {
                commands.spawn((
                    Sprite::from_image(textures.kat.clone()),
                    Transform::from_translation(Vec3::new(
                        taiko_config.start_offset
                            + fraction.at() * taiko_config.scroll_speed / BPS * BEATS_PER_SHOUSETSU,
                        0.,
                        0.,
                    )),
                    onpu.clone(),
                ));
            }
            _ => {}
        }
    }
}

fn move_onpu(
    time: Res<Time>,
    mut positions: Query<(&mut Onpu, &mut Transform)>,
    taiko_config: Res<TaikoConfig>,
) {
    for (mut _onpu, mut transform) in &mut positions {
        transform.translation.x -= taiko_config.scroll_speed * time.delta_secs();
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
