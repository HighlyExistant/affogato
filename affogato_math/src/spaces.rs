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

impl<T: std::ops::Add<Output = T>> BinaryOperation<T> for Add {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.add(rhs)
    }
}
impl<T: std::ops::Sub<Output = T>> BinaryOperation<T> for Sub {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.sub(rhs)
    }
}
impl<T: std::ops::Mul<Output = T>> BinaryOperation<T> for Mul {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.mul(rhs)
    }
}
impl<T: std::ops::Div<Output = T>> BinaryOperation<T> for Div {
    fn operation(lhs: T, rhs: T) -> T {
        lhs.div(rhs)
    }
}

impl<T: std::ops::Add<Output = T>> Closed<T, Add> for Naturals {}
impl<T: std::ops::Mul<Output = T>> Closed<T, Mul> for Naturals {}

impl<T: std::ops::Add<Output = T>> Closed<T, Add> for Integers {}
impl<T: std::ops::Sub<Output = T>> Closed<T, Sub> for Integers {}
impl<T: std::ops::Mul<Output = T>> Closed<T, Mul> for Integers {}

impl<T: std::ops::Add<Output = T>> Closed<T, Add> for Reals {}
impl<T: std::ops::Sub<Output = T>> Closed<T, Sub> for Reals {}
impl<T: std::ops::Mul<Output = T>> Closed<T, Mul> for Reals {}
impl<T: std::ops::Div<Output = T>> Closed<T, Div> for Reals {}

pub trait Group<S: Closed<Self, Add>> 
    where Self: std::ops::Add<Output = Self> + Sized {
}

impl<T: std::ops::Add<Output = T>, S: Closed<T, Add>> Group<S> for T  {
    
}