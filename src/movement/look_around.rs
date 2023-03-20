use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};

use super::*; 

// sets camera_dir component
pub fn rotate_camera(
    mut camera_dir_query: Query<&mut CameraDir, With<Camera>>,
    player_transform_query: Query<&Gravity, (With<Player>, Without<Camera>)>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    panic_extract!(rotate_camera:
        Ok(mut camera_dir) = camera_dir_query.get_single_mut();
        Ok(&Gravity(mut gravity, _)) = player_transform_query.get_single()
    );

    let CameraDir { horizontal_direction, pitch } = &mut *camera_dir;
    gravity = if gravity.length() > 0.0 { gravity.normalize() } else { -Vec3::Y };

    *horizontal_direction = (
        *horizontal_direction - 
        horizontal_direction.project_onto(
            (-gravity).normalize()
        )).normalize();

    ignore_extract!(
        Some(&MouseMotion { delta }) = mouse_motion.iter().last()
    );

    *pitch += delta.y / SENSITIVITY;

    *pitch = if *pitch >  MAX_ANGLE {
        MAX_ANGLE
    }   else if *pitch < -MAX_ANGLE {
        -MAX_ANGLE
    } else {
        *pitch
    };
    
    *horizontal_direction = Quat::from_axis_angle(
        (-gravity).normalize(), 
        -delta.x / SENSITIVITY
    ).mul_vec3(*horizontal_direction);
}

// Uses camera_dir to place the camera
pub fn update_camera(
    mut camera_query: Query<(&mut Transform, &CameraDir), With<Camera>>,
    player_transform_query: Query<(&Transform, &Gravity), (With<Player>, Without<Camera>)>
) {
    panic_extract!(update_camera:
        Ok((mut camera_transform, camera_dir)) = camera_query.get_single_mut();
        Ok((player_transform, &Gravity(mut gravity, _))) = player_transform_query.get_single()
    );

    let CameraDir { horizontal_direction, pitch } = *camera_dir;

    gravity = if gravity.length() > 0.0 { gravity.normalize() } else { -Vec3::Y };

    // Axis that the rotates around to look up and down
    let vertical_rotation_axis = horizontal_direction.cross(-gravity).normalize();

    // Rotate the camera up or down according to pitch (radians)
    let direction = Quat::from_axis_angle(
        vertical_rotation_axis, 
        pitch
    ).mul_vec3(horizontal_direction);

    println!("{}\n{}\n", vertical_rotation_axis, direction);

    // Updates the position of the camera
    camera_transform.translation = 
        (CAMERA_ORBIT_RADIUS * direction) + player_transform.translation;

    // Rotate camera to face player
    *camera_transform = camera_transform
        .looking_at(player_transform.translation, -gravity);
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