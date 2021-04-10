extern crate nalgebra as na;
mod control;
mod simulation;
use control::angular_velocity_controller::AngularVelocityController;
use na::{Matrix3, Vector3};
use plotters::prelude::*;
use simulation::drone::{Drone, Motor};
use simulation::integrator::Integrator;

fn main() {
    let width: f32 = 0.5;
    let height: f32 = 0.01;
    let depth: f32 = 0.5;
    let mass: f32 = 1.0;
    let inertia = Matrix3::new(
        1.0 / 12.0 * mass * (height * height + depth * depth),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (width * width + depth * depth),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (height * height + width * width),
    );
    let mut inverse_inertia: Matrix3<f32> = Matrix3::zeros();
    na::linalg::try_invert_to(inertia, &mut inverse_inertia);
    let mut drone = Drone::new(
        mass,
        inverse_inertia,
        Motor::new(
            1.0_f32 * Vector3::new(1.0, 0.0, -1.0).normalize(),
            1.0,
            -1.0,
        ),
        Motor::new(1.0_f32 * Vector3::new(1.0, 0.0, 1.0).normalize(), 1.0, 1.0),
        Motor::new(
            1.0_f32 * Vector3::new(-1.0, 0.0, -1.0).normalize(),
            1.0,
            1.0,
        ),
        Motor::new(
            1.0_f32 * Vector3::new(-1.0, 0.0, 1.0).normalize(),
            1.0,
            -1.0,
        ),
    );

    let integrator = Integrator::new(9.81);
    let mut angular_velocity_controller = AngularVelocityController::new();
    let mut measured_angular_velocity: Vector3<f32> = Vector3::zeros();

    //println!("{:?}", drone);
    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut rotations: Vec<Vector3<f32>> = Vec::new();
    positions.push(drone.position);
    rotations.push(drone.rotation);
    for _ in 0..1000 {
        let (motor1rpm, motor2rpm, motor3rpm, motor4rpm) =
            angular_velocity_controller.control(&measured_angular_velocity);
        measured_angular_velocity =
            integrator.step(&mut drone, 0.01, motor1rpm, motor2rpm, motor3rpm, motor4rpm);
        positions.push(drone.position);
        rotations.push(drone.rotation);
    }

    let root_drawing_area = BitMapBackend::new("images/plot.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..3.14, -8.0..8.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            positions
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[0] as f64)),
            &RED,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            positions
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[1] as f64)),
            &BLUE,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            positions
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[2] as f64)),
            &GREEN,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            rotations
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[0] as f64)),
            &CYAN,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            rotations
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[1] as f64)),
            &YELLOW,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            rotations
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[2] as f64)),
            &BLACK,
        ))
        .unwrap();
}
