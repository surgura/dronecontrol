extern crate nalgebra as na;
mod control;
mod simulation;
use control::angular_velocity_controller::AngularVelocityController;
use na::{Matrix3, Vector3};
use plotters::prelude::*;
use simulation::drone::{Drone, Motor};
use simulation::integrator::Integrator;

fn main() {
    let sizex: f32 = 0.5;
    let sizey: f32 = 0.5;
    let sizez: f32 = 0.01;
    let mass: f32 = 1.0;
    // TODO check if correct
    let inertia = Matrix3::new(
        1.0 / 12.0 * mass * (sizey * sizey + sizez * sizez),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (sizex * sizex + sizez * sizez),
        0.0,
        0.0,
        0.0,
        1.0 / 12.0 * mass * (sizex * sizex + sizey * sizey),
    );
    let mut inverse_inertia: Matrix3<f32> = Matrix3::zeros();
    na::linalg::try_invert_to(inertia, &mut inverse_inertia);
    let mut drone = Drone::new(
        mass,
        inverse_inertia,
        Motor::new(
            1.0_f32 * Vector3::new(1.0, -1.0, 0.0).normalize(),
            1.0,
            -1.0,
        ),
        Motor::new(1.0_f32 * Vector3::new(1.0, 1.0, 0.0).normalize(), 1.0, 1.0),
        Motor::new(
            1.0_f32 * Vector3::new(-1.0, -1.0, 0.0).normalize(),
            1.0,
            1.0,
        ),
        Motor::new(
            1.0_f32 * Vector3::new(-1.0, 1.0, 0.0).normalize(),
            1.0,
            -1.0,
        ),
    );

    let integrator = Integrator::new(9.81);
    let mut angular_velocity_controller = AngularVelocityController::new();
    let mut measured_angular_velocity: Vector3<f32> = Vector3::zeros();
    let target_angular_velocity: Vector3<f32> = Vector3::new(2.0, 2.0, 2.0);
    let max_pwm = 1.0;

    //println!("{:?}", drone);
    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut rotations: Vec<Vector3<f32>> = Vec::new();
    let mut angular_velocities: Vec<Vector3<f32>> = Vec::new();
    positions.push(drone.position);
    rotations.push(drone.rotation);
    for _ in 0..1000 {
        let (motor1rpm, motor2rpm, motor3rpm, motor4rpm) = angular_velocity_controller
            .control(&target_angular_velocity, &measured_angular_velocity);
        measured_angular_velocity = integrator.step(
            &mut drone,
            0.01,
            motor1rpm * max_pwm,
            motor2rpm * max_pwm,
            motor3rpm * max_pwm,
            motor4rpm * max_pwm,
        );
        positions.push(drone.position);
        rotations.push(drone.rotation);
        angular_velocities.push(drone.angular_velocity);
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
            angular_velocities
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[0] as f64)),
            &RED,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            angular_velocities
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[1] as f64)),
            &BLUE,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            angular_velocities
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64 * 0.01, v[2] as f64)),
            &GREEN,
        ))
        .unwrap();
    /*
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
    */
}
