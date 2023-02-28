use crate::primitives::*;
use crate::typedef::*;
pub struct Camera {
    pub e: Vec3f,
    pub d: f32,
    pub mat: mat3,
    pub width: usize,
    pub height: usize,
}
pub struct Light {
    pub pos: Vec3f,
    pub intensity: f32,
}
pub struct Scene {
    pub cam: Camera,
    pub light: Light,
    pub entities: Vec<PrimitiveSurface>,
    pub specular_default: Vec3f,
}
