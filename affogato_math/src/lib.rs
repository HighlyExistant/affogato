mod sets;
mod animation;
mod util;
pub mod matrix;
pub mod algebra;
pub mod spaces;
pub mod vector;
pub mod mappings;
pub mod geometry;
pub mod transformations;
pub use sets::*;
pub use animation::*;
pub use transformations::*;
pub use util::*;

mod test {
    use crate::{geometry::Circle, vector::FVec2, Zero};

    fn test_circl() {
        let unit = Circle::new(FVec2::ZERO, 1.0);
    }
}