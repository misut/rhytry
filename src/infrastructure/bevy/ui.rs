use bevy::{color::palettes::css::{RED, WHITE}, prelude::*};

const BACKGROUND_COLOR: Color = Color::linear_rgb(0.1, 0.1, 0.1);
const BACKGROUND_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const BACKGROUND_SIZE: Vec2 = Vec2::new(50000., 50000.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_background)
            .add_systems(PostUpdate, draw_cursor.after(TransformSystem::TransformPropagate));
    }
}

fn draw_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(BACKGROUND_SIZE.x, BACKGROUND_SIZE.y))),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::from_translation(BACKGROUND_POSITION),
    ));
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
