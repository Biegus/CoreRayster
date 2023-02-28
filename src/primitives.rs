use crate::typedef::*;

pub struct PrimitiveSurface {
    pub geometry: PrimitiveSurfaceGeometry,
    pub color: Vec3f,
    pub specular_light: Option<Vec3f>, // none = default
}
pub enum PrimitiveSurfaceGeometry {
    Sphere(Sphere),
    Triangle(Triangle),
}
pub struct Sphere {
    pub pos: Vec3f,
    pub radius: f32,
}
pub struct Triangle {
    pub vertices: [Vec3f; 3],
}
impl PrimitiveSurface {
    pub fn new(
        geometry: PrimitiveSurfaceGeometry,
        color: Vec3f,
        specular_light: Option<Vec3f>,
    ) -> PrimitiveSurface {
        PrimitiveSurface {
            geometry,
            color,
            specular_light,
        }
    }
}
