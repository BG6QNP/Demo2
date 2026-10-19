use bevy::{color::palettes::css::PURPLE, prelude::*};

use super::{
    Mode, SpinSpeed,
    components::{CameraRig, Hud, Orbit, Spin},
};

pub fn spin(time: Res<Time>, speed: Res<SpinSpeed>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * speed.0);
        transform.rotate_x(time.delta_secs() * 0.35);
    }
}

pub fn orbit(
    time: Res<Time>,
    speed: Res<SpinSpeed>,
    mut query: Query<(&mut Transform, &Orbit), Without<Spin>>,
) {
    let t = time.elapsed_secs() * speed.0;
    for (mut transform, orbit) in &mut query {
        let angle = t * orbit.speed + orbit.phase;
        transform.translation = Vec3::new(
            angle.cos() * orbit.radius,
            1.3 + (angle * 2.0).sin() * 0.45,
            angle.sin() * orbit.radius,
        );
        transform.rotation = Quat::from_euler(EulerRot::YXZ, angle, angle * 0.7, 0.0);
    }
}

pub fn orbit_camera(time: Res<Time>, mut query: Query<&mut Transform, With<CameraRig>>) {
    let angle = time.elapsed_secs() * 0.12;
    if let Ok(mut transform) = query.single_mut() {
        transform.translation = Vec3::new(angle.cos() * 9.5, 5.0, angle.sin() * 9.5);
        transform.look_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y);
    }
}

pub fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<Mode>>,
    mut next: ResMut<NextState<Mode>>,
    mut speed: ResMut<SpinSpeed>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(match state.get() {
            Mode::Play => Mode::Pause,
            Mode::Pause => Mode::Play,
        });
    }
    if keys.pressed(KeyCode::ArrowUp) {
        speed.0 = (speed.0 + 0.02).min(4.0);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        speed.0 = (speed.0 - 0.02).max(0.0);
    }
}

pub fn hud(
    time: Res<Time>,
    mode: Res<State<Mode>>,
    speed: Res<SpinSpeed>,
    mut query: Query<&mut Text, With<Hud>>,
) {
    let fps = 1.0 / time.delta_secs().max(f32::EPSILON);
    for mut text in &mut query {
        text.0 = format!(
            "Bevy 0.20 | Vulkan | {:?} | speed {:.1}x | {:.0} FPS\nSPACE pause | UP/DOWN speed | click objects to recolor",
            mode.get(),
            speed.0,
            fps
        );
    }
}

pub fn gizmos(mut gizmos: Gizmos) {
    gizmos.axes(Transform::from_xyz(0.0, 0.02, 0.0), 1.5);
    gizmos.circle(
        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        4.2,
        PURPLE,
    );
}
