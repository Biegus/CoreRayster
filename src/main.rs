use std::{f32::consts::PI, io::stdin};

mod mhelper;

mod typedef;
use crossterm_cursor::cursor;
use renderdata::Light;
use typedef::*;

use crate::primitives::PrimitiveSurface;

mod fhelper;
mod primitives;
mod renderdata;
mod renderer;

fn main() {
    make(100);
    println!("Render in better quality?");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failure while reading");
    if (s.trim() == "y") {
        make(400);
    }
}
fn make(quality: usize) {
    println!("Rendering 100 frames at {}x{} resolution", quality, quality);
    let mut cursor = cursor();
    let mat = mat3::new(1., 0., 0., 0., 1., 0., 0., 0., -1.);

    for k in 0..100 {
        let data = renderdata::Camera {
            e: Vec3f::new(0., 0., -2.),
            d: 1.2,
            mat: mat,
            width: quality,
            height: quality,
        };
        let sphere = primitives::Sphere {
            radius: 0.3,
            pos: Vec3f::new(0., 0., 0.1),
        };
        let sphere2 = primitives::Sphere {
            radius: 0.3,
            pos: Vec3f::new(-0.6, 0.0, 0.),
        };
        let f = (k as f32) * 0.03;
        let triangle = primitives::Triangle {
            vertices: [
                Vec3f::new(-0.4, 0.0 + f, 0.),
                Vec3f::new(0.4, 0. + f, 0.),
                Vec3f::new(0., 0.4 + f, 0.),
            ],
        };

        //println!("{}", mhelper::get_triangle_normal(&triangle));
        let scene = renderdata::Scene {
            cam: data,
            light: Light {
                intensity: 1.,
                pos: mhelper::rotate_y(Vec3f::new(0.0, 0., -200.), (k as f32 / 100.) * 2. * PI),
            },
            entities: vec![PrimitiveSurface::new(
                primitives::PrimitiveSurfaceGeometry::Triangle(triangle),
                Vec3f::new(1., 0., 0.),
                None,
            )],
            specular_default: Vec3f::new(1., 1., 1.),
        };
        let ar = renderer::render(&scene);
        let image = fhelper::img_from_array_with_gradient(&ar);

        image.save(format!("{:0>2}.bmp", k)).unwrap();
        print!("{:0>2}/100", k + 1);
        cursor.move_left(6).expect("failure while moving cursor");
    }
    println!("");
    println!("making gif...");
    fhelper::create_bmp_gif_with_terminal()
        .unwrap_or_else(|a| println!("Gif creation failed: {}", a));
    println!("finished");
}
