use bevy::{
    asset::AssetMetaCheck, color::palettes::css::GREEN, math::ops::powf, prelude::*,
    render::camera::Viewport,
};
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
        .add_systems(FixedUpdate, controls)
        .run();
}

fn controls(
    mut camera_query: Query<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Query<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    let Ok((mut camera, mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };
    let fspeed = 600.0 * time.delta_secs();
    let uspeed = fspeed as u32;
    let window_size = window.resolution.physical_size();

    // Camera movement controls
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += fspeed;
    }

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }

    if let Some(viewport) = camera.viewport.as_mut() {
        // Viewport movement controls
        if input.pressed(KeyCode::KeyW) {
            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyS) {
            viewport.physical_position.y += uspeed;
        }
        if input.pressed(KeyCode::KeyA) {
            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyD) {
            viewport.physical_position.x += uspeed;
        }

        // Bound viewport position so it doesn't go off-screen
        viewport.physical_position =
            viewport.physical_position.min(window_size - viewport.physical_size);

        // Viewport size controls
        if input.pressed(KeyCode::KeyI) {
            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyK) {
            viewport.physical_size.y += uspeed;
        }
        if input.pressed(KeyCode::KeyJ) {
            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyL) {
            viewport.physical_size.x += uspeed;
        }

        // Bound viewport size so it doesn't go off-screen
        viewport.physical_size = viewport
            .physical_size
            .min(window_size - viewport.physical_position)
            .max(UVec2::new(20, 20));
    }
}

fn setup(
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
                physical_position: (window_size * 0.125).as_uvec2(),
                physical_size: (window_size * 0.75).as_uvec2(),
                ..default()
            }),
            ..default()
        },
    ));

    // Create a minimal UI explaining how to interact with the example
    commands.spawn((
        Text::new(
            "Move the mouse to see the circle follow your cursor.\n\
                    Use the arrow keys to move the camera.\n\
                    Use the comma and period keys to zoom in and out.\n\
                    Use the WASD keys to move the viewport.\n\
                    Use the IJKL keys to resize the viewport.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    // Add mesh to make camera movement visible
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
    ));

    // Add background to visualize viewport bounds
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50000.0, 50000.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.01))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -200.0)),
    ));
}
