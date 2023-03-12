pub use super::*;

mod look_around;
pub use look_around::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(lock_cursor.in_schedule(OnEnter(AppState::InGame)))
            .add_system(release_cursor.in_schedule(OnExit(AppState::InGame)))


        ;
    }
}