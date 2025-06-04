#[test]
fn percentage_to_radians32() {
    let angle = AngleInPercentage::new(100.0f32);
    assert_ulps_eq!(angle.as_radians().value, std::f32::consts::TAU);
    let angle = AngleInPercentage::new(-200.0f32);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f32::consts::TAU);
}

#[test]
fn percentage_to_radians64() {
    let angle = AngleInPercentage::new(100.0f64);
    assert_ulps_eq!(angle.as_radians().value, std::f64::consts::TAU);
    let angle = AngleInPercentage::new(-200.0f64);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f64::consts::TAU);
}

#[test]
fn percentage_to_degrees32() {
    let angle = AngleInPercentage::new(100.0f32);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = AngleInPercentage::new(-200.0f32);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn percentage_to_degrees64() {
    let angle = AngleInPercentage::new(100.0f64);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = AngleInPercentage::new(-200.0f64);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn percentage_to_rotations32() {
    let angle = AngleInPercentage::new(100.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 1.0);
    let angle = AngleInPercentage::new(-200.0f32);
    assert_ulps_eq!(angle.as_rotations().value, -2.0);
}

#[test]
fn percentage_to_rotations64() {
    let angle = AngleInPercentage::new(100.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 1.0);
    let angle = AngleInPercentage::new(-200.0f64);
    assert_ulps_eq!(angle.as_rotations().value, -2.0);
}

use approx::assert_ulps_eq;
use bangle::AngleInPercentage;
