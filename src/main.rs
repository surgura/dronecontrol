extern crate nalgebra as na;
mod physics;
use na::Matrix3;
use physics::integrator::Integrator;
use physics::rigidbody::Rigidbody;

fn main() {
    // width = depth = 0.5
    // height = .01
    let width: f32 = 0.5;
    let height: f32 = 0.01;
    let depth: f32 = 0.5;
    let mass: f32 = 1.0;
    let inertia = Matrix3::new(
        1.0 / 12.0 * mass * (height * height + depth * depth),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (width * width + depth * depth),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (height * height + width * width),
    );
    let mut drone = Rigidbody::new(mass, inertia);

    let integrator = Integrator::new(9.81);

    //println!("{:?}", drone);
    for _ in 0..10 {
        integrator.step(&mut drone, 0.01);
        //println!("{:?} {:?}", drone.rotation, drone.angular_velocity);
        println!("{:?}", drone.rotation);
    }
}
