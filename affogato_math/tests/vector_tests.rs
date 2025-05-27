use affogato_math::vector::{PolarCoordinate, Vector2};
#[test]
pub fn test_coordinates() {
    let polar = PolarCoordinate::new(1.0, 45.0f32.to_radians());
    assert!(Vector2::from(polar).epsilon_eq(Vector2::new(2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0), f32::EPSILON));

    let polar = PolarCoordinate::new(1.0, 90.0f32.to_radians());
    assert!(Vector2::from(polar).epsilon_eq(Vector2::new(0.0f32, 1.0f32), f32::EPSILON), "{:?}", Vector2::from(polar));
    
    let polar = PolarCoordinate::new(1.0, 180.0f32.to_radians());
    assert!(Vector2::from(polar).epsilon_eq(Vector2::new(-1.0f32, 0.0f32), f32::EPSILON), "{:?}", Vector2::from(polar));
}