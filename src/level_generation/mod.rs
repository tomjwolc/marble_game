pub use super::*;

use bevy::gltf::Gltf;

mod player;
mod load_glb;

pub use {
    player::*,
    load_glb::*
};

pub struct LevelGenerationPlugin;

impl Plugin for LevelGenerationPlugin {
    fn build(&self, app: &mut App) {
        //     Assumed that the gltf/glb asset will always be loaded 
        // when the state is changed to InGame
        add_spawning_system!(app, load_glb);
        add_spawning_system!(app, spawn_player);
        add_spawning_system!(app, reset_camera);
        add_despawning_system!(app, despawn_game_entities);

        app 
            .insert_resource(LevelStack::from_level("test_level"))
            .insert_resource(ActivationTable(vec![false; 10]))
        ;
    }
}

fn despawn_game_entities(
    entities_query: Query<Entity, With<InGameEntity>>,
    mut commands: Commands
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}