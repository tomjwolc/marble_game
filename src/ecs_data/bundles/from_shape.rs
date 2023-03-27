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
                &ComputedColliderShape::TriMesh
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
            "box" => Self::from_collider(
                Collider::cuboid(sizes[0], sizes[1], sizes[2])
            ), "cylander" => Self::from_collider(
                Collider::cylinder(sizes[0], sizes[1])
            ), "ball" => Self::from_collider(
                Collider::ball(sizes[0])
            ), _ => Self::from_mesh(
                mesh_assets, 
                mesh_handle, 
            )
        }
    }
}