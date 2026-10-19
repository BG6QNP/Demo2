use bevy::{color::palettes::css::*, prelude::*};

pub fn recolor(
    click: On<PointerClick>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut index: Local<usize>,
) {
    let palette = [ORANGE, AQUA, LIME, MAGENTA, GOLD, PURPLE];
    if let Ok(mut material) = query.get_mut(click.entity) {
        material.0 = materials.add(StandardMaterial {
            base_color: palette[*index % palette.len()].into(),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        });
        *index += 1;
    }
}

pub fn hover_in(
    over: On<PointerOver>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(handle) = query.get(over.entity)
        && let Some(mut material) = materials.get_mut(handle)
    {
        material.emissive = LinearRgba::rgb(0.6, 0.6, 0.6);
    }
}

pub fn hover_out(
    out: On<PointerOut>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(handle) = query.get(out.entity)
        && let Some(mut material) = materials.get_mut(handle)
    {
        material.emissive = LinearRgba::BLACK;
    }
}
