use super::*;

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalImpulse, &mut ExternalForce, &Velocity, &Gravity), With<Player>>,
    camera_transform_query: Query<&Transform, With<Camera>>,
    can_jump: Res<CanJump>,
    mut key_queue: ResMut<KeyQueue>
) {
    panic_extract!(move_player:
        Ok((
            mut player_impulse, 
            mut player_force, 
            player_velocity,
            &Gravity(mut gravity, _)
        )) = player_query.get_single_mut();
        Ok(camera_transform) = camera_transform_query.get_single()
    );

    gravity = if gravity.length() > 0.0 { gravity } else { -Vec3::Y };

    let left = MARBLE_SPEED * camera_transform.left().normalize();
    let forward = MARBLE_SPEED * left.cross(-gravity).normalize();

    player_force.torque = Vec3::ZERO;

    if 
        (keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up)) && 
        player_velocity.angvel.dot(left.normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque += left;
    } else if 
        (keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down)) && 
        player_velocity.angvel.dot((-left).normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque -= left;
    } 
    
    if 
        (keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right)) && 
        player_velocity.angvel.dot(forward.normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque += forward;
    } else if 
        (keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left)) && 
        player_velocity.angvel.dot((-forward).normalize()) < MAX_ANGLE_SPEED 
    {
        player_force.torque -= forward;
    }

    let is_not_falling = gravity.dot(player_velocity.linvel) <= 0.0;

    if can_jump.0 && is_not_falling && key_queue.0.remove(&KeyCode::Space).is_some() {
        player_impulse.impulse = JUMP_IMPULSE * -gravity.normalize();
    }
}

pub fn move_sensor(
    player_transform_query: Query<(&Transform, &Gravity), With<Player>>,
    mut sensor_transform_query: Query<&mut Transform, (With<PlayerSensor>, Without<Player>)>
) {
    log_extract!(move_sensor:
        Ok((player_transform, &Gravity(gravity, _))) = player_transform_query.get_single();
        Ok(mut sensor_transform) = sensor_transform_query.get_single_mut()
    );

    sensor_transform.translation = 
        player_transform.translation + JUMP_SENSOR_OFFSET * gravity.normalize();
}

pub fn update_can_jump(
    rapier_context: Res<RapierContext>,
    player_sensor_entity_query: Query<Entity, With<PlayerSensor>>,
    jumpy_entity_query: Query<Entity, (With<Jumpy>, Without<PlayerSensor>)>,
    mut can_jump: ResMut<CanJump>
) {
    ignore_extract!(Ok(sensor_entity) = player_sensor_entity_query.get_single());

    can_jump.0 = false;
    
    for (collider1, collider2, is_intersecting) in rapier_context.intersections_with(sensor_entity) {
        if is_intersecting {
            for jumpy in jumpy_entity_query.iter() {
                can_jump.0 = can_jump.0 || jumpy == collider1 || jumpy == collider2;
            }
        }
    }
}