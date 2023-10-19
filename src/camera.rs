use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers,
    window::PrimaryWindow,
};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::AppState;

#[derive(Debug, Component)]
pub struct CardCamera;
#[derive(Debug, Component)]
pub struct BoardCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup);
        app.add_plugins(PanCamPlugin::default());
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // no "background color", we need to see the main camera's output
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        CardCamera,
        RenderLayers::layer(0),
    ));
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        BoardCamera,
        RenderLayers::layer(1),
    ))
    .insert(PanCam::default());
}

pub fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}
