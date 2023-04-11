use super::*;

pub fn spawn_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>
) {
    let mesh_handle = mesh_assets.add(shape::UVSphere { 
        radius: MARBLE_RADIUS ,
        sectors: NUM_SPHERE_SECTORS,
        stacks: NUM_SPHERE_STACKS
    }.into());
    
    commands.spawn((
        Player,
        MovableBundle::new(
            mesh_handle,
            default_material.0.clone(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Collider::ball(MARBLE_RADIUS), 
            SensorChannel::CanJump.not(),
            ColliderMassProperties::Mass(MARBLE_MASS), 
            *MATERIAL_PROPERTIES.get("default_marble").unwrap()
        ),
        ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO
        },
        Damping { linear_damping: 0.0, angular_damping: ANGULAR_DAMPING }
    ));

    commands.spawn((
        CanJumpSensor,
        SensorBundle::new(
            Collider::ball(MARBLE_RADIUS),
            Transform::default(),
            SensorChannel::CanJump
        )
    ));
}

pub fn reset_camera(mut camera_transform_query: Query<&mut Transform, With<Camera>>) {
    *camera_transform_query.single_mut() = Transform::
        from_xyz(0.0, 0.0, -4.0)
        .looking_at(Vec3::ZERO, Vec3::Y).with_scale(SCALE * Vec3::ONE)
}