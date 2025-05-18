use zharko::renderers::{Color, Image, Renderer, PPM};

const IMAGE_WIDTH: usize = 500;
const IMAGE_HEIGHT: usize = 500;

fn main() {
    let renderer = PPM::new();

    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    image.fill_rect(100, 100, 200, 200, Color::new(255, 0, 0));

    renderer.draw(image);
}
