use core::f64;

use crate::{
    color::{Color, write_color},
    hitabble_list::HittableList,
    hittable::Hittable,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};
mod color;
mod hitabble_list;
mod hittable;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    match world.hit(r, 0.0, f64::MAX) {
        Some(rec) => {
            return 0.5
                * Vec3::new(
                    rec.normal.x() + 1.0,
                    rec.normal.y() + 1.0,
                    rec.normal.z() + 1.0,
                );
        }
        None => {
            let unit_direction = Vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}
fn main() {
    //Image
    let image_width: i64 = 200;
    let aspect_ratio = 16.0 / 9.0;
    //World

    let mut world = HittableList::new(2);
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    //calculate the image height and ensure that it's at 1
    let mut image_height = (image_width / aspect_ratio as i64) as i64;
    image_height = if image_height < 1 { 1 } else { image_height };

    //camera
    let focal_length = 1.0;
    let view_height = 2.0;
    let view_width = view_height * (image_width / image_height) as f64;
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
            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\rDone             \n",);
}
