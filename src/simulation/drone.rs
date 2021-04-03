extern crate nalgebra as na;
use na::{Matrix3, Vector3};

#[derive(Debug)]
pub struct Drone {
	pub mass: f32,
	pub inverse_inertia: Matrix3<f32>,
	pub position: Vector3<f32>,
	pub velocity: Vector3<f32>,
	pub rotation: Vector3<f32>,
	pub angular_velocity: Vector3<f32>,
	pub motor1: Motor,
	pub motor2: Motor,
	pub motor3: Motor,
	pub motor4: Motor,
}

impl Drone {
	pub fn new(
		mass: f32,
		inverse_inertia: Matrix3<f32>,
		motor1: Motor,
		motor2: Motor,
		motor3: Motor,
		motor4: Motor,
	) -> Drone {
		return Drone {
			mass: mass,
			inverse_inertia: inverse_inertia,
			position: Vector3::new(0.0, 0.0, 0.0),
			velocity: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.0, 0.0, 0.0),
			angular_velocity: Vector3::new(0.0, 0.0, 0.0),
			motor1: motor1,
			motor2: motor2,
			motor3: motor3,
			motor4: motor4,
		};
	}
}

#[derive(Debug)]
pub struct Motor {
	pub position: Vector3<f32>,
	pub lift: f32,   // Lift force = lift * rpm^2
	pub torque: f32, // Torque = torque * rpm^2
}

impl Motor {
	pub fn new(position: Vector3<f32>, lift: f32, torque: f32) -> Motor {
		return Motor {
			position: position,
			lift: lift,
			torque: torque,
		};
	}
}
