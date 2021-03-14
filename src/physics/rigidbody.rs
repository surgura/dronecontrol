extern crate nalgebra as na;
use na::Vector3;

#[derive(Debug)]
pub struct Rigidbody {
	pub position: Vector3<f32>,
	pub velocity: Vector3<f32>,
	pub mass: f32,
}

impl Rigidbody {
	pub fn new(mass: f32) -> Rigidbody {
		return Rigidbody {
			position: Vector3::new(0.0, 0.0, 0.0),
			velocity: Vector3::new(0.0, 0.0, 0.0),
			mass: mass,
		};
	}
}
