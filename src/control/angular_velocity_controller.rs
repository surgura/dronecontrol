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
		let mut desireddivby4 = Vector4::new(
			-error[0] - error[1] - error[2],
			error[0] - error[1] + error[2],
			-error[0] + error[1] + error[2],
			error[0] + error[1] - error[2],
		);

		// Shift all pwms so lowest is 0
		let min: f32 = *desireddivby4
			.iter()
			.min_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap();
		for element in desireddivby4.iter_mut() {
			*element -= min;
		}

		// TODO pwm ratio can go > 1

		return (
			desireddivby4[0],
			desireddivby4[1],
			desireddivby4[2],
			desireddivby4[3],
		);
	}
}
