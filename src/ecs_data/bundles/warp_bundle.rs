use super::*;

#[derive(Bundle)]
pub struct WarpBundle {
    pub friction: Friction,
    pub restitution: Restitution,

    pub activatable: Activatable,
    pub warp_to: WarpTo,
    pub sensor_channels: SensorChannel,
    pub object_events: ObjectEvents,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,

    pub collider: Collider,
    pub pbr_bundle: PbrBundle,
    pub in_game_entity: InGameEntity,
}

impl Default for WarpBundle {
    fn default() -> Self {
        Self {
            friction: Friction::coefficient(WARP_FRICTION),
            restitution: Restitution::coefficient(WARP_RESTITUTION),
            activatable: Activatable { requirements: Vec::new(), is_active: true },
            warp_to: WarpTo::Out,
            sensor_channels: SensorChannel::CanJump,
            object_events: ObjectEvents::new(),
            collider: Collider::cylinder(0.25 * SCALE, 0.75 * SCALE),
            pbr_bundle: PbrBundle::default(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::all(),
            in_game_entity: InGameEntity,
        }
    }
}