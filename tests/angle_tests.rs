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

#[test]
fn test_converted_doubler_to_degrees() {
    let rotations: AngleInRotations = Angle::new(0.5);
    let degrees = generic_doubler_to_degrees(rotations);
    assert_ulps_eq!(degrees.value, 360.0);
}

fn generic_doubler_to_degrees(angle: impl ConvertAngle) -> AngleInDegrees {
    let degrees = angle.convert();
    degrees * 2.0
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
    let degrees = AngleInDegrees::from_other(radians);
    let rotations = AngleInRotations::from_other(degrees);
    let percentage = AngleInPercentage::from_other(rotations);
    let radians = AngleInRadians::from_other(percentage);
    assert_ulps_eq!(radians.value, 1.0);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, AngleUnit,
    ConvertAngle, Degrees, FromOther,
};
