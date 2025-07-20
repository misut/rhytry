use crate::assets::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct OnpuPlugin;

#[derive(Component)]
pub struct Onpu;

impl Plugin for OnpuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_onpu);
    }
}

fn spawn_onpu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.don.clone()),
        Transform::from_translation(Vec3::new(0., 0., 1.0)),
        Onpu,
    ));
}
