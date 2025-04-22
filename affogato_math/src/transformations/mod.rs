mod translation;
mod rotate;
mod scale;
pub use translation::*;
pub use rotate::*;
pub use scale::*;

pub trait Transformation: Translation<Self::Translate> + Rotation<Self::Rotate> + Scaling<Self::Scale> {
    type Scale;
    type Rotate;
    type Translate;
    fn transform() {
        
    }
}