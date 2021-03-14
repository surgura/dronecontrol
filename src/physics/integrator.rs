use super::rigidbody::Rigidbody;

pub struct Integrator {
	/// Gravitational acceleration in m/s^2
	gravity: f32,
}

impl Integrator {
	pub fn new(gravity: f32) -> Integrator {
		return Integrator { gravity: gravity };
	}

	pub fn step(&self, body: &mut Rigidbody, delta: f32) {
		body.velocity.z -= self.gravity * delta;
		body.position += body.velocity * delta;
	}
}
