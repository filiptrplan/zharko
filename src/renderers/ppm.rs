use std::fs;

use indicatif::ProgressBar;

#[derive(Default)]
pub struct PPM {}

impl PPM {
    pub fn new() -> Self {
        PPM {}
    }
}

const OUTPUT_FILE: &str = "test.ppm";

impl super::Renderer for PPM {
    fn draw(self, image: super::Image) {
        let mut buffer = String::new();

        let bar = ProgressBar::new(image.height as u64);

        // Header
        buffer += format!("P3\n{} {}\n255\n", image.width, image.height).as_str();

        for y in 0..image.height {
            bar.inc(1);
            for x in 0..image.width {
                let color = image.get_pixel(x, y);
                buffer += format!("{} {} {} ", color.r, color.g, color.b).as_str();
            }
            buffer += "\n";
        }

        bar.finish();

        if let Err(e) = fs::write(OUTPUT_FILE, buffer) {
            panic!("Error writing to file: {}", e);
        }
    }
}
