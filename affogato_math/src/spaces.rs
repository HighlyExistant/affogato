use crate::{Integers, Naturals, Reals};

pub struct Add;
pub struct Sub;
pub struct Div;
pub struct Mul;
pub trait BinaryOperation<T> {
    fn operation(lhs: T, rhs: T) -> T;
}
pub trait Closed<T, O: BinaryOperation<T>> {}
pub trait Associative {}
pub trait Distributive {}
pub trait Commutative {}

impl<T: core::ops::Add<Output = T>> BinaryOperation<T> for Add {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.add(rhs)
    }
}
impl<T: core::ops::Sub<Output = T>> BinaryOperation<T> for Sub {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.sub(rhs)
    }
}
impl<T: core::ops::Mul<Output = T>> BinaryOperation<T> for Mul {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.mul(rhs)
    }
}
impl<T: core::ops::Div<Output = T>> BinaryOperation<T> for Div {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.div(rhs)
    }
}

impl<T: core::ops::Add<Output = T>> Closed<T, Add> for Naturals {}
impl<T: core::ops::Mul<Output = T>> Closed<T, Mul> for Naturals {}

impl<T: core::ops::Add<Output = T>> Closed<T, Add> for Integers {}
impl<T: core::ops::Sub<Output = T>> Closed<T, Sub> for Integers {}
impl<T: core::ops::Mul<Output = T>> Closed<T, Mul> for Integers {}

impl<T: core::ops::Add<Output = T>> Closed<T, Add> for Reals {}
impl<T: core::ops::Sub<Output = T>> Closed<T, Sub> for Reals {}
impl<T: core::ops::Mul<Output = T>> Closed<T, Mul> for Reals {}
impl<T: core::ops::Div<Output = T>> Closed<T, Div> for Reals {}

pub trait Group<S: Closed<Self, Add>> 
    where Self: core::ops::Add<Output = Self> + Sized {
}

impl<T: core::ops::Add<Output = T>, S: Closed<T, Add>> Group<S> for T  {
    
}