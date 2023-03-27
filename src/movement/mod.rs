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

#[derive(Resource)]
pub struct CanJump(bool);

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
                update_can_jump,
                move_player
            ).before(update_key_queue).chain().distributive_run_if(AppState::in_game))
            .add_systems((
                rotate_camera.before(update_camera),
                update_camera,
            ).distributive_run_if(AppState::in_game))

        ;
    }
}