use super::*;

pub fn pause_physics(
    mut pausable_query: Query<(&mut RigidBody, &Velocity, &mut Pausable), (With<InGameEntity>, Without<Sensor>)>
) {
    println!("pause physics");

    for (mut rigid_body, velocity, mut pausable) in pausable_query.iter_mut() {
        *pausable = Pausable {
            velocity: *velocity,
            prev_rigid_body: Some(*rigid_body)
        };

        *rigid_body = RigidBody::Fixed;
    }
}

pub fn unpause_physics(
    mut pausable_query: Query<(&mut RigidBody, &mut Velocity, &Pausable), (With<InGameEntity>, Without<Sensor>)>
) {
    println!("unpause physics");

    for (mut rigid_body, mut velocity, pausable) in pausable_query.iter_mut() {
        if let Some(prev_rigid_body) = pausable.prev_rigid_body {
            *rigid_body = prev_rigid_body;
            *velocity = pausable.velocity;
        }
    }
}