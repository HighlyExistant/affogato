use affogato::{algebra::ComplexNumber, geometry::{FSegment2D, LinearSegment2D, Segment, Segment2D}, linear::FVec2, polynomial, sets::CartesianProduct, spatial::morton::MortonU64};

fn main() {
    let morton = MortonU64::encode_xy(5, 12);
    println!("{morton:b}");
    println!("{:?}", morton.decode_xy());
}