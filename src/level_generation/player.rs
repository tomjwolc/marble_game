use super::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Player, 
        InGameEntity, 
        Collider::ball(MARBLE_RADIUS),
        RigidBody::Dynamic, 
        GravityScale(0.9),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        ActiveEvents::COLLISION_EVENTS
    )).insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: MARBLE_RADIUS,
            ..default()
        })),
        material: materials.add(StandardMaterial {
            base_color: MARBLE_COLOR,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn reset_camera(mut camera_transform_query: Query<&mut Transform, With<Camera>>) {
    *camera_transform_query.single_mut() = Transform::
        from_xyz(0.0, 0.0, -4.0)
        .looking_at(Vec3::ZERO, Vec3::Y);
}