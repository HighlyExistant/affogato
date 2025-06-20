use affogato::{vector::{FMat2, FVec2}};

fn main() {
    let vec0 = FVec2::new(1.0, 0.0);
    let rot0 = FMat2::new(
        0.0, 1.0, 
        -1.0, 0.0
    );
    println!("{}", rot0*vec0);
}