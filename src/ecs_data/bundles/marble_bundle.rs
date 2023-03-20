use super::*;

#[derive(Bundle)]
pub struct MarbleBundle {
    friction: Friction,
    restitution: Restitution,

    in_game_entity: InGameEntity, 
    collider: Collider,
    rigid_body: RigidBody, 
    velocity: Velocity,
    gravity: GravityBundle,
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

            in_game_entity: InGameEntity, 
            collider: Collider::ball(radius),
            rigid_body: RigidBody::Dynamic, 
            velocity,
            gravity: GravityBundle::from_mass(mass),
            active_events: ActiveEvents::COLLISION_EVENTS,
            pausable: Pausable::default(),
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius,
                    sectors: (10.0 * radius / SCALE).max(40.0) as usize,
                    stacks: (10.0 * radius / SCALE).max(40.0) as usize,
                    ..default()
                })),
                material,
                transform,
                ..default()
            }
        }
    }

    pub fn fixed(mut self) -> Self {
        self.rigid_body = RigidBody::Fixed;

        self
    }
}

impl Default for MarbleBundle {
    fn default() -> Self {
        MarbleBundle { 
            friction: Friction::coefficient(0.0),
            restitution: Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Max },

            in_game_entity: InGameEntity, 
            collider: Collider::ball(2.0 * MARBLE_RADIUS),
            rigid_body: RigidBody::Dynamic, 
            velocity: Velocity::zero(),
            gravity: GravityBundle::from_mass(3.0 * MARBLE_MASS),
            active_events: ActiveEvents::COLLISION_EVENTS,
            pausable: Pausable::default(),
            pbr_bundle: PbrBundle::default()
        }
    }
}