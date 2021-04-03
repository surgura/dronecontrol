extern crate nalgebra as na;
use super::drone::{Drone, Motor};
use arrayvec::ArrayVec;
use itertools::izip;
use na::Vector3;

pub struct Integrator {
	/// Gravitational acceleration in m/s^2
	gravity: Vector3<f32>,
}

#[derive(Debug)]
struct Effect {
	position: Vector3<f32>,
	lift: Vector3<f32>,
	torque: Vector3<f32>,
}

impl Integrator {
	pub fn new(gravity: f32) -> Integrator {
		return Integrator {
			gravity: Vector3::new(0.0, -gravity, 0.0),
		};
	}

	pub fn step(
		&self,
		drone: &mut Drone,
		delta: f32,
		motor1rpm: f32,
		motor2rpm: f32,
		motor3rpm: f32,
		motor4rpm: f32,
	) {
		let motors: [&Motor; 4] = [&drone.motor1, &drone.motor2, &drone.motor3, &drone.motor4];
		let rpms: [f32; 4] = [motor1rpm, motor2rpm, motor3rpm, motor4rpm];
		let motor_direction = Vector3::new(0.0, 1.0, 0.0).normalize();
		let effects: ArrayVec<Effect, 4> = izip!(&motors, &rpms)
			.map(|(motor, rpm)| Effect {
				position: motor.position,
				lift: motor.lift * rpm * motor_direction,
				torque: motor.torque * rpm * motor_direction,
			})
			.collect();

		// Angular acceleration caused by all motors together
		let angular_acceleration: Vector3<f32> = drone.inverse_inertia
			* effects
				.iter()
				.map(|effect| effect.position.cross(&effect.lift) + effect.torque)
				.sum::<Vector3<f32>>();

		// Halfway the integration duration
		let half_delta = delta / 2.0;

		// Rotation at halfway time
		let half_rotation: Vector3<f32> = 0.5 * half_delta * half_delta * angular_acceleration
			+ half_delta * drone.angular_velocity
			+ drone.rotation;
		let half_theta: f32 = half_rotation.norm();
		let half_e: Vector3<f32> = half_rotation.normalize();

		// Acceleration at halfway time
		let half_acceleration: Vector3<f32>;
		if half_theta == 0.0 {
			half_acceleration = self.gravity
				+ (1.0 / drone.mass)
					* effects
						.iter()
						.map(|effect| effect.lift)
						.sum::<Vector3<f32>>();
		} else {
			half_acceleration = self.gravity
				+ (1.0 / drone.mass)
					* effects
						.iter()
						.map(|effect| {
							half_theta.cos() * effect.lift
								+ half_theta.sin() * half_e.cross(&(effect.lift))
								+ (1.0 - half_theta.cos()) * (half_e.dot(&(effect.lift)) * half_e)
						})
						.sum::<Vector3<f32>>();
		}
		// Estimation of velocity at halfway time
		let half_velocity = drone.velocity + half_delta * half_acceleration;

		// Estimate velocity and position after delta time by linearly interpolating halfway velocity
		drone.velocity = drone.velocity + delta * half_acceleration;
		drone.position = drone.position + delta * half_velocity;

		// Rotation and rotational velocity can be determined exactly
		drone.rotation = drone.rotation
			+ delta * drone.angular_velocity
			+ 0.5 * delta * delta * angular_acceleration;
		drone.angular_velocity = drone.angular_velocity + delta * angular_acceleration;

		// Modulo 2pi rotation vector length
		if drone.rotation.norm() != 0.0 {
			drone.rotation =
				(drone.rotation.norm() % (std::f32::consts::PI * 2.0)) * drone.rotation.normalize();
		}
	}
}
