use super::Color;

use crate::rast2vec::ImageRGBA;

use image::{Rgba, RgbaImage};

pub fn ndarray2image(array: &ImageRGBA) -> RgbaImage {
    let (h, w) = unsafe { array.size().get() };
    let mut result = RgbaImage::new(w as u32, h as u32);
    let array = array.as_raw();

    for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
        result[(x as u32, y as u32)] = Rgba(array[(y, x)].rgba);
    }

    result
}

pub fn image2ndarray(img: &RgbaImage, upscale: usize) -> ImageRGBA {
    let h = img.height() as usize * upscale;
    let w = img.width() as usize * upscale;
    let mut result = ndarray::Array::from_elem((h, w), Color { rgba: [0 as u8; 4] });

    for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
        result[(y, x)] = Color {
            rgba: img[((x / upscale) as u32, (y / upscale) as u32)].0,
        };
    }

    result.into()
}

pub fn ndarray_subview(img: &ImageRGBA, l: usize, u: usize, h: usize, w: usize) -> ImageRGBA {
    let mut result = ndarray::Array::from_elem((h, w), Color { rgba: [0 as u8; 4] });

    for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
        result[(y, x)] = img.as_raw()[(y + u, x + l)]
    }

    result.into()
}
