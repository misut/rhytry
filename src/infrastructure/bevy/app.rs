use bevy::{
    app::{App, Plugin},
    color::palettes::css::{RED, WHITE},
    prelude::*,
    state::{app::AppExtStates, state::States},
};

use crate::infrastructure::bevy::{assets::LoadingPlugin, onpu::OnpuPlugin};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(LoadingPlugin)
            .add_plugins(OnpuPlugin)
            .add_systems(PostUpdate, draw_cursor.after(TransformSystem::TransformPropagate));
    }
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;
    let Ok(window) = window.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // To test Camera::world_to_viewport, convert result back to viewport space and then back to world space.
    let Ok(viewport_check) = camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
    else {
        return;
    };
    let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy()) else {
        return;
    };

    gizmos.circle_2d(world_pos, 10., WHITE);
    // Should be the same as world_pos
    gizmos.circle_2d(world_check, 8., RED);
}
