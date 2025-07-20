use bevy::prelude::*;

use crate::assets::LoadingPlugin;
use crate::onpu::OnpuPlugin;

mod assets;
mod onpu;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            OnpuPlugin,
        ));
    }
}
