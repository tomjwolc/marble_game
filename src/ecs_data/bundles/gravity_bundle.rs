use super::*;

#[derive(Bundle)]
pub struct GravityBundle {
    pub gravity: Gravity,
    pub grivity_scale: GravityScale,
    pub force: ExternalForce,
    pub mass: ColliderMassProperties
}

impl GravityBundle {
    pub fn from_mass(mass: f32) -> Self {
        Self {
            mass: ColliderMassProperties::Mass(mass),
            ..Default::default()
        }
    }

    pub fn from_density(density: f32) -> Self {
        Self {
            mass: ColliderMassProperties::Density(density),
            ..Default::default()
        }
    }

    pub fn with_gravity(mut self, gravity: Gravity) -> Self {
        self.gravity = gravity;

        self
    }
}

impl Default for GravityBundle {
    fn default() -> Self {
        Self { 
            gravity: Gravity(-GRAVITY * Vec3::Y, GravityType::Planets), 
            grivity_scale: GravityScale(0.0),
            force: ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO } ,
            mass: ColliderMassProperties::Mass(1.0)
        }
    }
}