use rt_one_weekend::{
    camera::Camera,
    grid::Grid,
    hittable::HittableList,
    renderer::{colour_at_ray, Renderer},
    sphere::Sphere,
    vec3::Point3,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const HEIGHT: usize = IMAGE_HEIGHT as usize;
    const WIDTH: usize = IMAGE_WIDTH as usize;

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, IMAGE_HEIGHT);

    let pixels: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();

    let renderer = Renderer {
        camera: cam,
        filename: "output/output.png".into(),
    };

    renderer.render_img(world, colour_at_ray, pixels);
}
