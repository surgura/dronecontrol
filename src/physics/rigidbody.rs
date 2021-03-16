extern crate nalgebra as na;
use na::{Matrix3, Vector3};

#[derive(Debug)]
pub struct Rigidbody {
	pub mass: f32,
	pub inertia: Matrix3<f32>,
	pub position: Vector3<f32>,
	pub velocity: Vector3<f32>,
	pub rotation: Vector3<f32>,
	pub angular_velocity: Vector3<f32>,
}

impl Rigidbody {
	pub fn new(mass: f32, inertia: Matrix3<f32>) -> Rigidbody {
		return Rigidbody {
			mass: mass,
			inertia: inertia,
			position: Vector3::new(0.0, 0.0, 0.0),
			velocity: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.0, 0.0, 0.0),
			angular_velocity: Vector3::new(0.0, 0.0, 0.0),
		};
	}
}
