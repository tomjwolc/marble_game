use super::*;

#[derive(Bundle)]
pub struct MarbleBundle {
    mass: ColliderMassProperties,
    friction: Friction,
    restitution: Restitution,

    in_game_entity: InGameEntity, 
    collider: Collider,
    rigid_body: RigidBody, 
    velocity: Velocity,
    gravity_scale: GravityScale,
    active_events: ActiveEvents,
    pausable: Pausable,
    pbr_bundle: PbrBundle
}

impl MarbleBundle {
    pub fn new(
        radius: f32, 
        mass: f32, 
        friction: f32, 
        restitution: f32, 
        transform: Transform,
        meshes: &mut Assets<Mesh>,
        material: Handle<StandardMaterial>,
        velocity: Velocity
    ) -> Self {
        MarbleBundle { 
            friction: Friction::coefficient(friction),
            restitution: Restitution { coefficient: restitution, combine_rule: CoefficientCombineRule::Max },
            mass: ColliderMassProperties::Mass(mass),

            in_game_entity: InGameEntity, 
            collider: Collider::ball(radius),
            rigid_body: RigidBody::Dynamic, 
            velocity,
            gravity_scale: GravityScale(MARBLE_GRAVITY),
            active_events: ActiveEvents::COLLISION_EVENTS,
            pausable: Pausable::default(),
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius,
                    ..default()
                })),
                material,
                transform,
                ..default()
            }
        }
    }
}

impl Default for MarbleBundle {
    fn default() -> Self {
        MarbleBundle { 
            friction: Friction::coefficient(0.0),
            restitution: Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Max },
            mass: ColliderMassProperties::Mass(3.0 * MARBLE_MASS),

            in_game_entity: InGameEntity, 
            collider: Collider::ball(2.0 * MARBLE_RADIUS),
            rigid_body: RigidBody::Dynamic, 
            velocity: Velocity::zero(),
            gravity_scale: GravityScale(MARBLE_GRAVITY),
            active_events: ActiveEvents::COLLISION_EVENTS,
            pausable: Pausable::default(),
            pbr_bundle: PbrBundle::default()
        }
    }
}