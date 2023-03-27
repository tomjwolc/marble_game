// use bevy::scene::SceneInstance;

use bevy::gltf::{GltfMesh, GltfNode};
use serde::{Serialize, Deserialize};

use super::*;

pub fn load_glb_asset(
    asset_server: Res<AssetServer>,
    mut level_stack: ResMut<LevelStack>
) {
    if level_stack.get_current_level().handle.is_none() {
        level_stack.get_current_level_mut().handle = Some(asset_server.load(
            format!("levels/{}.glb", level_stack.get_current_level().file_name).as_str()
        ));
    }
}

pub fn load_glb(
    mut commands: Commands,
    level_stack: Res<LevelStack>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    mesh_assets: ResMut<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>
) {
    if let Some(gltf) = gltf_assets.get(&level_stack.get_current_level().handle.as_ref().unwrap()) {
        for node_handle in gltf.nodes.iter() {
            //     Continues if the node asset does not exist, the node has no mesh, 
            // or the mesh asset does not exist
            extract!(continue; 
                Some( node ) = gltf_node_assets.get(node_handle);
                Some( gltf_mesh_handle ) = &node.mesh;
                Some( gltf_mesh ) = gltf_mesh_assets.get(gltf_mesh_handle)
            );

            // println!("{}",  if let Some(extras) = &node.extras { extras.value.as_str() } else { "{}" });

            let extras_data: ExtrasData = serde_json::from_str(
                if let Some(extras) = &node.extras { extras.value.as_str() } else { "{}" }
            ).unwrap();

            // Most objects only have 1 primitive
            let mesh0 = gltf_mesh.primitives[0].mesh.clone();
            let material0 = if let Some( material ) 
                = gltf_mesh.primitives[0].material.clone() {
                material
            } else {
                default_material.0.clone()
            };

            let object_type = GltfObjectType::from(extras_data);
            let mut transform = node.transform.with_scale(SCALE * Vec3::ONE);
            transform.translation *= SCALE;

            println!("{:?}", object_type);

            match object_type {
                GltfObjectType::Warp(
                    warp_to, 
                    activatable
                ) => {
                    commands.spawn(WarpBundle {
                        pbr_bundle: PbrBundle {
                            mesh: mesh0.clone(),
                            material: material0,
                            transform,
                            ..Default::default()
                        },
                        collider: Collider::from_bevy_mesh(
                            mesh_assets.get(&mesh0).unwrap(), 
                            &ComputedColliderShape::TriMesh
                        ).expect("Could not create warp collider from mesh"),
                        warp_to: warp_to.clone(),
                        activatable,
                        ..Default::default()
                    }).with_children(|parent| {
                        parent.spawn((SensorBundle::from_collider(
                            Collider::cylinder(WARP_SENSOR_HEIGHT / 2.0, 1.0)
                            ).with_transform(Transform::from_xyz(0.0, WARP_SENSOR_HEIGHT / 2.0, 0.0))
                            .with_sensor_channel(SensorChannel::Warp), 
                            warp_to
                        ));
                    });
                },

                GltfObjectType::Sensor(
                    sensor_channel, 
                    shape, 
                    sizes
                ) => {
                    commands.spawn(SensorBundle::from_shape(
                        shape, sizes, &mesh_assets, 
                        &mesh0
                    ).with_transform(transform).with_sensor_channel(sensor_channel));
                },

                GltfObjectType::Movable(
                    shape, 
                    sizes, 
                    mass_properties,
                    material_properties
                ) => {
                    commands.spawn(MovableBundle::from_shape(
                        shape, sizes, &mesh_assets, 
                        &mesh0
                    ).with_pbr_bundle(PbrBundle {
                        mesh: mesh0,
                        material: material0,
                        transform,
                        ..Default::default()
                    }).with_properties(mass_properties, material_properties));
                },
                
                GltfObjectType::Fixed(
                    shape, 
                    sizes, 
                    material_properties
                ) => {
                    commands.spawn(FixedBundle::from_shape(
                        shape, sizes, &mesh_assets, 
                        &mesh0
                    ).with_pbr_bundle(PbrBundle {
                        mesh: mesh0,
                        material: material0,
                        transform,
                        ..Default::default()
                    }).with_properties(material_properties));
                },

                _ => {
                    for primitive in gltf_mesh.primitives.iter() {
                        let material = if let Some( material ) = primitive.material.clone() {
                            material
                        } else {
                            default_material.0.clone()
                        };
                        
                        commands.spawn(FixedBundle::from_mesh(
                            &mesh_assets, 
                            &primitive.mesh
                        ).with_pbr_bundle(PbrBundle {
                            mesh: primitive.mesh.clone(),
                            material,
                            transform,
                            ..Default::default()
                        }));
                    }
                }
            };
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExtrasData {
    // warp info
        file_name: Option<String>,
        activation_requirements: Option<Vec<usize>>,
    // activator info
        activator_type: Option<String>,
        activator_id: Option<usize>,
    // sensor/object info
        // Box vs ball vs ..
        shape: Option<String>,
        sizes: Option<Vec<f32>>,
    // sensor info
        sensor_channel: Option<String>,
    // object info
        // blender doesn't support bool's as custom properties >:(
        is_dynamic: Option<i8>,
        // Either mass or density with be Some(..)
        mass: Option<f32>,
        density: Option<f32>,
        // default vs ice vs ..
        material_type: Option<String>
}

#[derive(Debug)]
enum GltfObjectType {
    Warp(WarpTo, Activatable),
    Sensor(SensorChannel, String, Vec<f32>),
    Movable(String, Vec<f32>, ColliderMassProperties, MaterialProperties),
    Fixed(String, Vec<f32>, MaterialProperties),
    Object
}

impl GltfObjectType {
    

    /*
    fn spawn_bundles(&self, 
        commands: &mut Commands, 
        primitives: &Vec<GltfPrimitive>, 
        mesh_assets: &mut Assets<Mesh>,
        default_material: &DefaultMaterial,
        transform: Transform
    ) -> impl Bundle {
        match self {
            GltfObjectType::Warp(warp_to, activatable) => {
                let Some( mesh ) = mesh_assets.get(&primitives[0].mesh) else { return };

                commands.spawn(WarpBundle {
                    collider: Collider::from_bevy_mesh(
                        mesh, 
                        &ComputedColliderShape::TriMesh
                    ).expect("Could not create collider from warp mesh"),
                    pbr_bundle: PbrBundle {
                        mesh: primitives[0].mesh.clone(),
                        material: primitives[0].material.clone().expect("Warp primitive didn't have material"),
                        transform,
                        ..Default::default()
                    },
                    warp_to: warp_to.clone(),
                    activatable: activatable.clone(),
                    ..Default::default()
                });
            },
            GltfObjectType::RespawnDetector => {
                let Some( mesh ) = mesh_assets.get(&primitives[0].mesh) else { return };

                commands.spawn((
                    Collider::from_bevy_mesh(
                        mesh, 
                        &ComputedColliderShape::TriMesh
                    ).expect("Could not create collider from warp mesh"),
                    RigidBody::Dynamic,
                    LockedAxes::TRANSLATION_LOCKED,
                    TransformBundle::from_transform(transform),
                    Sensor,
                    SensorChannel::Respawn,
                    InGameEntity
                ));
            },
            GltfObjectType::Shape(shape, size) => {
                for primitive in primitives.iter() {
                    commands.spawn((
                        PbrBundle {
                            mesh: mesh_assets.add(shape::Box::new(
                                2.0 * size[0], 2.0 * size[1], 2.0 * size[2]
                            ).into()),
                            material: primitive.material
                                .as_ref()
                                .unwrap_or(&default_material.0)
                                .clone(),
                            transform,
                            ..Default::default()
                        },
                        Collider::cuboid(size[0], size[1], size[2]),
                        ActiveEvents::COLLISION_EVENTS,
                        RigidBody::Dynamic,
                        GravityBundle::from_mass(0.2),
                        Friction::coefficient(0.0),
                        Restitution::coefficient(0.2),
                        Velocity::zero(),
                        Jumpy,
                        Pausable::default(),
                        InGameEntity
                    ));
                }
            },
            _ => {
                for primitive in primitives.iter() {
                    let Some( mesh ) = mesh_assets.get(&primitive.mesh) else { continue };

                    commands.spawn((
                        PbrBundle {
                            mesh: primitive.mesh.clone(),
                            material: primitive.material
                                .as_ref()
                                .unwrap_or(&default_material.0)
                                .clone(),
                            transform,
                            ..Default::default()
                        },
                        Collider::from_bevy_mesh(
                            mesh, 
                            &ComputedColliderShape::TriMesh
                        ).expect("Could not create collider from bevy mesh"),
                        Jumpy,
                        InGameEntity
                    ));
                }
            }
        }
    }
    */
}

impl From<ExtrasData> for GltfObjectType {
    fn from(extras_data: ExtrasData) -> Self {
        match extras_data {

            // Warp
            ExtrasData { 
                file_name: Some(file_name), 
                activation_requirements,
                ..
            } => {
                // If activation_requirements is none, then there are no activation requirements
                let requirements = if let Some(requirements) = activation_requirements 
                    { requirements } else { Vec::new() };
                let warp_to = if file_name.len() == 0 { WarpTo::Out } else { WarpTo::File(file_name) };

                // Only starts out active if there are no requirements
                let is_active = requirements.len() == 0;

                GltfObjectType::Warp(
                    warp_to, 
                    Activatable {
                        requirements,
                        is_active
                    }
                )
            }, 
            
            // Sensor
            ExtrasData {
                sensor_channel: Some(sensor_channel),
                shape: Some(shape),
                sizes: Some(sizes),
                ..
            } => GltfObjectType::Sensor(
                match sensor_channel.to_lowercase().as_str() {
                    "warp" => SensorChannel::Warp,
                    "respawn" => SensorChannel::Respawn,
                    "button" => SensorChannel::Button,
                    _ => SensorChannel::none()
                }, 
                shape, 
                sizes 
            ), 

            // Movable and Fixed
            ExtrasData { 
                is_dynamic: Some(is_dynamic), 
                shape: Some(shape),
                sizes: Some(sizes),
                material_type,
                mass,
                density,
                .. 
            } => {
                let material_properties = 
                    MaterialProperties::from(material_type, is_dynamic != 0);
                    
                let mass_properties = if let Some(mass) = mass {
                    ColliderMassProperties::Mass(mass)
                } else if let Some(density) = density {
                    ColliderMassProperties::Density(density)
                } else {
                    ColliderMassProperties::Mass(1.0)
                };

                if is_dynamic != 0 {
                    GltfObjectType::Movable(shape, sizes, mass_properties, material_properties)
                } else {
                    GltfObjectType::Fixed(shape, sizes, material_properties)
                }
            },

            // Regular object
            _ => GltfObjectType::Object
        }
    }
}