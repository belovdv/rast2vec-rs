use image::{DynamicImage, Rgb, RgbImage};

use imageproc::contours::find_contours;
use imageproc::drawing::draw_line_segment_mut;

fn f(input: DynamicImage) -> RgbImage {
    let mut result = RgbImage::new(input.width(), input.height());

    let input = input.to_luma8();

    for contour in find_contours::<u8>(&input) {
        for pairs in contour.points.chunks(2) {
            let from = (pairs[0].x as f32, pairs[0].y as f32);
            let to = (pairs[1].x as f32, pairs[1].y as f32);
            draw_line_segment_mut(&mut result, from, to, Rgb([0, 0, 0]));
        }
    }

    result
}

fn main() {
    let input_path = std::env::args().nth(1).unwrap();
    let input_path = std::path::Path::new(&input_path);
    let input = image::open(input_path).unwrap();

    let output = f(input);

    let output_path = std::env::args().nth(1).unwrap();
    let output_path = std::path::Path::new(&output_path);
    output.save(output_path).unwrap();
}
