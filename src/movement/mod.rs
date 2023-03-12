pub use super::*;

mod look_around;
pub use look_around::*;

mod drive;
pub use drive::*;

#[derive(Resource)]
pub struct CanJump(bool);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CanJump(false))
            .add_system(lock_cursor.in_schedule(OnEnter(AppState::InGame)))
            .add_system(release_cursor.in_schedule(OnExit(AppState::InGame)))
            .add_systems((
                move_sensor,
                update_can_jump,
                move_player
            ).chain().distributive_run_if(AppState::in_game))
            .add_systems((
                rotate_camera.before(update_camera),
                update_camera,
            ).distributive_run_if(AppState::in_game))

        ;
    }
}