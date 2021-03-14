use super::rigidbody::Rigidbody;
extern crate nalgebra as na;
use na::Vector3;

pub struct Force<'a> {
	body: &'a mut Rigidbody,
	offset: Vector3<f32>,
}

impl<'a> Force<'a> {
	fn new(body: &'a mut Rigidbody, offset: Vector3<f32>) -> Force<'a> {
		return Force {
			body: body,
			offset: offset,
		};
	}
}
