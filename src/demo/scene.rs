use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::css::*, core_pipeline::tonemapping::Tonemapping,
    pbr::ScreenSpaceAmbientOcclusion, post_process::bloom::Bloom, prelude::*,
};

use super::{
    components::{CameraRig, Hud, Orbit, Spin},
    interaction,
};

const TAU: f32 = std::f32::consts::TAU;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        CameraRig,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        Transform::from_xyz(9.0, 5.0, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, 0.5, 0.0)),
    ));

    commands.spawn((
        PointLight {
            intensity: 4_000_000.0,
            color: ORANGE.into(),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 4.5, 0.0),
    ));

    commands.spawn((
        SpotLight {
            intensity: 8_000_000.0,
            color: AQUA.into(),
            shadow_maps_enabled: true,
            inner_angle: 0.25,
            outer_angle: 0.5,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 30.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.06, 0.09),
            metallic: 0.6,
            perceptual_roughness: 0.25,
            ..default()
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Torus::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: LinearRgba::rgb(12.0, 0.8, 30.0),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.8, 0.0)
            .with_scale(Vec3::splat(0.9))
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        Spin,
    ));

    let cube = meshes.add(Cuboid::new(0.7, 0.7, 0.7));
    let sphere = meshes.add(Sphere::new(0.45).mesh().ico(3).unwrap());
    let palette = [ORANGE, AQUA, LIME, MAGENTA, GOLD, PURPLE];

    for (i, color) in palette.into_iter().enumerate() {
        let material = materials.add(StandardMaterial {
            base_color: color.into(),
            metallic: 0.9,
            perceptual_roughness: 0.15,
            ..default()
        });
        commands
            .spawn((
                Mesh3d(if i % 2 == 0 {
                    cube.clone()
                } else {
                    sphere.clone()
                }),
                MeshMaterial3d(material),
                Transform::from_xyz(0.0, 1.0, 0.0),
                Orbit {
                    radius: 3.2,
                    speed: 0.5 + i as f32 * 0.11,
                    phase: i as f32 * TAU / 6.0,
                },
            ))
            .observe(interaction::recolor)
            .observe(interaction::hover_in)
            .observe(interaction::hover_out);
    }

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        TextFont::from_font_size(18.0),
        TextColor(Color::WHITE),
        Hud,
    ));
}
