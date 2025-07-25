use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::infrastructure::bevy::game::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "texture/onpu_don.png")]
    pub don: Handle<Image>,
    #[asset(path = "texture/onpu_kat.png")]
    pub kat: Handle<Image>,
    #[asset(path = "texture/ooki_don.png")]
    pub ooki_don: Handle<Image>,
    #[asset(path = "texture/ooki_kat.png")]
    pub ooki_kat: Handle<Image>,
}
