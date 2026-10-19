mod demo;

use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, WgpuSettings},
    },
};

use demo::DemoPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy 0.20 | Vulkan | Demo2".into(),
                        resolution: (1280u32, 720u32).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        // Strictly use Vulkan. There is intentionally no fallback.
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins((MeshPickingPlugin, DemoPlugin))
        .run();
}
