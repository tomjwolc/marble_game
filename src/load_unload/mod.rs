pub use super::*;

use bevy::gltf::Gltf;

mod player;
mod load_glb;
mod lighting;
mod unload;
mod reload;

pub use {
    player::*,
    load_glb::*,
    lighting::*,
    unload::*,
    reload::*
};

pub struct LoadUnloadPlugin;

impl Plugin for LoadUnloadPlugin {
    fn build(&self, app: &mut App) {
        /* Assumed that the gltf/glb asset will always be loaded 
         when the state is changed to InGame if LoadType is fresh */
        add_spawning_system!(app, load_glb if resource_equals(LoadType::Fresh));
        add_spawning_system!(app, spawn_lighting if resource_equals(LoadType::Fresh));
        add_spawning_system!(app, spawn_player if resource_equals(LoadType::Fresh));
        add_spawning_system!(app, reload if resource_equals(LoadType::Reload));
        add_spawning_system!(app, reset_camera);

        add_despawning_system!(app, despawn_all_game_entities if resource_equals(UnloadType::Complete));
        add_despawning_system!(app, despawn_loaded_game_entities if resource_equals(UnloadType::Hard));
        add_despawning_system!(app, soft_despawn_game_entities if resource_equals(UnloadType::Soft));

        app
            .add_system(load_glb_asset
                .run_if(resource_equals(LoadType::Fresh))
                .in_schedule(OnEnter(MenuState::Loading))
            )
            .add_system(try_load_glb_data
                .run_if(resource_equals(LoadType::Fresh))
                .in_set(OnUpdate(MenuState::Loading))
            )

            .add_system(immediate_exit_loading
                .run_if(resource_equals(LoadType::Reload))
                .in_schedule(OnEnter(MenuState::Loading))
            )

            .insert_resource(LevelStack::initial_stack())
            .insert_resource(LoadedGlbData(Vec::new()))
            .insert_resource(UnloadType::Hard)
            .insert_resource(LoadType::Fresh)
        ;
    }
}