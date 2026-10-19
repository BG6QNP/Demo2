mod components;
mod interaction;
mod scene;
mod systems;

use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Mode>()
            .insert_resource(ClearColor(Color::srgb(0.005, 0.006, 0.012)))
            .insert_resource(GlobalAmbientLight {
                color: Color::srgb(0.45, 0.55, 1.0),
                brightness: 120.0,
                ..default()
            })
            .insert_resource(SpinSpeed(1.0))
            .add_systems(Startup, scene::setup)
            .add_systems(
                Update,
                (systems::spin, systems::orbit, systems::orbit_camera).run_if(in_state(Mode::Play)),
            )
            .add_systems(Update, (systems::controls, systems::hud, systems::gizmos));
    }
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum Mode {
    #[default]
    Play,
    Pause,
}

#[derive(Resource)]
pub struct SpinSpeed(pub f32);
