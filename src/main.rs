use affogato::linear::{FVec2, Vector};

mod linear;
mod data;
mod definitions;
mod complex;
mod smoothing;
mod shapes;
use data::iterator::ExtendIterator;
fn main() {
    // let arr = vec![
    //     FVec2::new(9.0, 1.0),
    //     FVec2::new(8.0, 6.0),
    //     FVec2::new(6.0, 7.0),
    //     FVec2::new(8.0, 3.0),
    //     FVec2::new(1.0, 8.0),
    //     FVec2::new(3.0, 9.0),
    // ];
    // let len = arr.len() as isize;
    // // let idx: Vec<u32> = quicksort::quicksort_get_indices(&arr, 0, len-1);
    // let idx = FVec2::sort_y(arr.as_slice(), 0, len-1);
    // for i in idx {
    //     println!("{:?}\t{}", arr[i as usize], i);
    // }
    let slice = [0, 1, 1, 2, 3, 3, 3, 3, 4, 5, 6, 6, 1, 2, 3, 132, 321, 3, 31, 1, 1];
    let vec = slice.iter().unique_copy().map(|v|{*v}).collect::<Vec<_>>();
    
    println!("{:#?}", vec);
    println!("{:#?}", vec.len());
}
