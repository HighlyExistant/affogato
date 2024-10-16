use affogato::{algebra::ComplexNumber, geometry::{LinearSegment2D, Segment}, linear::FVec2};

fn main() {
    let a = LinearSegment2D::new(FVec2::new(0.0, 0.0), FVec2::new(1.0, 1.0));
    let x = a.split_in_thirds();
}