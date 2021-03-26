extern crate nalgebra as na;
use super::drone::Drone;
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

	pub fn step(&self, body: &mut Drone, delta: f32) {
		// Apply gravity
		//body.position = body.position + delta * body.velocity + 0.5 * self.gravity * delta * delta;
		//body.velocity = body.velocity + delta * self.gravity;

		// Apply motors
		let mut inverse_inertia: Matrix3<f32> = Matrix3::zeros();
		na::linalg::try_invert_to(body.inertia, &mut inverse_inertia);

		//let force_pos: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
		let force_pos: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
		let force: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

		let angular_acceleration = inverse_inertia * force_pos.cross(&force);

		// Estimate position and velocity
		let half_delta = delta / 2.0;
		let half_rotation: Vector3<f32> = 0.5 * half_delta * half_delta * angular_acceleration
			+ half_delta * body.angular_velocity
			+ body.rotation;
		let half_theta: f32 = half_rotation.norm();
		let half_e: Vector3<f32> = half_rotation.normalize();
		let half_acceleration: Vector3<f32>;
		if half_theta == 0.0 {
			half_acceleration = (1.0 * body.mass) * force; // + self.gravity;
		} else {
			half_acceleration = (1.0 * body.mass)
				* (half_theta.cos() * force
					+ half_theta.sin() * half_e.cross(&force)
					+ (1.0 - half_theta.cos()) * (half_e.dot(&force)) * half_e);
			//+ self.gravity;
		}
		let half_velocity = body.velocity + half_delta * half_acceleration;

		body.velocity = body.velocity + delta * half_acceleration;
		body.position = body.position + delta * half_velocity;

		// Rotation and rotational velocity can be determined exactly
		body.rotation = body.rotation
			+ delta * body.angular_velocity
			+ 0.5 * delta * delta * angular_acceleration;
		body.angular_velocity = body.angular_velocity + delta * angular_acceleration;

		body.rotation =
			(body.rotation.norm() % (std::f32::consts::PI * 2.0)) * body.rotation.normalize();
	}
}
