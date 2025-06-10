#[test]
fn basic_sanity() {
    let angle: Angle<Degrees> = Angle::new(90.0);
    assert_eq!(angle.value, 90.0);
}

#[test]
fn test_generic_to_radians() {
    let rotations: AngleInRotations = Angle::new(0.5);
    let degrees = generic_to_degrees(rotations);
    assert_ulps_eq!(degrees.value, 180.0);
}

fn generic_to_degrees<U>(angle: Angle<U>) -> AngleInDegrees
where
    U: AngleUnit,
{
    angle.as_degrees()
}

#[test]
fn default_value() {
    let angle = AngleInRadians::<f32>::default();
    assert_eq!(angle.value, 0.0);
}

#[test]
fn conversion_from_owned() {
    let radians = AngleInRadians::new(1.0);
    let degrees = AngleInDegrees::from(radians);
    let rotations = AngleInRotations::from(degrees);
    let percentage = AngleInPercentage::from(rotations);
    let radians = AngleInRadians::from(percentage);
    assert_ulps_eq!(radians.value, 1.0);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, AngleUnit, Degrees,
};
