pub trait SignedDistance<T> {
    type Distance;
    fn sdf(&self, object: &T) -> Self::Distance;
}

pub trait RoundSignedDistance<T>: SignedDistance<T> {
    type Radius;
    fn round_sdf(&self, object: &T, r: Self::Radius) -> Self::Distance;
}