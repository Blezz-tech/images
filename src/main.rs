use std::fs;

const IMG_PATH: &str = "./target/imgs/";
const IMG_NAME: &str = "fractal.png";

fn main() {
    let _ = fs::create_dir_all("./target/imgs/");

    let imgx = 256;
    let imgy = 256;
    let imgz: i32 = 256;

    for z in 0..imgz {
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (1.0 * x as f32) as u8;
            let g = (1.0 * z as f32) as u8;
            let b = (1.0 * y as f32) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        imgbuf
            .save(IMG_PATH.to_owned() + &z.to_string() + IMG_NAME)
            .unwrap();
    }
}
