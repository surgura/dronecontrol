use na::{Vector3, Vector4};

pub struct AngularVelocityController {}

impl AngularVelocityController {
	pub fn new() -> AngularVelocityController {
		return AngularVelocityController {};
	}

	pub fn control(
		&mut self,
		target_angular_velocity: &Vector3<f32>,
		measured_angular_velocity: &Vector3<f32>,
	) -> (f32, f32, f32, f32) {
		let error = target_angular_velocity - measured_angular_velocity;

		println!("error: {:?}", error);

		let mut desiredx4 = Vector4::new(
			-error[0] - error[1] - error[2],
			error[0] - error[1] + error[2],
			-error[0] + error[1] + error[2],
			error[0] + error[1] - error[2],
		) * 0.05_f32;

		// Shift all pwms so lowest is 0
		let min: f32 = *desiredx4
			.iter()
			.min_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap();
		for element in desiredx4.iter_mut() {
			*element -= min;
		}
		let desired = 0.25 * desiredx4;

		// TODO pwm ratio can go > 1

		return (desired[0], desired[1], desired[2], desired[3]);
	}
}
