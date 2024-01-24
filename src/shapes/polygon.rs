use crate::linear::Vector;
pub trait CalculateCentroid {
    type VectorType: Vector;
    fn centroid(&self) -> Self::VectorType;
}

// 6sc             [6]
// 6inc            [12]
// [1sc 1inc]x6    [18]
// [1sc 1inc]x6    [27]
// [8sc 1inc]x6    [30]
// [14sc 1inc]x2   [32]
// 1sc 1inc 13sc 1inc 1sc 1inc 13sc 1inc [36] 5
// [36sc]x3