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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(DefaultMaterial(Handle::default()))
        .add_plugins(DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(MenuPlugin)
        .add_plugin(LevelGenerationPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(GravityPlugin)
        .add_plugin(SensorPlugin)
        .add_startup_system(basic_setup)
        .run();
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

mod level_generation;
mod menus;
mod movement;
mod win_lose_reward;
mod materials;
mod ecs_data;
mod gravity;
mod sensors;

mod prelude {
    pub use {crate::{
        level_generation::*, menus::*,
        movement::*, win_lose_reward::*,
        materials::*, ecs_data::*, gravity::*,
        sensors::*
    }, 
        bevy::prelude::*,
        bevy_rapier3d::prelude::*
    };
}