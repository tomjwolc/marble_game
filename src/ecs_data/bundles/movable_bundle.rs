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
    sensor_channels: SensorChannel,
    object_events: ObjectEvents,
    in_game_entity: InGameEntity,
    jumpy: Jumpy
}

impl MovableBundle {
    pub fn new(
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
        transform: Transform,
        collider: Collider,
        sensor_channels: SensorChannel,
        mass: ColliderMassProperties,
        material_properties: MaterialProperties
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform,
                ..Default::default()
            },
            collider,
            gravity_bundle: GravityBundle { mass, ..default() },
            friction: material_properties.friction,
            restitution: material_properties.restitution,
            sensor_channels,
            ..Default::default()
        }
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
            sensor_channels: SensorChannel::Respawn.not(),
            object_events: ObjectEvents::new(),
            in_game_entity: InGameEntity,
            jumpy: Jumpy
        }
    }
}