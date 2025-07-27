use bevy::{asset::AssetMetaCheck, prelude::*};
use rhytry::infrastructure::bevy::app::AppPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rhytry".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin { meta_check: AssetMetaCheck::Never, ..default() }),
        )
        .add_plugins(AppPlugin)
        .run();
}
