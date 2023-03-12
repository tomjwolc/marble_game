pub use super::*;

mod platforms;
pub use platforms::*;

mod player;
pub use player::*;

pub struct LevelGenerationPlugin;

impl Plugin for LevelGenerationPlugin {
    fn build(&self, app: &mut App) {
        add_spawning_system!(app, spawn_platforms);
        add_spawning_system!(app, spawn_player);
        add_spawning_system!(app, reset_camera);
        add_despawning_system!(app, despawn_game_entities);

        app
            .add_systems((
                rotate_camera.before(update_camera),
                update_camera,
            ).distributive_run_if(AppState::in_game))
        ;
    }
}

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct InGameEntity;

fn despawn_game_entities(
    entities_query: Query<Entity, With<InGameEntity>>,
    mut commands: Commands
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}