use bevy::prelude::*;
use bevy::render::{render_resource::*, texture::ImageSampler};
use bevy::time::common_conditions::on_timer;
use std::f32::consts::PI;
use std::time::Duration;

use crate::RESOLUTION;

use self::interaction::{InterationPlugin, PANEL_HEIGHT};
use self::render::render_particles;
use self::sandbox::Sandbox;
use self::simulation::update_particles;

mod interaction;
mod movement;
pub mod particle;
mod render;
mod sandbox;
mod simulation;
mod temperature;

pub const CELL_SIZE: f32 = 4.0;
pub const SANDBOX_SIZE: (f32, f32) = (
    RESOLUTION.0 / CELL_SIZE,
    (RESOLUTION.1 - PANEL_HEIGHT) / CELL_SIZE,
);

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        info!("Sandbox size {0} {1}", SANDBOX_SIZE.0, SANDBOX_SIZE.1);
        app.add_plugins(InterationPlugin)
            .add_systems(Startup, spawn_sandbox)
            .add_systems(
                Update,
                (update_particles, render_particles)
                    .chain()
                    .distributive_run_if(on_timer(Duration::from_secs_f32(1. / 60.))),
            );
    }
}

pub fn spawn_sandbox(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image_handle = {
        let mut image = Image::new_fill(
            Extent3d {
                width: SANDBOX_SIZE.0 as u32,
                height: SANDBOX_SIZE.1 as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
        );
        image.sampler = ImageSampler::nearest();
        images.add(image)
    };

    commands
        .spawn(Sandbox::new(
            SANDBOX_SIZE.0 as usize,
            SANDBOX_SIZE.1 as usize,
        ))
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, PANEL_HEIGHT * 0.5, 1.0),
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 1.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, PI, PI),
            },
            ..Default::default()
        });
}
