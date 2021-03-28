extern crate nalgebra as na;
use super::drone::Drone;
use na::{Matrix3, Vector3};

pub struct Integrator {
	/// Gravitational acceleration in m/s^2
	gravity: Vector3<f32>,
}

struct Motor {
	pub position: Vector3<f32>,
	pub direction: Vector3<f32>,
	pub force: f32,
}

impl Integrator {
	pub fn new(gravity: f32) -> Integrator {
		return Integrator {
			gravity: Vector3::new(0.0, -gravity, 0.0),
		};
	}

	pub fn step(&self, body: &mut Drone, delta: f32) {
		// TODO move these so they are controllable
		let motors: [Motor; 4] = [
			Motor {
				position: Vector3::new(1.0, 0.0, 0.0),
				direction: Vector3::new(0.0, 1.0, 0.0),
				force: 2.5,
			},
			Motor {
				position: Vector3::new(0.0, 0.0, 1.0),
				direction: Vector3::new(0.0, 1.0, 0.0),
				force: 2.5,
			},
			Motor {
				position: Vector3::new(-1.0, 0.0, 0.0),
				direction: Vector3::new(0.0, 1.0, 0.0),
				force: 2.5,
			},
			Motor {
				position: Vector3::new(0.0, 0.0, -1.0),
				direction: Vector3::new(0.0, 1.0, 0.0),
				force: 2.5,
			},
		];

		// Angular acceleration caused by all motors together
		let angular_acceleration: Vector3<f32> = body.inverse_inertia
			* motors
				.iter()
				.map(|motor| motor.force * motor.position.cross(&motor.direction))
				.sum::<Vector3<f32>>();

		// Halfway the integration duration
		let half_delta = delta / 2.0;

		// Rotation at halfway time
		let half_rotation: Vector3<f32> = 0.5 * half_delta * half_delta * angular_acceleration
			+ half_delta * body.angular_velocity
			+ body.rotation;
		let half_theta: f32 = half_rotation.norm();
		let half_e: Vector3<f32> = half_rotation.normalize();

		// Acceleration at halfway time
		let half_acceleration: Vector3<f32>;
		if half_theta == 0.0 {
			half_acceleration = self.gravity
				+ (1.0 / body.mass)
					* motors
						.iter()
						.map(|motor| motor.force * motor.direction)
						.sum::<Vector3<f32>>();
		} else {
			half_acceleration = self.gravity
				+ (1.0 / body.mass)
					* motors
						.iter()
						.map(|motor| {
							half_theta.cos() * motor.force * motor.direction
								+ half_theta.sin() * half_e.cross(&(motor.force * motor.direction))
								+ (1.0 - half_theta.cos())
									* (half_e.dot(&(motor.force * motor.direction)) * half_e)
						})
						.sum::<Vector3<f32>>();
		}
		// Estimation of velocity at halfway time
		let half_velocity = body.velocity + half_delta * half_acceleration;

		// Estimate velocity and position after delta time by linearly interpolating halfway velocity
		body.velocity = body.velocity + delta * half_acceleration;
		body.position = body.position + delta * half_velocity;

		// Rotation and rotational velocity can be determined exactly
		body.rotation = body.rotation
			+ delta * body.angular_velocity
			+ 0.5 * delta * delta * angular_acceleration;
		body.angular_velocity = body.angular_velocity + delta * angular_acceleration;

		// Modulo 2pi rotation vector length
		if body.rotation.norm() != 0.0 {
			body.rotation =
				(body.rotation.norm() % (std::f32::consts::PI * 2.0)) * body.rotation.normalize();
		}
	}
}
