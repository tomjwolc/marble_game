use super::*;

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalImpulse, &mut ExternalForce, &Velocity), With<Player>>,
    camera_transform_query: Query<&Transform, With<Camera>>,
    can_jump: Res<CanJump>
) {
    let Ok((
        mut player_impulse, 
        mut player_force, 
        player_velocity
    )) = player_query.get_single_mut() else { 
        println!("player_query: {:?}", player_query); 
        return
    };

    let Ok(camera_transform) = camera_transform_query.get_single() else { 
        panic!("Could not find camera transform in move_player");
    };

    let left = MARBLE_SPEED * camera_transform.left().normalize();
    let forward = MARBLE_SPEED * left.cross(Vec3::Y).normalize();

    player_force.torque = Vec3::ZERO;

    if 
        keys.pressed(KeyCode::W) && 
        player_velocity.angvel.dot(left.normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque += left;
    } else if 
        keys.pressed(KeyCode::S) && 
        player_velocity.angvel.dot((-left).normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque -= left;
    } 
    
    if 
        keys.pressed(KeyCode::D) && 
        player_velocity.angvel.dot(forward.normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque += forward;
    } else if 
        keys.pressed(KeyCode::A) && 
        player_velocity.angvel.dot((-forward).normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque -= forward;
    } 

    if keys.just_pressed(KeyCode::Space) && can_jump.0 {
        player_impulse.impulse.y = JUMP_IMPULSE
    }
}

pub fn move_sensor(
    player_transform_query: Query<&Transform, With<Player>>,
    mut sensor_transform_query: Query<&mut Transform, (With<PlayerSensor>, Without<Player>)>
) {
    let Ok(player_transform) = player_transform_query.get_single() else {
        println!("Could not find player in move_sensor");
        return;
    };

    let Ok(mut sensor_transform) = sensor_transform_query.get_single_mut() else {
        println!("Could not find player sensor in move_sensor");
        return
    };

    sensor_transform.translation = player_transform.translation - (MARBLE_RADIUS) * Vec3::Y;
}

pub fn update_can_jump(
    rapier_context: Res<RapierContext>,
    player_sensor_entity_query: Query<Entity, With<PlayerSensor>>,
    jumpy_entity_query: Query<Entity, (With<Jumpy>, Without<PlayerSensor>)>,
    mut can_jump: ResMut<CanJump>
) {
    let Ok(sensor_entity) = player_sensor_entity_query.get_single() else {
        return
    };

    can_jump.0 = false;
    
    for (collider1, collider2, is_intersecting) in rapier_context.intersections_with(sensor_entity) {
        if is_intersecting {
            for jumpy in jumpy_entity_query.iter() {
                can_jump.0 = can_jump.0 || jumpy == collider1 || jumpy == collider2;
            }
        }
    }
}