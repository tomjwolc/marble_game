use super::*;

pub fn spawn_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn((
            Collider::cuboid(100.0, 0.1, 100.0), 
            TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)),
            InGameEntity
        ));

    commands
        .spawn((
            Collider::ball(1.0),
            RigidBody::Dynamic,
            GravityScale(0.5),
            TransformBundle::from(Transform::from_xyz(0.0001, 10.0, 0.0001)),
            Restitution { coefficient: 0.8, combine_rule: CoefficientCombineRule::Average },
            ActiveEvents::COLLISION_EVENTS,
            InGameEntity
        ));
}