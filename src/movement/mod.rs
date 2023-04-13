use colored::Colorize;

pub use super::*;
use std::collections::HashMap;

mod look_around;
mod drive;
mod key_queue;

pub use {
    look_around::*,
    drive::*,
    key_queue::*
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CanJump(false))
            .insert_resource(KeyQueue(HashMap::new()))
            .add_system(lock_cursor.in_schedule(OnEnter(AppState::InGame)))
            .add_system(release_cursor.in_schedule(OnExit(AppState::InGame)))
            .add_system(update_key_queue.run_if(AppState::in_game))
            .add_systems((
                move_sensor,
                move_player
            ).before(update_key_queue).chain().distributive_run_if(AppState::in_game))
            .add_systems((
                rotate_camera.before(update_camera),
                update_camera,
            ).distributive_run_if(AppState::in_game))
            // .add_system(log_moves.run_if(AppState::in_game))
        ;
    }
}

pub fn log_moves(
    player_transform_query: Query<
        (&Transform, Option<&SoftUnloadedData>, Option<&InGameEntity>, Option<&Player>), 
        (Or<(&SoftUnloadedData, &InGameEntity)>, Changed<Transform>)
    >
) {
    for (
        transform, 
        data_option, 
        is_in_game, 
        is_player
    ) in player_transform_query.iter() {
        let mut str = if let Some(data) = data_option {
            format!(     "unloaded_entity[{}]", data.id).green().italic()
        } else {
            String::from("     loaded_entity").green()
        };

        str = if is_player.is_some() { str.red() } else { str };

        println!("{} {} --- {}", if is_in_game.is_some() { "██" } else { "  " }, str, transform.translation);
    }
}