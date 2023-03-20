pub use super::*;

mod adjust_gravity;
pub use adjust_gravity::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                reorient_gravity,
                apply_gravitational_force
            ).chain().distributive_run_if(AppState::in_game))
        ;
    }
}