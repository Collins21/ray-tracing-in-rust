use crate::{
    color::{Color, write_color},
    ray::Ray,
    vec3::{Point3, Vec3},
};
mod color;
mod ray;
mod utility;
mod vec3;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color { e: [1.0, 1.0, 1.0] } + a * Color { e: [0.5, 0.7, 1.0] }
}
fn main() {
    //Image
    let image_width: i64 = 400;
    let aspect_ratio = 16.0 / 9.0;
    //calculate the image height and ensure that it's at 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i64;
    image_height = if image_height < 1 { 1 } else { image_height };

    //camera
    let focal_length = 1.0;
    let view_height = 2.0;
    let view_width = view_height as f64 * (image_width / image_height) as f64;
    let camera_center = Point3 { e: [0.0, 0.0, 0.0] };

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let viewport_u = Vec3 {
        e: [view_width, 0.0, 0.0],
    };
    let viewport_v = Vec3 {
        e: [0.0, -view_height, 0.0],
    };

    //calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3 {
            e: [0.0, 0.0, focal_length],
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    //render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                dir: ray_direction,
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprint!("\rDone             \n",);
}
