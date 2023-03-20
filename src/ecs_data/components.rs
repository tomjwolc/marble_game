use super::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

// Applied to all objects that can be jumped off of
#[derive(Component)]
pub struct Jumpy;

#[derive(Component, Debug)]
pub struct CameraDir {
    pub horizontal_direction: Vec3,
    pub pitch: f32
}

impl Default for CameraDir {
    fn default() -> Self {
        Self { horizontal_direction: Vec3::Z, pitch: 0.0 }
    }
}

#[derive(Default, Component)]
pub struct Pausable {
    pub velocity: Velocity,
    pub prev_rigid_body: Option<RigidBody>
}

#[derive(Component, Clone)]
pub struct Gravity(pub Vec3, pub bool);

#[derive(Component)]
pub struct GravityChangeSensor(pub Vec3);

#[derive(Component)]
pub struct NotGravityWell;