extern crate nalgebra as na;
use super::rigidbody::Rigidbody;
use na::{Matrix3, Vector3};

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

		let mut inverse_inertia: Matrix3<f32> = Matrix3::zeros();
		na::linalg::try_invert_to(body.inertia, &mut inverse_inertia);

		let force_pos: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
		let force: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

		//println!("{:?}", inverse_inertia);
		body.rotation = body.rotation
			+ delta * body.angular_velocity
			+ 0.5 * delta * delta * inverse_inertia * force_pos.cross(&force);
		body.angular_velocity =
			body.angular_velocity + delta * inverse_inertia * force_pos.cross(&force);
	}
}
