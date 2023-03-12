use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};

use super::*; 

pub fn rotate_camera(
    mut camera_dir_query: Query<&mut CameraDir, With<Camera>>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok(mut camera_dir) = camera_dir_query.get_single_mut() else { 
        panic!("move_camera() could not find camera") 
    };

    let Some(MouseMotion { delta }) = mouse_motion.iter().last() else { return };

    let forward = -camera_dir.0;
    let vertical_rotation_axis = Vec3::Y.cross(forward).normalize();
    let mut rotation = Quat::from_rotation_x(0.0);

    if  // rotates the quat vertically if our vertical rotation is within acceptable parameters
        (delta.y < 0.0 && forward.angle_between(Vec3::Y) > (PI / 2.0) - MAX_ANGLE) ||
        (delta.y > 0.0 && forward.angle_between(Vec3::Y) < (PI / 2.0) + MAX_ANGLE)
    {
        rotation = rotation.mul_quat(Quat::from_axis_angle(vertical_rotation_axis, delta.y / SENSITIVITY));
    }
    
    rotation = rotation.mul_quat(Quat::from_axis_angle(Vec3::Y, -delta.x / SENSITIVITY));

    camera_dir.0 = rotation.mul_vec3(camera_dir.0);
}

pub fn update_camera(
    mut camera_query: Query<(&mut Transform, &CameraDir), With<Camera>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let Ok((mut camera_transform, camera_dir)) = camera_query.get_single_mut() else { 
        panic!("move_camera() could not find camera") 
    };

    let Ok(player_transform) = player_transform_query.get_single() else { 
        panic!("move_camera() could not find player") 
    };

    // Updates the position of the camera
    camera_transform.translation = 
        (CAMERA_ORBIT_RADIUS * camera_dir.0) + player_transform.translation;

    // Rotate camera to face player
    *camera_transform = camera_transform
        .looking_at(player_transform.translation, Vec3::Y);
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