use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Debug, Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default(), MainCamera));
}

fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}
