use crate::primitives::*;
use crate::renderdata::Camera;
use crate::typedef::*;
use nalgebra::{Matrix3x1, Vector2, Vector3};
pub fn hit_surface_geometry(
    ray: Vec3f,
    e: Vec3f,
    geometry: &PrimitiveSurfaceGeometry,
) -> Option<Vec3f> {
    match geometry {
        PrimitiveSurfaceGeometry::Sphere(sphere) => hit_sphere(ray, e, &sphere),
        PrimitiveSurfaceGeometry::Triangle(triangle) => hit_triangle(ray, e, &triangle),
    }
}
pub fn get_surface_normal(pos: Vec3f, geometry: &PrimitiveSurfaceGeometry) -> Vec3f {
    match geometry {
        PrimitiveSurfaceGeometry::Sphere(sphere) => get_sphere_normal(pos, &sphere),
        PrimitiveSurfaceGeometry::Triangle(triangle) => get_triangle_normal(&triangle),
    }
}
pub fn hit_triangle(ray: Vec3f, ee: Vec3f, triangle: &Triangle) -> Option<Vec3f> {
    let a = triangle.vertices[0].x - triangle.vertices[1].x;
    let b = triangle.vertices[0].y - triangle.vertices[1].y;
    let c = triangle.vertices[0].z - triangle.vertices[1].z;

    let d = triangle.vertices[0].x - triangle.vertices[2].x;
    let e = triangle.vertices[0].y - triangle.vertices[2].y;
    let f = triangle.vertices[0].z - triangle.vertices[2].z;

    let g = ray.x;
    let h = ray.y;
    let i = ray.z;

    let j = triangle.vertices[0].x - ee.x;
    let k = triangle.vertices[0].y - ee.y;
    let l = triangle.vertices[0].z - ee.z;

    let m = a * (e * i - h * f) + b * (g * f - d * i) + c * (d * h - e * g);

    let beta = (j * (e * i - h * f) + k * (g * f - d * i) + l * (d * h - e * g)) / m;
    let gamma = (i * (a * k - j * b) + h * (j * c - a * l) + g * (b * l - k * c)) / m;
    let t = -(f * (a * k - j * b) + e * (j * c - a * l) + d * (b * l - k * c)) / m;

    if beta >= 0. && beta <= 1. - gamma && gamma >= 0. && gamma <= 1. && t > 0. {
        return Some(
            triangle.vertices[0]
                + beta * (triangle.vertices[1] - triangle.vertices[0])
                + gamma * (triangle.vertices[2] - triangle.vertices[0]),
        );
    }
    return None;
}
pub fn hit_sphere(ray: Vec3f, e: Vec3f, sphere: &Sphere) -> Option<Vec3f> {
    let dif = e - sphere.pos;
    let rdr = ray.dot(&ray);
    let disc =
        ray.dot(&(dif)).powi(2) - (rdr) * ((dif).dot(&(dif)) - sphere.radius * sphere.radius);
    if disc < 0. {
        return None;
    }
    let sq = disc.sqrt();

    let b = -ray.dot(&dif);
    let t1 = (b + sq) / (rdr);
    let t2 = (b - sq) / (rdr);
    if t1 <= 0. && t2 <= 0. {
        return None;
    }
    let tf;
    if t1 < 0. {
        tf = t2
    } else if t2 < 0. {
        tf = t1
    } else {
        tf = t1.min(t2);
    }
    return Some(e + ray * tf);
}
pub fn get_vector_from_pixel(data: &Camera, x: i32, y: i32) -> Vector3<f32> {
    let u = data.mat.column(0);
    let v = data.mat.column(1);
    let w = data.mat.column(2);
    let fw = data.width as f32;
    let fh = data.height as f32;
    let fx = x as f32;
    let fy = y as f32;

    let p_point: Vec2f = Vec2f::new(
        fw / 2. / (data.width as f32) - 0.5 / (data.width as f32),
        fh / 2. / (data.height as f32) - 0.5 / (data.height as f32),
    );
    return -data.d * w
        + ((fx / (data.width as f32)) - p_point[0]) * u
        + (fy / (data.height as f32) - p_point[1]) * v;
}
pub fn get_sphere_normal(pos: Vec3f, sphere: &Sphere) -> Vec3f {
    return (pos - sphere.pos).normalize();
}
pub fn get_triangle_normal(triangle: &Triangle) -> Vec3f {
    let x = triangle.vertices[1] - triangle.vertices[0];
    let y = triangle.vertices[2] - triangle.vertices[0];
    return y.cross(&x).normalize();
}
pub fn rotate_y(a: Vec3f, theta: f32) -> Vec3f {
    let cs = theta.cos();
    let sn = theta.sin();
    let x = a.x;
    let y = a.y;
    let z = a.z;
    return Vec3f::new(
        cs * x + 0. * y + sn * z,
        0. * x + 1. * y + 0. * z,
        -sn * x + 0. + cs * z,
    );
}
