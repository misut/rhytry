use bevy::{
    app::{App, Plugin},
    state::{app::AppExtStates, state::States},
};

use crate::infrastructure::bevy::{assets::LoadingPlugin, onpu::OnpuPlugin};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((LoadingPlugin, OnpuPlugin));
    }
}
