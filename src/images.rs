extern crate image;

use std::path::{Path, PathBuf};
use self::image::{Rgb, GenericImageView};

pub fn blend_images(path1: &PathBuf, path2: &PathBuf, out_path: &Path, amount: f64) {
    let img1 = image::open(path1).unwrap();
    let img2 = image::open(path2).unwrap();

    let (w, h) = img1.dimensions();
    if (w, h) != img2.dimensions() {
        panic!("{} and {} have different sizes", path1.display(), path2.display());
    }
    let mut out_image = image::DynamicImage::new_rgb8(w, h);

    let img1_pixels = img1.as_rgb8().unwrap();
    let img2_pixels = img2.as_rgb8().unwrap();
    let out_pixels = out_image.as_mut_rgb8().unwrap();

    for (x, y, pixel1) in img1_pixels.enumerate_pixels() {
        let pixel2 = img2_pixels.get_pixel(x, y);
        out_pixels.put_pixel(x, y, blend_pixels(pixel1, pixel2, amount));
    }

    out_pixels.save(out_path).unwrap();
}

pub fn blend_pixels(p1: &Rgb<u8>, p2: &Rgb<u8>, amount: f64) -> Rgb<u8> {
    if amount < 0. || amount > 1. {
        panic!("Invalid blend amount {}. Must be between 0 and 1.", amount);
    }
    let result = Rgb {
        data: [
            ((p1.data[0] as f64 * (1. - amount)) + (p2.data[0] as f64 * amount)) as u8, // R
            ((p1.data[1] as f64 * (1. - amount)) + (p2.data[1] as f64 * amount)) as u8, // G
            ((p1.data[2] as f64 * (1. - amount)) + (p2.data[2] as f64 * amount)) as u8, // B
        ],
    };
    return result;
}
