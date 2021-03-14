extern crate nalgebra as na;
use super::rigidbody::Rigidbody;
use na::Vector3;

pub struct Integrator {
	/// Gravitational acceleration in m/s^2
	gravity: Vector3<f32>,
}

impl Integrator {
	pub fn new(gravity: f32) -> Integrator {
		return Integrator {
			gravity: Vector3::new(0.0, -gravity, 0.0),
		};
	}

	pub fn step(&self, body: &mut Rigidbody, delta: f32) {
		body.position = body.position + delta * body.velocity + 0.5 * self.gravity * delta * delta;
		body.velocity = body.velocity + delta * self.gravity;
	}
}
