use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};

use super::*; 

pub fn rotate_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok(mut camera_transform) = camera_transform_query.get_single_mut() else { 
        panic!("move_camera() could not find camera") 
    };
    let Ok(player_transform) = player_transform_query.get_single() else { 
        panic!("move_camera() could not find player") 
    };

    let Some(MouseMotion { delta }) = mouse_motion.iter().last() else { return };

    let forward = camera_transform.forward();
    let vertical_rotation_axis = Vec3::Y.cross(forward).normalize();
    let mut rotation = Quat::from_rotation_x(0.0);

    if 
        (delta.y < 0.0 && forward.angle_between(Vec3::Y) > (PI / 2.0) - MAX_ANGLE) ||
        (delta.y > 0.0 && forward.angle_between(Vec3::Y) < (PI / 2.0) + MAX_ANGLE)
    {
        rotation = rotation.mul_quat(Quat::from_axis_angle(vertical_rotation_axis, delta.y / SENSITIVITY));
    }
    
    rotation = rotation.mul_quat(Quat::from_axis_angle(Vec3::Y, -delta.x / SENSITIVITY));

    camera_transform.rotate_around(player_transform.translation, rotation);
}

pub fn update_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let Ok(mut camera_transform) = camera_transform_query.get_single_mut() else { 
        panic!("move_camera() could not find camera") 
    };
    let Ok(player_transform) = player_transform_query.get_single() else { 
        panic!("move_camera() could not find player") 
    };

    *camera_transform = camera_transform
        .looking_at(player_transform.translation, Vec3::Y);

    camera_transform.translation = 
        CAMERA_ORBIT_RADIUS * 
        (camera_transform.translation - player_transform.translation).normalize() + 
        player_transform.translation;

    // camera_transform.translation = player_transform.translation - CAMERA_ORBIT_RADIUS * camera_transform.forward();

    // let forward = camera_transform.forward();
    // let mut right_projected = camera_transform.right();
    // right_projected.y = 0.0;

    // let vertical_rotation_axis = Vec3::Y.cross(forward).normalize();
    // // Corrects drift in the up direction
    // let mut rotation = Quat::from_rotation_arc(camera_transform.right(), right_projected);

    // if 
    //     (delta.y < 0.0 && forward.angle_between(Vec3::Y) > (PI / 2.0) - MAX_ANGLE) ||
    //     (delta.y > 0.0 && forward.angle_between(Vec3::Y) < (PI / 2.0) + MAX_ANGLE)
    // {
    //     rotation = rotation.mul_quat(Quat::from_axis_angle(vertical_rotation_axis, delta.y / SENSITIVITY));
    // }
    
    // rotation = rotation.mul_quat(Quat::from_axis_angle(Vec3::Y, -delta.x / SENSITIVITY));

    // camera_transform.rotate_local(rotation);

    // camera_transform.translation = player_transform.translation - CAMERA_ORBIT_RADIUS * camera_transform.forward();
}

pub fn lock_cursor(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn release_cursor(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}