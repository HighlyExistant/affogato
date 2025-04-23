use std::{collections::HashSet, ops::Range};

use affogato::{geometry::{Ray3D, Rect3D, Sphere, Tetrahedron}, mappings::morton::MortonU64, matrix::{Matrix4, SquareMatrix}, vector::{FVec2, FVec3, OuterProduct, Vector}, One, Zero};
use affogato_physics::collision::{Collision, GJKColliderSolid};
use graphics_feature::Geometry;
pub fn quickhull(points: &[FVec3]) -> (HashSet<usize>) {
    if points.len() <= 2 {
        return (0..points.len()).collect::<HashSet<usize>>();
    }
    let mut convex_hull = HashSet::new();
    let mut sort = points.to_vec();
    sort.sort_by(|cmp, cmp2|{
        cmp.x.total_cmp(&cmp2.x)
    });
    
    let p1 = sort[0];
    let p2 = sort[sort.len()-1];

    convex_hull.insert(0);
    convex_hull.insert(sort.len()-1);
    
    convex_hull
}
fn main() {
    
    let t = Tetrahedron::new(
        FVec3::new(1.0, 0.0, 0.0), 
        FVec3::new(0.0, 1.0, 0.0), 
        FVec3::new(0.0, 0.0, 1.0), 
        FVec3::new(-1.0, 0.0, 0.0)
    );

    println!("{}", t.normal_const::<3>());
}