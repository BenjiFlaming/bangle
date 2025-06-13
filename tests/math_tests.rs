#[test]
fn equality() {
    let first = Angle::degrees(90.0);
    let second = Angle::degrees(90.0);
    assert_eq!(first, second);
}

#[test]
fn compare() {
    let first = Angle::degrees(90.0);
    let second = Angle::degrees(180.0);
    assert!(first < second);
    assert!(second > first);
}

#[test]
fn negation32() {
    let degrees = Angle::degrees(90.0f32);
    let negative = -degrees;
    assert_ulps_eq!(negative.value, -90.0);
}

#[test]
fn negation64() {
    let degrees = Angle::degrees(90.0f64);
    let negative = -degrees;
    assert_ulps_eq!(negative.value, -90.0);
}

#[test]
fn radians_plus_degrees() {
    let radians = Angle::radians(FRAC_PI_2);
    let degrees = Angle::degrees(90.0);
    let result = radians + degrees;
    assert_ulps_eq!(result.value, PI);
}

#[test]
fn radians_plus_degrees_in_place() {
    let mut radians = Angle::radians(FRAC_PI_2);
    radians += Angle::degrees(90.0);
    assert_ulps_eq!(radians.value, PI);
}

#[test]
fn radians_minus_degrees() {
    let radians = Angle::radians(PI);
    let degrees = Angle::degrees(90.0);
    let result = radians - degrees;
    assert_ulps_eq!(result.value, FRAC_PI_2);
}

#[test]
fn radians_minus_degrees_in_place() {
    let mut radians = Angle::radians(PI);
    radians -= Angle::degrees(90.0);
    assert_ulps_eq!(radians.value, FRAC_PI_2);
}

#[test]
fn radians_multiplied() {
    let radians = Angle::radians(FRAC_PI_2);
    let result = radians * 2.0;
    assert_ulps_eq!(result.value, PI);
}

#[test]
fn radians_multiplied_in_place() {
    let mut radians = Angle::radians(FRAC_PI_2);
    radians *= 2.0;
    assert_ulps_eq!(radians.value, PI);
}

#[test]
fn radians_divided() {
    let radians = Angle::radians(PI);
    let result = radians / 2.0;
    assert_ulps_eq!(result.value, FRAC_PI_2);
}

#[test]
fn radians_divided_in_place() {
    let mut radians = Angle::radians(PI);
    radians /= 2.0;
    assert_ulps_eq!(radians.value, FRAC_PI_2);
}

use approx::assert_ulps_eq;
use bangle::Angle;
use core::f32::consts::{FRAC_PI_2, PI};
