use std::sync::atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};

use super::{Vector2, Vector3, Vector4};

macro_rules! impl_vector_fetch {
    ($non_atomic:ident, $vector_type:ty, $($element:tt),+) => {
        pub fn fetch_add(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_add(val.$element, ordering)),+)
        }
        pub fn fetch_max(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_max(val.$element, ordering)),+)
        }
        pub fn fetch_min(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_min(val.$element, ordering)),+)
        }
        pub fn fetch_nand(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_nand(val.$element, ordering)),+)
        }
        pub fn fetch_or(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_or(val.$element, ordering)),+)
        }
        pub fn fetch_sub(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_sub(val.$element, ordering)),+)
        }
        pub fn fetch_xor(&self, val: $non_atomic<$vector_type>, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.fetch_xor(val.$element, ordering)),+)
        }
        pub fn load(&self, ordering: Ordering) -> $non_atomic<$vector_type> {
            $non_atomic::<$vector_type>::new($(self.$element.load(ordering)),+)
        }
        pub fn store(&self, val: $non_atomic<$vector_type>, ordering: Ordering) {
            $(
                self.$element.store(val.$element, ordering);
            )+
        }
    };
}
macro_rules! impl_vector_atomic {
    ($atomic:ident, $($element:tt),+) => {
        impl $atomic<AtomicI8> {
            impl_vector_fetch!($atomic, i8, $($element),+);
        }
        impl $atomic<AtomicI16> {
            impl_vector_fetch!($atomic, i16, $($element),+);
        }
        impl $atomic<AtomicI32> {
            impl_vector_fetch!($atomic, i32, $($element),+);
        }
        impl $atomic<AtomicI64> {
            impl_vector_fetch!($atomic, i64, $($element),+);
        }
        impl $atomic<AtomicU8> {
            impl_vector_fetch!($atomic, u8, $($element),+);
        }
        impl $atomic<AtomicU16> {
            impl_vector_fetch!($atomic, u16, $($element),+);
        }
        impl $atomic<AtomicU32> {
            impl_vector_fetch!($atomic, u32, $($element),+);
        }
        impl $atomic<AtomicU64> {
            impl_vector_fetch!($atomic, u64, $($element),+);
        }
        impl $atomic<AtomicIsize> {
            impl_vector_fetch!($atomic, isize, $($element),+);
        }
        impl $atomic<AtomicUsize> {
            impl_vector_fetch!($atomic, usize, $($element),+);
        }
    };
}
impl_vector_atomic!(Vector2, x, y);
impl_vector_atomic!(Vector3, x, y, z);
impl_vector_atomic!(Vector4, x, y, z, w);