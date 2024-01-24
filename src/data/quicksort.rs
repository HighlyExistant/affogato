// obtained from: https://gist.github.com/lispandfound/96b9065bf240f94a0c0f#file-quicksort-rs
// modified using https://www.hackertouch.com/quick-sort-in-rust.html
// A generic quicksort algorithm implementation in rust, modified to suit my needs

use crate::{definitions::Number, linear::{Vector2, Vector3, Vector4}};

pub fn quicksort_get_indices<T: Eq + PartialEq + Clone + PartialOrd>(arr: &[T], low: isize, high: isize) -> Vec<u32> {
    let mut indices = Vec::from_iter(0..arr.len() as u32);
    quicksort_core(arr, &mut indices, low, high);
    indices
}
pub(crate) fn quicksort_core<T: Eq + PartialEq + Clone + PartialOrd>(arr: &[T], indices: &mut Vec<u32>, low: isize, high: isize) {

    if low < high {
        let p = partition(
            arr, 
            indices, 
            low, 
            high, 
            &|a, b| {a <= b}, 
            &|a, b| {a < b}, 
            &|a, b| {a > b}
        );
        quicksort_core(arr, indices, low, p - 1);
        quicksort_core(arr, indices, p + 1, high);
    }
}
pub(crate) fn partition<T, F, Min, Max>(arr: &[T], indices: &mut Vec<u32>, low: isize, high: isize, f: &F, min: &Min, max: &Max) -> isize
    where T: Clone,
        F: Fn(&T, &T) -> bool,
        Min: Fn(&T, &T) -> bool,
        Max: Fn(&T, &T) -> bool, {
    // let pivot = match arr.get(indices[high as usize] as usize) {
    //     Some(v) => {v.clone()}
    //     _ => {panic!("Array index {:?} out of bounds", high)}
    // };
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    loop {
        store_index += 1;
        while min(&arr[indices[store_index as usize] as usize], &arr[indices[pivot] as usize]) {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && max(&arr[indices[last_index as usize] as usize], &arr[indices[pivot] as usize]) {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            indices.swap(store_index as usize, last_index as usize);
        }
    }
    indices.swap(store_index as usize, pivot as usize);
    store_index
}
impl<T: Sized + Number> Vector2<T> {
    pub fn sort_x(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_x(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_y(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_y(arr, &mut indices, low, high);
        indices
    }
    fn sort_core_x(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.x <= b.x},
                &|a: &Vector2<T>, b: &Vector2<T>| {a.x < b.x},
                &|a: &Vector2<T>, b: &Vector2<T>| {a.x > b.x},
            );
            Self::sort_core_x(data, indices, low, p - 1);
            Self::sort_core_x(data, indices, p + 1, high);
        }
    }
    fn sort_core_y(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.y <= b.y},
                &|a: &Vector2<T>, b: &Vector2<T>| {a.y < b.y},
                &|a: &Vector2<T>, b: &Vector2<T>| {a.y > b.y},
            );
            Self::sort_core_y(data, indices, low, p - 1);
            Self::sort_core_y(data, indices, p + 1, high);
        }
    }
}


impl<T: Sized + Number> Vector3<T> {
    pub fn sort_x(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_x(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_y(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_y(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_z(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_z(arr, &mut indices, low, high);
        indices
    }
    fn sort_core_x(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.x <= b.x},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.x < b.x},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.x > b.x},
            );
            Self::sort_core_x(data, indices, low, p - 1);
            Self::sort_core_x(data, indices, p + 1, high);
        }
    }
    fn sort_core_y(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.y <= b.y},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.y < b.y},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.y > b.y},
            );
            Self::sort_core_y(data, indices, low, p - 1);
            Self::sort_core_y(data, indices, p + 1, high);
        }
    }
    fn sort_core_z(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.z <= b.z},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.y < b.z},
                &|a: &Vector3<T>, b: &Vector3<T>| {a.z > b.z},
            );
            Self::sort_core_z(data, indices, low, p - 1);
            Self::sort_core_z(data, indices, p + 1, high);
        }
    }
}
impl<T: Sized + Number> Vector4<T> {
    pub fn sort_x(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_x(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_y(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_y(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_z(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_z(arr, &mut indices, low, high);
        indices
    }
    pub fn sort_w(arr: &[Self], low: isize, high: isize) -> Vec<u32> {
        let mut indices = Vec::from_iter(0..arr.len() as u32);
        Self::sort_core_w(arr, &mut indices, low, high);
        indices
    }
    fn sort_core_x(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.x <= b.x},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.x < b.x},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.x > b.x},
            );
            Self::sort_core_x(data, indices, low, p - 1);
            Self::sort_core_x(data, indices, p + 1, high);
        }
    }
    fn sort_core_y(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.y <= b.y},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.y < b.y},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.y > b.y},
            );
            Self::sort_core_y(data, indices, low, p - 1);
            Self::sort_core_y(data, indices, p + 1, high);
        }
    }
    fn sort_core_z(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.z <= b.z},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.y < b.z},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.z > b.z},
            );
            Self::sort_core_z(data, indices, low, p - 1);
            Self::sort_core_z(data, indices, p + 1, high);
        }
    }
    fn sort_core_w(data: &[Self], indices: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {

            let p = partition(
                data, 
                indices, 
                low, high, 
                &|a, b| {a.w <= b.w},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.w < b.w},
                &|a: &Vector4<T>, b: &Vector4<T>| {a.w > b.w},
            );
            Self::sort_core_w(data, indices, low, p - 1);
            Self::sort_core_w(data, indices, p + 1, high);
        }
    }
}