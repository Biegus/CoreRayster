use nalgebra::base;

use crate::mhelper;
use crate::primitives::*;
use crate::renderdata::*;
use crate::typedef::*;

pub fn render(scene: &Scene) -> Vec<Vec<Vec3f>> {
    let camera = &scene.cam;
    let mut array: Vec<Vec<Vec3f>> = Vec::new();
    array.resize(camera.height as usize, Vec::<Vec3f>::new());
    for i in 0..array.len() {
        array[i].resize(camera.width as usize, Vec3f::new(0., 0., 0.));
    }

    for i in 0..camera.height {
        for j in 0..camera.width {
            let mut best: f32 = f32::INFINITY;
            for id in 0..scene.entities.len() {
                let surface = &scene.entities[id];
                let ii = camera.height - 1 - i;
                let ray = mhelper::get_vector_from_pixel(&camera, j as i32, ii as i32);
                let possible_hit = mhelper::hit_surface_geometry(ray, camera.e, &surface.geometry);
                if let Some(hit) = possible_hit {
                    let n = mhelper::get_surface_normal(hit, &surface.geometry);

                    let l = (scene.light.pos - hit).normalize();
                    let v = camera.e - hit;
                    let dist = v.magnitude(); // i need lenght anyway
                    let v = v / dist;

                    let h = (v + l).normalize();

                    let base_light =
                        0.6 * surface.color * scene.light.intensity * ((0_f32).max(n.dot(&l)));
                   
                    let specular = surface.specular_light.unwrap_or(scene.specular_default);
                    let ping =
                        specular * 0.45 * scene.light.intensity * (0_f32).max(n.dot(&h)).powf(30.);

                    let mut shadow = 0.0;
                    for other_id in 0..scene.entities.len() {
                        if other_id == id {
                            continue;
                        }
                        let other = &scene.entities[other_id];
                        let possible_shadow_hit =
                            mhelper::hit_surface_geometry(l, hit, &other.geometry);
                        if possible_shadow_hit.is_some() {
                            shadow -= 0.2;
                        }
                    }

                    let pixel = base_light + ping + shadow * Vec3f::new(1., 1., 1.);
                    if dist < best {
                        array[i][j] = pixel;
                        if pixel.magnitude() < 0.1 {
                            array[i][j] = Vec3f::new(0.03, 0.03, 0.03); //useless sqrt
                        }
                        best = dist;
                    }
                }
            }
        }
    }
    return array;
}
