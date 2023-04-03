use super::*;

pub trait FromShape where Self: Sized {
    fn from_collider(collider: Collider) -> Self;

    fn with_transform(self, transform: Transform) -> Self;

    fn with_pbr_bundle(self, pbr_bundle: PbrBundle) -> Self;

    fn from_mesh(
        mesh_assets: &Assets<Mesh>, 
        mesh_handle: &Handle<Mesh>,
    ) -> Self {
        Self::from_collider(
            Collider::from_bevy_mesh(
                mesh_assets.get(&mesh_handle).unwrap(), 
                // &ComputedColliderShape::TriMesh
                &ComputedColliderShape::ConvexDecomposition(VHACDParameters { 
                    resolution: 16,
                    ..Default::default()
                })
            ).expect("Could not create collider from mesh")
        )
    }

    fn from_shape(
        shape: String,
        sizes: Vec<f32>, 
        mesh_assets: &Assets<Mesh>,
        mesh_handle: &Handle<Mesh>,
    ) -> Self {
        match shape.as_str() {
            "box" => 
                Self::from_collider(Collider::cuboid(sizes[0] / 2.0, sizes[1] / 2.0, sizes[2] / 2.0))
            , "cylander" => 
                Self::from_collider(Collider::cylinder(sizes[0], sizes[1]))
            , "ball" => 
               Self::from_collider(Collider::ball(sizes[0]))
            , _ => 
                Self::from_mesh(mesh_assets, mesh_handle)
        }
    }

    fn from_shape_with_pbr(
        shape: String,
        sizes: Vec<f32>, 
        transform: Transform,
        mesh_assets: &mut Assets<Mesh>,
        material: Handle<StandardMaterial>,
        mesh_handle: &Handle<Mesh>,
    ) -> Self {
        match shape.as_str() {
            "box" => 
                Self::from_collider(Collider::cuboid(sizes[0] / 2.0, sizes[1] / 2.0, sizes[2] / 2.0))
                // .with_pbr_bundle(PbrBundle {
                //     mesh: mesh_assets.add(shape::Box::new(sizes[0], sizes[1], sizes[2]).into()),
                //     material,
                //     transform,
                //     ..Default::default()
                // })
            , "cylander" => 
                Self::from_collider(Collider::cylinder(sizes[0], sizes[1]))
                // .with_pbr_bundle(PbrBundle {
                //     mesh: mesh_assets.add(shape::Cylinder { 
                //         height: 2.0 * sizes[0], 
                //         radius: sizes[1], 
                //         resolution: CYLINDER_RESOLUTION, 
                //         segments: NUM_CYLINDER_SEGMENTS
                //     }.into()),
                //     material,
                //     transform,
                //     ..Default::default()
                // })
            , "ball" => 
               Self::from_collider(Collider::ball(sizes[0]))
            //    .with_pbr_bundle(PbrBundle {
            //        mesh: mesh_assets.add(shape::UVSphere {
            //         radius: sizes[0],
            //         sectors: NUM_SPHERE_SECTORS,
            //         stacks: NUM_SPHERE_STACKS
            //        }.into()),
            //        material,
            //        transform,
            //        ..Default::default()
            //    })
            , _ => 
                Self::from_mesh(mesh_assets, mesh_handle)
                // .with_pbr_bundle(PbrBundle {
                //     mesh: mesh_handle.clone(),
                //     material,
                //     transform,
                //     ..Default::default()
                // })
        }.with_pbr_bundle(PbrBundle {
            mesh: mesh_handle.clone(),
            material,
            transform,
            ..Default::default()
        })
    }
}