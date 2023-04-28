use std::time::Duration;

use super::*;

#[derive(Component)]
pub struct Timed {
    pub spawn_time: Duration,
    pub lifetime: Duration
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(kill_timed);
    }
}

pub fn kill_timed(
    time: Res<Time>,
    timed_entity_query: Query<(Entity, &Timed)>,
    mut commands: Commands
) {
    for (entity, Timed { spawn_time, lifetime }) in timed_entity_query.into_iter() {
        // println!("{:?} -- {:?} -- {:?}", entity, spawn_time, lifetime);
        if time.elapsed() >= *spawn_time + *lifetime {
            // println!("Timer removed");
            commands.entity(entity).remove::<Timed>();
        }
    }
}