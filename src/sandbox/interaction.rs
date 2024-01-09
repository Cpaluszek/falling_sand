use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, RichText},
    EguiContexts, EguiPlugin,
};

use super::{
    particle::{
        get_particle, Material, ACID_COLOR, ALCOHOL_COLOR, LAVA_COLOR, SAND_COLOR, SMOKE_COLOR,
        SPARK_COLORS, STEAM_COLOR, STONE_COLOR, WATER_COLOR, WOOD_COLOR,
    },
    sandbox::Sandbox,
    CELL_SIZE,
};

pub const BRUSH_RADIUS: isize = 4;
pub const BRUSH_RADIUS_SQR: isize = BRUSH_RADIUS * BRUSH_RADIUS;

#[derive(Resource)]
pub struct SelectedParticle {
    material: Material,
}

pub struct InterationPlugin;

impl Plugin for InterationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(SelectedParticle {
                material: Material::Sand,
            })
            .insert_resource(ButtonsColors {
                sand: Color32::from_rgb(
                    (SAND_COLOR.r() * 255.0) as u8,
                    (SAND_COLOR.g() * 255.0) as u8,
                    (SAND_COLOR.b() * 255.0) as u8,
                ),
                water: Color32::from_rgb(
                    (WATER_COLOR.r() * 255.0) as u8,
                    (WATER_COLOR.g() * 255.0) as u8,
                    (WATER_COLOR.b() * 255.0) as u8,
                ),
                stone: Color32::from_rgb(
                    (STONE_COLOR.r() * 255.0) as u8,
                    (STONE_COLOR.g() * 255.0) as u8,
                    (STONE_COLOR.b() * 255.0) as u8,
                ),
                steam: Color32::from_rgb(
                    (STEAM_COLOR.r() * 255.0) as u8,
                    (STEAM_COLOR.g() * 255.0) as u8,
                    (STEAM_COLOR.b() * 255.0) as u8,
                ),
                wood: Color32::from_rgb(
                    (WOOD_COLOR.r() * 255.0) as u8,
                    (WOOD_COLOR.g() * 255.0) as u8,
                    (WOOD_COLOR.b() * 255.0) as u8,
                ),
                acid: Color32::from_rgb(
                    (ACID_COLOR.r() * 255.0) as u8,
                    (ACID_COLOR.g() * 255.0) as u8,
                    (ACID_COLOR.b() * 255.0) as u8,
                ),
                lava: Color32::from_rgb(
                    (LAVA_COLOR.r() * 255.0) as u8,
                    (LAVA_COLOR.g() * 255.0) as u8,
                    (LAVA_COLOR.b() * 255.0) as u8,
                ),
                smoke: Color32::from_rgb(
                    (SMOKE_COLOR.r() * 255.0) as u8,
                    (SMOKE_COLOR.g() * 255.0) as u8,
                    (SMOKE_COLOR.b() * 255.0) as u8,
                ),
                spark: Color32::from_rgb(
                    (SPARK_COLORS[0].r() * 255.0) as u8,
                    (SPARK_COLORS[0].g() * 255.0) as u8,
                    (SPARK_COLORS[0].b() * 255.0) as u8,
                ),
                alcohol: Color32::from_rgb(
                    (ALCOHOL_COLOR.r() * 255.0) as u8,
                    (ALCOHOL_COLOR.g() * 255.0) as u8,
                    (ALCOHOL_COLOR.b() * 255.0) as u8,
                ),
            })
            .add_systems(Update, (place_particles, select_particle_ui));
    }
}

#[derive(Resource)]
pub struct ButtonsColors {
    sand: Color32,
    water: Color32,
    stone: Color32,
    steam: Color32,
    wood: Color32,
    acid: Color32,
    lava: Color32,
    smoke: Color32,
    spark: Color32,
    alcohol: Color32,
}

// should be a multiple of cell size
pub const PANEL_HEIGHT: f32 = 24.0;

pub fn select_particle_ui(
    mut contexts: EguiContexts,
    mut selected: ResMut<SelectedParticle>,
    colors: Res<ButtonsColors>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::bottom("bottom_panel")
        .exact_height(PANEL_HEIGHT)
        .show(ctx, |ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::from("Sand").color(Color32::BLACK))
                            .fill(colors.sand),
                    )
                    .clicked()
                {
                    selected.material = Material::Sand;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Water").color(Color32::BLACK))
                            .fill(colors.water),
                    )
                    .clicked()
                {
                    selected.material = Material::Water;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Stone").color(Color32::BLACK))
                            .fill(colors.stone),
                    )
                    .clicked()
                {
                    selected.material = Material::Stone;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Steam").color(Color32::BLACK))
                            .fill(colors.steam),
                    )
                    .clicked()
                {
                    selected.material = Material::Steam;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Wood").color(Color32::BLACK))
                            .fill(colors.wood),
                    )
                    .clicked()
                {
                    selected.material = Material::Wood;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Acid").color(Color32::BLACK))
                            .fill(colors.acid),
                    )
                    .clicked()
                {
                    selected.material = Material::Acid;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Lava").color(Color32::BLACK))
                            .fill(colors.lava),
                    )
                    .clicked()
                {
                    selected.material = Material::Lava;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Smoke").color(Color32::BLACK))
                            .fill(colors.smoke),
                    )
                    .clicked()
                {
                    selected.material = Material::Smoke;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Spark").color(Color32::BLACK))
                            .fill(colors.spark),
                    )
                    .clicked()
                {
                    selected.material = Material::Spark;
                }
                if ui
                    .add(
                        egui::Button::new(RichText::from("Alcohol").color(Color32::BLACK))
                            .fill(colors.alcohol),
                    )
                    .clicked()
                {
                    selected.material = Material::Alcohol;
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
                        sandbox.set(x, y, Some(get_particle(selected.material)));
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
