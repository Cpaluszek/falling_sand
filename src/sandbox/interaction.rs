use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, RichText},
    EguiContexts, EguiPlugin,
};

use super::{
    particle::{get_particle, ParticleTypes},
    sandbox::Sandbox,
    CELL_SIZE,
};

pub const BRUSH_RADIUS: isize = 2;
pub const BRUSH_RADIUS_SQR: isize = BRUSH_RADIUS * BRUSH_RADIUS;

#[derive(Resource)]
pub struct SelectedParticle {
    particle_type: ParticleTypes,
}

pub struct InterationPlugin;

impl Plugin for InterationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(SelectedParticle {
                particle_type: ParticleTypes::Sand,
            })
            .add_systems(Update, (place_particles, select_particle_ui));
    }
}

// Note: should be a multiple of cell size
pub const PANEL_HEIGHT: f32 = 24.;
pub const SAND_COL: Color32 = Color32::from_rgb(249, 226, 175);
pub const WATER_COL: Color32 = Color32::from_rgb(137, 180, 250);
pub const STONE_COL: Color32 = Color32::from_rgb(127, 132, 156);
pub const STEAM_COL: Color32 = Color32::from_rgb(205, 214, 244);

pub fn select_particle_ui(mut contexts: EguiContexts, mut selected: ResMut<SelectedParticle>) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::bottom("bottom_panel")
        .exact_height(PANEL_HEIGHT)
        .show(ctx, |ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::from("Sand").color(Color32::BLACK))
                            .fill(SAND_COL),
                    )
                    .clicked()
                {
                    selected.particle_type = ParticleTypes::Sand;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Water").color(Color32::BLACK))
                            .fill(WATER_COL),
                    )
                    .clicked()
                {
                    selected.particle_type = ParticleTypes::Water;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Stone").color(Color32::BLACK))
                            .fill(STONE_COL),
                    )
                    .clicked()
                {
                    selected.particle_type = ParticleTypes::Stone;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Steam").color(Color32::BLACK))
                            .fill(STEAM_COL),
                    )
                    .clicked()
                {
                    selected.particle_type = ParticleTypes::Steam;
                }
            });
        });
}

pub fn place_particles(
    mut sandbox_query: Query<&mut Sandbox>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    selected: Res<SelectedParticle>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window: &Window = window_query.get_single().unwrap();
    let mut sandbox = sandbox_query.single_mut();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let y_treshold: f32 = -((CELL_SIZE * sandbox.height() as f32) / 2.) + PANEL_HEIGHT;
        if world_pos.y < y_treshold {
            return;
        }

        let cx = ((world_pos.x / CELL_SIZE) + (sandbox.width() / 2) as f32) as usize;
        let cy =
            (((world_pos.y - PANEL_HEIGHT) / CELL_SIZE) + (sandbox.height() / 2) as f32) as usize;
        if sandbox.out_of_bounds_usize(cx, cy) {
            return;
        }

        for x_offset in -BRUSH_RADIUS..=BRUSH_RADIUS {
            for y_offset in -BRUSH_RADIUS..=BRUSH_RADIUS {
                let x = cx.saturating_add_signed(x_offset);
                let y = cy.saturating_add_signed(y_offset);
                if sandbox.out_of_bounds_usize(x, y) {
                    return;
                }
                let dist_sqr = x_offset * x_offset + y_offset * y_offset;
                if dist_sqr <= BRUSH_RADIUS_SQR {
                    if sandbox.out_of_bounds_usize(cx, cy) {
                        return;
                    }

                    if mouse_button_input.pressed(MouseButton::Left)
                        && sandbox.checked_get(x, y).is_none()
                    {
                        sandbox.set(x, y, Some(get_particle(selected.particle_type)));
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
