use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

mod sandbox;
mod utils;
use sandbox::SandboxPlugin;

pub const RESOLUTION: (f32, f32) = (1280.0, 720.0);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Falling Sand".into(),
                        resolution: RESOLUTION.into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(SandboxPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: RESOLUTION.0,
                height: RESOLUTION.1,
            },
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}
