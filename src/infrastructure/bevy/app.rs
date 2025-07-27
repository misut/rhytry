use bevy::{
    app::{App, Plugin},
    state::{app::AppExtStates, state::States},
};

use crate::infrastructure::bevy::{assets::LoadingPlugin, taiko::TaikoPlugin, ui::UiPlugin};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    LoadingApp,
    PlayingTaiko,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((LoadingPlugin, TaikoPlugin, UiPlugin));
    }
}
