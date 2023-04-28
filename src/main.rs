// use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
// use bevy_framepace::FramepacePlugin;
pub use prelude::*;

#[macro_use]
pub mod macro_utils;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "The Amazing Marble game".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .add_state::<AppState>()
        .add_state::<MenuState>()

        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(DefaultMaterial(Handle::default()))
        .insert_resource(PkvStore::new("Twol Games", "The Amazing Marble Game"))

        .add_plugins(DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default().always_on_top())
        .add_plugin(MenuPlugin)
        .add_plugin(LoadUnloadPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(GravityPlugin)
        .add_plugin(SensorPlugin)
        .add_plugin(TimerPlugin)

        .add_startup_system(basic_setup)
        // .add_startup_system(debug_system)

        .add_system(clear_store)

        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(FramepacePlugin)

        .run();
}

// pub fn debug_system(
//     mut a: ResMut<PkvStore>
// ) {
//     println!("{:?}", a.get::<String>("a"));
//     println!("{:?}", a.clear());
// }

pub fn clear_store(
    keys: Res<Input<KeyCode>>,
    mut pkv_store: ResMut<PkvStore>
) {
    if keys.just_pressed(KeyCode::C) {
        let _ = pkv_store.clear();
    }

    if keys.just_pressed(KeyCode::L) {
        for key in [COMPLETED_LEVELS].into_iter() {
            println!("{:?}: {:?}", key, pkv_store.get::<CompletedLevels>(key))
        }
    }
}

fn basic_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut default_material: ResMut<DefaultMaterial>
) {
    commands.spawn((Camera3dBundle {
        transform: Transform
            ::from_xyz(0.0, 0.0, -4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, CameraDir::default()));

    // Default Material
    default_material.0 = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
}

mod load_unload;
mod menus;
mod movement;
mod materials;
mod ecs_data;
mod gravity;
mod sensors;
mod timer_plugin;
mod load_savedata;

mod prelude {
    pub use {crate::{
        load_unload::*, menus::*,
        movement::*, materials::*, 
        ecs_data::*, gravity::*,
        sensors::*, timer_plugin::*,
        load_savedata::*
    }, 
        bevy::prelude::*,
        bevy_rapier3d::prelude::*,
        std::collections::HashSet,
        bevy_pkv::*
    };
}