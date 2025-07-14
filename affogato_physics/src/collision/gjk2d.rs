#![cfg(feature="alloc")]

use affogato_core::{num::Zero, sets::Real};
use affogato_math::{matrix::Matrix3, vector::{NormedVectorSpace, Vector2, Vector3, VectorSpace}};

use crate::collision::{Collision, CollisionInfo};
extern crate alloc;
fn furthest_point<T: Real>(direction: Vector2<T>, vertices: &alloc::vec::Vec<Vector2<T>>, transform: &Matrix3<T>) -> Vector2<T> {
    let mut max_distance = -T::INFINITY;
    let mut max = Vector2::<T>::ZERO;

    for vertex in vertices.clone() {
        let point = Vector2::<T>::from(*transform * Vector3::<T>::new(vertex.x(), vertex.y(), T::ONE));
        let distance = point.dot(&direction);
        if distance > max_distance {
            max_distance = distance;
            max = point;
        }
    }
    max
}
fn same_direction<T: Real>(
	direction: &Vector2<T>,
	ao: &Vector2<T>) -> bool
{
	return direction.dot(ao) > T::ZERO;
}

fn minkowski_support<T: Real>(direction: Vector2<T>, vertices1: &alloc::vec::Vec<Vector2<T>>, vertices2: &alloc::vec::Vec<Vector2<T>>, transform1: &Matrix3<T>, transform2: &Matrix3<T>) -> Vector2<T>  {
    let b = furthest_point(-direction, vertices2, transform2);
    let a = furthest_point(direction, vertices1, transform1);
    a - b
}

fn gjk<T: Real>(
    vertices1: &alloc::vec::Vec<Vector2<T>>, 
    vertices2: &alloc::vec::Vec<Vector2<T>>, 
    transform1: &Matrix3<T>, 
    transform2: &Matrix3<T>
) -> Option<alloc::vec::Vec<Vector2<T>>> {
    let mut simplex: alloc::vec::Vec<Vector2<T>> = alloc::vec![Vector2::ZERO; 3];

    let (mut a, mut b, mut c, mut ao, mut ab, mut ac, mut abperp, mut acperp) = (
        Vector2::<T>::ZERO, Vector2::<T>::ZERO, Vector2::<T>::ZERO, Vector2::<T>::ZERO, 
        Vector2::<T>::ZERO, Vector2::<T>::ZERO, Vector2::<T>::ZERO, Vector2::<T>::ZERO
    );

    let mut d = Vector2::right();

    a = minkowski_support(d, vertices1, vertices2, transform1, transform2);
    simplex[0] = a;
    
    if a.dot(&d) <= T::ZERO {
        return None;
    }

    d = -d;
    let mut i = 0usize;

    loop {
        i += 1;
        simplex[i] = minkowski_support(d, vertices1, vertices2, transform1, transform2);
        a = simplex[i];

        if a.dot(&d) <= T::ZERO {
            return None;
        }
        ao = -a;

        if i < 2 {
            b = simplex[0];
            ab = b-a;
            
            // d = vector_triple_product_v3(ab, ao, ab);
            d = ab.vector_triple_product(&ao, &ab);
            if d.length_squared() == T::ZERO {
                d = Vector2::new(ab.y(), -ab.x());
            }
            continue;
        }

        b = simplex[1];
        c = simplex[0];
        ab = b-a;
        ac = c-a;

        // acperp = vector_triple_product_v3(ab, ac, ac);
        acperp = ac.vector_triple_product(&ac, &ab);

        if acperp.dot(&ao) >= T::ZERO {
            d = acperp;
        } else {
            // abperp = vector_triple_product_v3(ac, ab, ab);
            abperp = ab.vector_triple_product(&ab, &ac);

            if abperp.dot(&ao) < T::ZERO {
                return Some(simplex);
            }

            simplex[0] = simplex[1];

            d = abperp;
        }

        simplex[1] = simplex[2];

        i -= 1;
    }

    unreachable!()
}
#[derive(Clone, Copy, Debug, Default)]
struct EdgeInfo<T: Real> {
    distance: T,
    normal: Vector2<T>,
    index: usize,
}
fn find_closest_edge<T: Real>(s: &[Vector2<T>]) -> EdgeInfo<T> {
    let mut edge_info = EdgeInfo {
        distance: T::MAX,
        index: 0,
        normal: Vector2::right(),
    };
    for i in 0..s.len() {
        let j = if i + 1 == s.len() {
            0
        } else {
            1
        };
        let a = s[i];
        let b = s[j];
        let e = b - a;

        let oa = a;

        let n = e.vector_triple_product(&oa, &e).normalize();

        let d = n.dot(&a);
        if d < edge_info.distance {
            edge_info.distance = d;
            edge_info.normal = n;
            edge_info.index = j;
        }
    }
    edge_info
}

fn gjk_epa<T: Real>(vertices1: &alloc::vec::Vec<Vector2<T>>, vertices2: &alloc::vec::Vec<Vector2<T>>, transform1: &Matrix3<T>, transform2: &Matrix3<T>) -> Option<CollisionInfo<Vector2<T>>> {
    let mut simplex = gjk(vertices1, vertices2, transform1, transform2)?;
    let mut p = Vector2::ZERO;
    let mut edge_info = EdgeInfo {
        distance: T::MAX,
        index: 0,
        normal: Vector2::right(),
    };
    let mut result = CollisionInfo {
        distance: T::MAX,
        normal: Vector2::right(),
    };
    let mut index = 0;
    for i in 0..25 {
        edge_info = find_closest_edge(&simplex);
        let p = minkowski_support(edge_info.normal, vertices1, vertices2, transform1, transform2);
        let d = p.dot(&edge_info.normal);
        if d - edge_info.distance < T::EPSILON {
            result.normal = edge_info.normal;
            result.distance = d;
            index = edge_info.index;
            return Some(result);
        } else {
            simplex.insert(edge_info.index, p);
        }
    }
    result.normal = edge_info.normal;
    result.distance = p.dot(&edge_info.normal);
    Some(result)
}

pub struct GJKColliderFlat<T: Real> {
    pub vertices: alloc::vec::Vec<Vector2<T>>,
    pub transform: Matrix3<T>,
}
impl<T: Real> GJKColliderFlat<T> {
    pub fn new(vertices: alloc::vec::Vec<Vector2<T>>, transform: Matrix3<T>) -> Self {
        GJKColliderFlat { vertices, transform }
    }
}
impl<T: Real> Collision<Self> for GJKColliderFlat<T> {
    type CollisionInfo = CollisionInfo<Vector2<T>>;
    fn collides(&self, object: &Self) -> Option<Self::CollisionInfo> {
        gjk_epa(&self.vertices, &object.vertices, &self.transform, &object.transform)
    }
}