use super::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

// Applied to all objects that can be jumped off of
#[derive(Component)]
pub struct Jumpy;

#[derive(Component)]
pub struct CameraDir(pub Vec3);

#[derive(Default, Component)]
pub struct Pausable {
    pub velocity: Velocity,
    pub prev_rigid_body: RigidBody
}