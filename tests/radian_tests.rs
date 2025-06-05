#[test]
fn radians_by_name() {
    let angle_by_type = AngleInRadians::new(FRAC_PI_2_32);
    let angle_by_function = Angle::radians(FRAC_PI_2_32);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn radians_to_degrees32() {
    let angle = AngleInRadians::new(FRAC_PI_2_32);
    assert_ulps_eq!(angle.as_degrees().value, 90.0);
}

#[test]
fn radians_to_degrees64() {
    let angle = AngleInRadians::new(FRAC_PI_2_64);
    assert_ulps_eq!(angle.as_degrees().value, 90.0);
}

#[test]
fn radians_to_rotations32() {
    let angle = AngleInRadians::new(1.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 0.15915482);
}

#[test]
fn radians_to_rotations64() {
    let angle = AngleInRadians::new(1.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 0.15915482422145);
}

#[test]
fn radians_to_percentage32() {
    let angle = AngleInRadians::new(1.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 15.915482);
}

#[test]
fn radians_to_percentage64() {
    let angle = AngleInRadians::new(1.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 15.915482422145);
}

use approx::assert_ulps_eq;
use bangle::{Angle, AngleInRadians};
use core::{f32::consts::FRAC_PI_2 as FRAC_PI_2_32, f64::consts::FRAC_PI_2 as FRAC_PI_2_64};
