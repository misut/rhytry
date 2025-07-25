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
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add background to visualize viewport bounds
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50000.0, 50000.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.01))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -200.0)),
    ));
}
