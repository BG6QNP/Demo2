use bevy::prelude::*;

#[derive(Component)]
pub struct Spin;

#[derive(Component)]
pub struct Orbit {
    pub radius: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct CameraRig;

#[derive(Component)]
pub struct Hud;
