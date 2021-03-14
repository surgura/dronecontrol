extern crate nalgebra as na;
mod physics;
use physics::integrator::Integrator;
use physics::rigidbody::Rigidbody;

fn main() {
    let mut drone = Rigidbody::new(1.0);

    let integrator = Integrator::new(9.81);

    println!("{:?}", drone);
    for _ in 0..10 {
        integrator.step(&mut drone, 0.01);
        println!("{:?}", drone);
    }
}
