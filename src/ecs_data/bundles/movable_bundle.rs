use super::*;

#[derive(Bundle)]
pub struct MovableBundle {
    pbr_bundle: PbrBundle,
    collider: Collider,
    active_events: ActiveEvents,
    rigid_body: RigidBody,
    gravity_bundle: GravityBundle,
    friction: Friction,
    restitution: Restitution,
    velocity: Velocity,
    sensor_channel: SensorChannel,
    in_game_entity: InGameEntity,
    jumpy: Jumpy
}

impl MovableBundle {
    pub fn with_properties(mut self, mass_properties: ColliderMassProperties, material_properties: MaterialProperties) -> Self {
        self.friction = material_properties.friction;
        self.restitution = material_properties.restitution;
        self.gravity_bundle.mass = mass_properties;

        self
    }
}

impl FromShape for MovableBundle {
    fn from_collider(collider: Collider) -> Self {
        Self {
            collider,
            ..Default::default()
        }
    }

    fn with_transform(mut self, transform: Transform) -> Self {
        self.pbr_bundle.transform = transform;

        self
    }

    fn with_pbr_bundle(mut self, pbr_bundle: PbrBundle) -> Self {
        self.pbr_bundle = pbr_bundle;

        self
    }
}

impl Default for MovableBundle {
    fn default() -> Self {
        Self {
            pbr_bundle: PbrBundle::default(),
            collider: Collider::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            rigid_body: RigidBody::Dynamic,
            gravity_bundle: GravityBundle::from_density(1.0),
            friction: MOVABLE_FRICTION,
            restitution: MOVABLE_RESTITUTION,
            velocity: Velocity::zero(),
            sensor_channel: SensorChannel::Warp | SensorChannel::Activator,
            in_game_entity: InGameEntity,
            jumpy: Jumpy
        }
    }
}