// Это волшебная функция, которая принимает список объектов и рисует их.
// Её не нужно редактировать.

use image::{ImageBuffer, Rgb};

use super::{Color, Point};

fn color2rgb(c: Color) -> Rgb<u8> {
    match c {
        Color::Black => Rgb([0, 0, 0]),
        Color::Green => Rgb([50, 200, 50]),
        Color::Blue => Rgb([50, 50, 200]),
        Color::Yellow => Rgb([200, 200, 50]),
        Color::White => Rgb([250, 250, 250]),
    }
}

pub fn draw_image(f: impl Fn(Point<i32>) -> Color) {
    // Construct a new by repeated calls to the supplied closure.
    let img: ImageBuffer<_, _> = ImageBuffer::from_fn(500, 500, |x, y| {
        color2rgb(f(Point {
            x: x as i32,
            y: y as i32,
        }))
    });

    img.save("image.png").unwrap()
}
