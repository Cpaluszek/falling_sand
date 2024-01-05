use bevy::prelude::*;

use super::{
    particle::{get_particle, ParticleTypes},
    sandbox::Sandbox,
    CELL_SIZE,
};

pub const BRUSH_RADIUS: isize = 2;
pub const BRUSH_RADIUS_SQR: isize = BRUSH_RADIUS * BRUSH_RADIUS;

pub struct ParticlePlacerPlugin;

impl Plugin for ParticlePlacerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_particles);
    }
}

pub fn place_particles(
    mut sandbox_query: Query<&mut Sandbox>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window: &Window = window_query.get_single().unwrap();
    let mut sandbox = sandbox_query.single_mut();

    // Select particle type

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let cx = ((world_pos.x / CELL_SIZE) + (sandbox.width() / 2) as f32) as usize;
        let cy = ((world_pos.y / CELL_SIZE) + (sandbox.height() / 2) as f32) as usize;

        if sandbox.out_of_bounds_usize(cx, cy) {
            return;
        }

        for x_offset in -BRUSH_RADIUS..=BRUSH_RADIUS {
            for y_offset in -BRUSH_RADIUS..=BRUSH_RADIUS {
                let dist_sqr = x_offset * x_offset + y_offset * y_offset;
                if dist_sqr <= BRUSH_RADIUS_SQR {
                    let x = cx.saturating_add_signed(x_offset);
                    let y = cy.saturating_add_signed(y_offset);

                    if mouse_button_input.pressed(MouseButton::Left) && sandbox.get(x, y).is_none()
                    {
                        sandbox.set(x, y, Some(get_particle(ParticleTypes::Sand)));
                    } else if mouse_button_input.pressed(MouseButton::Right)
                        && sandbox.get(x, y).is_some()
                    {
                        sandbox.set(x, y, None);
                    }
                }
            }
        }
    }
}
