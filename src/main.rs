use minifb::{Key, Window, WindowOptions};
use opencv::core::Point;
use opencv::imgproc::{CHAIN_APPROX_SIMPLE, RETR_EXTERNAL};
use opencv::prelude::{Mat, MatTraitConst};
use rgb::alt::BGR8;
use rgb::{RGB8, RGBA, RGBA8};

struct Buffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}
impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn data(&self) -> &[u32] {
        &self.data
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.data[y * self.width + x] = color;
    }

    pub fn set_pixel_rgb(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }

        self.set_pixel(x, y, (r as u32) << 16 | (g as u32) << 8 | (b as u32));
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn write_cv_8uc3(&mut self, image: &Mat) {
        for y in 0..image.rows() {
            for x in 0..image.cols() {
                let color = image.at_2d::<BGR8>(y, x).expect("Failed to get RGB value");
                self.set_pixel_rgb(x as usize, y as usize, color.r, color.g, color.b);
            }
        }
    }

    pub fn write_cv_8uc1(&mut self, image: &Mat) {
        for y in 0..image.rows() {
            for x in 0..image.cols() {
                let color = image.at_2d::<u8>(y, x).expect("Failed to get RGB value");
                self.set_pixel_rgb(x as usize, y as usize, *color, *color, *color);
            }
        }
    }
}
fn main() {
    // Setup
    let mut buffer = Buffer::new(1280, 720);
    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width,
        buffer.height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        step(&mut buffer);

        window
            .update_with_buffer(buffer.data(), buffer.width, buffer.height)
            .unwrap();
    }
}

fn step(buffer: &mut Buffer) {
    // Load image
    let image_cv_8uc3 =
        opencv::imgcodecs::imread("assets/1.png", opencv::imgcodecs::IMREAD_ANYCOLOR)
            .expect("Failed to load image assets/1.png");
    let image_size = image_cv_8uc3.size().expect("Failed to get image size");

    // Find edges
    let mut edges_cv_8uc1 = unsafe {
        Mat::new_rows_cols(image_size.height, image_size.width, opencv::core::CV_8UC1)
            .expect("Failed to create edges-mat")
    };
    opencv::imgproc::canny(&image_cv_8uc3, &mut edges_cv_8uc1, 100.0, 200.0, 3, false)
        .expect("Failed to compute edges");

    // Write out data
    let image = edges_cv_8uc1;
    buffer.write_cv_8uc1(&image);
}
