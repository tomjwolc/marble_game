pub use prelude::*;

#[macro_use]
pub mod macro_util;

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
        .add_plugins(DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(MenuPlugin)
        .add_plugin(LevelGenerationPlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(basic_setup)
        .run();
}

fn basic_setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::
            from_xyz(0.0, 0.0, -4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, CameraDir(-Vec3::Z)));
}

mod data;
mod level_generation;
mod menus;
mod movement;
mod win_lose_reward;
mod states;
mod materials;
mod bundles;
mod components;

mod prelude {
    pub use {crate::{
        data::*, level_generation::*, menus::*,
        movement::*, win_lose_reward::*, states::*, 
        materials::*, bundles::*, components::*
    }, 
        bevy::prelude::*,
        bevy_rapier3d::prelude::*
    };
}