use na::Vector3;

pub struct AngularVelocityController {}

impl AngularVelocityController {
	pub fn new() -> AngularVelocityController {
		return AngularVelocityController {};
	}

	pub fn control(&mut self, measured_angular_velocity: &Vector3<f32>) -> (f32, f32, f32, f32) {
		return (2.0, 2.0, 2.0, 2.0);
	}
}
