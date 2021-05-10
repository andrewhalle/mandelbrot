use nannou::image;
use nannou::prelude::*;
use num::Complex;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    texture: nannou::wgpu::Texture,
    center: (f64, f64),
    zoom: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(512, 512)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let center = (1.0, 0.0);
    let zoom = 4.0;
    let img = mandelbrot_image(center, zoom);
    let texture = wgpu::Texture::from_image(app, &img);

    Model {
        texture,
        center,
        zoom,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    use Key::*;

    match key {
        Left => {
            model.center.0 += model.zoom / 4.0;
        }
        Right => {
            model.center.0 -= model.zoom / 4.0;
        }
        Up => {
            model.center.1 += model.zoom / 4.0;
        }
        Down => {
            model.center.1 -= model.zoom / 4.0;
        }
        J => {
            model.zoom /= 2.0;
        }
        K => {
            model.zoom *= 2.0;
        }
        _ => {}
    }

    let img = mandelbrot_image(model.center, model.zoom);
    let texture = wgpu::Texture::from_image(app, &img);

    model.texture = texture;
}

fn mandelbrot_image(center: (f64, f64), zoom: f64) -> image::DynamicImage {
    image::DynamicImage::ImageRgb8(image::ImageBuffer::from_fn(
        1024,
        1024,
        mandelbrot(512, 512, center, zoom),
    ))
}

enum MandelbrotResult {
    Escaped(u32),
    Bound,
}

impl MandelbrotResult {
    fn pixel(self) -> image::Rgb<u8> {
        use MandelbrotResult::*;
        let colors = [
            image::Rgb([42, 72, 88]),
            image::Rgb([41, 77, 93]),
            image::Rgb([39, 81, 99]),
            image::Rgb([37, 86, 103]),
            image::Rgb([34, 91, 108]),
            image::Rgb([30, 96, 113]),
            image::Rgb([26, 101, 117]),
            image::Rgb([21, 106, 121]),
            image::Rgb([14, 111, 125]),
            image::Rgb([5, 116, 128]),
            image::Rgb([0, 121, 131]),
            image::Rgb([0, 126, 134]),
            image::Rgb([0, 132, 136]),
            image::Rgb([0, 137, 138]),
            image::Rgb([0, 142, 140]),
            image::Rgb([0, 147, 141]),
            image::Rgb([0, 152, 142]),
            image::Rgb([3, 157, 143]),
            image::Rgb([18, 162, 143]),
            image::Rgb([30, 167, 143]),
            image::Rgb([40, 172, 143]),
            image::Rgb([50, 177, 142]),
            image::Rgb([60, 182, 141]),
            image::Rgb([70, 187, 140]),
            image::Rgb([80, 191, 139]),
            image::Rgb([90, 196, 137]),
            image::Rgb([100, 201, 135]),
            image::Rgb([110, 205, 133]),
            image::Rgb([121, 210, 131]),
            image::Rgb([131, 214, 129]),
            image::Rgb([142, 218, 127]),
            image::Rgb([153, 222, 124]),
            image::Rgb([165, 226, 122]),
            image::Rgb([176, 230, 120]),
            image::Rgb([188, 234, 117]),
            image::Rgb([200, 237, 115]),
            image::Rgb([212, 241, 113]),
            image::Rgb([224, 244, 112]),
            image::Rgb([237, 247, 111]),
            image::Rgb([250, 250, 110]),
        ];

        match self {
            Bound => image::Rgb([0, 0, 0]),
            Escaped(i) => colors[(i % 26) as usize],
        }
    }
}

fn mandelbrot(
    height: u32,
    width: u32,
    center: (f64, f64),
    zoom: f64,
) -> impl Fn(u32, u32) -> image::Rgb<u8> {
    move |x, y| {
        let x = ((x as f64) / (width as f64) * zoom) - (center.0 + zoom / 2.0);
        let y = -(((y as f64) / (height as f64) * zoom) - (center.1 + zoom / 2.0));

        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(x, y);
        let limit = 2.0;

        let result = {
            let mut i = 0;
            loop {
                z = (z * z) + c;

                if z.norm() > limit {
                    break MandelbrotResult::Escaped(i);
                }
                if i == 1000 {
                    break MandelbrotResult::Bound;
                }

                i += 1;
            }
        };

        result.pixel()
    }
}
