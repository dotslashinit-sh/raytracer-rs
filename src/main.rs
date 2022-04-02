use std::env;
use raytracer_rs::*;

fn sphere_hit(r: &ray::Ray, center: Vec3, radius: f32) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a = r.direct().length_squared();
    let half_b = vec_dot_product(&r.direct(), &oc);
    let c = oc.length_squared() - radius * radius;
    half_b * half_b - a * c > 0.0
}

fn get_color(r: &ray::Ray, center: Vec3, radius: f32) -> image::Pixel {
    if sphere_hit(r, center, radius) {
        return image::Pixel::new(255, 0, 0);
    }
    else {
        let t = 0.5 * (1.0 + r.direct().y);
        return (1.0 - t) * image::Pixel::new(25, 127, 255) + t * image::Pixel::new(255, 255, 255);
    }
}

#[allow(deprecated)]
fn main() {
    let aspect_ratio = 16.0/9.0;
    let image_height = 400;
    let image_width = (image_height as f32 * aspect_ratio) as u32;

    let view_height: f32 = 2.0;
    let view_width: f32 = view_height * aspect_ratio;
    let focal_length = 1.0;

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, view_height as f32, 0.0);
    let horizontal: Vec3 = Vec3::new(view_width as f32, 0.0, 0.0);
    let lower_left_corner = origin - vertical / 2.0 - horizontal / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let image_path = format!("{}/Desktop/image.png", env::home_dir().
    expect("Error: Couldn't find the home directory!").to_str().unwrap());

    println!("Image information:\n\
    Width: {}\n\
    Height: {}\n\
    File: {}", image_width, image_height, image_path);

    let mut image = image::Image::new(image_width, image_height, 3);

    for y in 0..image.height {
        for x in 0..image.width {
            let u = x as f32 / (image_width as f32 - 1.0);
            let v = y as f32 / (image_height as f32 - 1.0);

            let r: ray::Ray = ray::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let pixel = get_color(&r, Vec3::new(0.0, 0.0, -1.0), 0.5);
            image.set_pixel(x, y, pixel).unwrap();
        }
        print!("\rPercentage done: {:.2}%", y as f32/(image.height-1) as f32 * 100.0);
    }
    
    println!("\nDone!");
    image.write_to_file(image_path.as_str());
}
