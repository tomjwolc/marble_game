use bevy::gltf::Gltf;

use super::*;

#[derive(Resource, Clone)]
pub struct DefaultMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct NextLevel {
    pub handle: Option<Handle<Gltf>>,
    pub file_name: &'static str
}

impl NextLevel {
    pub fn from_level(file_name: &'static str) -> Self {
        Self {
            handle: None,
            file_name
        }
    }

    pub fn set_file(&mut self, file_name: &'static str) {
        *self = Self {
            handle: None,
            file_name
        }
    }
}