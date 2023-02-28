use crate::typedef::*;
use bmp::{px, Image, Pixel};
use std::{
    env,
    process::{Command, Output},
};
pub fn img_from_array_with_gradient(ar: &Vec<Vec<Vec3f>>) -> Image {
    let h = ar.len() as u32;
    let w = ar[0].len() as u32;
    let mut img = Image::new(w, h);
    for (x, y) in img.coordinates() {
        let f = (ar[y as usize][x as usize] * 255.);

        if (f.magnitude() == 0.) {
            //useless sqr
            let grad = ((y as f32 / (h as f32)) * 120.) as u8;
            img.set_pixel(x, y, px!(grad, grad, grad));
        } else {
            img.set_pixel(x, y, px!(f.x, f.y, f.z));
        }
    }
    return img;
}
pub fn create_bmp_gif_with_terminal() -> Result<(), String> {
    if env::consts::OS == "windows" {
        return Err(String::from("auto gif supported only linux/wsl"));
    }
    return Command::new("sh")
        .arg("-c")
        .arg("convert -delay 10 -loop 0 *.bmp res.gif")
        .output()
        .map_err(|_| String::from("terminal error"))
        .map(|_| ());
}
