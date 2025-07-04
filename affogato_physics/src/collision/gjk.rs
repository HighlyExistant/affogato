#![cfg(feature="alloc")]
extern crate alloc;

use affogato_core::{groups::vector_spaces::{VectorSpace, NormedVectorSpace}, num::Zero, sets::Real};
use alloc::fmt::{Debug, Display};

use affogato_math::{geometry::Dimension, matrix::{Matrix4, SquareMatrix}, vector::{Vector3, Vector4}, Transformation};
use core::ops::{Add, Mul};
use alloc::vec::Vec;
use super::{Collision, CollisionInfo, HitCollisionInfo};
#[derive(Debug)]
pub(crate) struct Simplex<V: VectorSpace + Dimension, const DIMENSION: usize> {
    pub points: [V; DIMENSION],
    pub size: usize,
}
impl<V: VectorSpace + Dimension, const DIMENSION: usize> Simplex<V, DIMENSION> {
    pub fn new() -> Self {
        Self { points: [V::ZERO; DIMENSION], size: 0 }
    }
    pub fn push_front(&mut self, value: V) {
        for i in (1..self.points.len()).rev() {
            self.points[i] = self.points[i-1].clone();
        }
        self.points[0] = value.clone();
        self.size = DIMENSION.min(self.size+1);
    }
    fn initialize(&mut self, list: &[V]) {
		self.size = 0;
        for p in list {
            self.points[self.size] = p.clone();
            self.size += 1;
        }
    }
}
// pub struct QHSimplex<T: Real> {
//     s: Simplex<Vector3<T>, 4>,
//     claimed: Vec<Vector3<T>>,
//     outside: Vec<Vector3<T>>,
// }
// impl<T: Real> QHSimplex<T> {
//     pub fn build_from_points(tetrahedron: &[Vector3<T>; 4]) {
//         let mut i = 0;
//         let center = (tetrahedron[0]+tetrahedron[1]+tetrahedron[2]+tetrahedron[3]).mul(T::from_f64(0.25));

//     }
//     pub fn from_points(points: &[Vector3<T>]) -> Simplex<Vector3<T>, 4> {
//         let mut min_vertices = [Vector3::from(T::MAX); 4];
//         let mut max_vertices = [Vector3::from(T::MIN); 4];

//         for p in points {
//             for i in 0..3 {
//                 if p.get(i).unwrap() < min_vertices[i].get(i).unwrap() {
//                     min_vertices[i] = p.clone();
//                 }
//                 if p.get(i).unwrap() > max_vertices[i].get(i).unwrap() {
//                     max_vertices[i] = p.clone();
//                 }
//             }
//         }
//         let (v1, v2) = {
//             let mut max_distance = max_vertices[0].x - min_vertices[0].x;
//             let mut max_axis = 0;
//             for i in 1..3 {
//                 let distance = max_vertices[i].get(i).unwrap() - min_vertices[i].get(i).unwrap();
//                 if distance > max_distance {
//                     max_distance = distance;
//                     max_axis = i;
//                 }
//             }
//             (min_vertices[max_axis], max_vertices[max_axis])
//         };

//         let v3 = {
//             let mut max_distance = T::MIN;
//             let mut max_vertex = &Vector3::ZERO;
//             for i in points {
//                 let distance = i.line_distance(v2, v1);
//                 if distance > max_distance {
//                     max_distance = distance;
//                     max_vertex = i;
//                 }
//             }
//             max_vertex.clone()
//         };

//         let v4 = {
//             let mut max_distance = T::MIN;
//             let mut max_vertex = &Vector3::ZERO;
//             for i in points {
//                 let distance = i.plane_distance(v1, v2, v3);
//                 if distance > max_distance {
//                     max_distance = distance;
//                     max_vertex = i;
//                 }
//             }
//             max_vertex.clone()
//         };
//         let signed_dst = v4.signed_plane_distance(v1, v2, v3);
        
//         for p in points {
//             if p.equals_with_epsilon(v1, T::from_f64(10e-5)) || p.equals_with_epsilon(v2, T::from_f64(10e-5)) || p.equals_with_epsilon(v3, T::from_f64(10e-5)) || p.equals_with_epsilon(v4, T::from_f64(10e-5)) {
//                 continue;
//             }
//             let max_dst = T::from_f64(0.01);
//             let max_dist_fac;
//         }
        
//         Self { s: Simplex { points: [v1, v2, v3, v4], size: 4 }, claimed, outside }
//     }
// }
impl<V: VectorSpace + Dimension, const DIMENSION: usize> Simplex<V, DIMENSION>  {
    const CHECK: () = if V::DIMENSION == DIMENSION-1 {
    } else {
        panic!("The dimension must be equal to the vectors dimension + 1");
    };
}

/// # furthest_point
/// *this function only works if the object is a convext polygon*
///
/// given a direction vector and a vector of points
/// return the furthest value.
fn furthest_point<T: Real>(direction: Vector3<T>, vertices: &Vec<Vector3<T>>, transform: &Matrix4<T>) -> Vector3<T>{
    // let transform = Matrix4::identity();
    let mut max_distance = -T::MAX;
    let mut max = Vector3::<T>::ZERO;

    for vertex in vertices.clone() {
        let point = Vector3::<T>::from(*transform * Vector4::<T>::new(vertex.x(), vertex.y(), vertex.z(), T::ONE));
        let distance = point.dot(&direction);
        if distance > max_distance {
            max_distance = distance;
            max = point;
        }
    }
    max
}

fn minkowski_support<T: Real>(direction: Vector3<T>, vertices1: &Vec<Vector3<T>>, vertices2: &Vec<Vector3<T>>, transform1: &Matrix4<T>, transform2: &Matrix4<T>) -> Vector3<T>  {
    let b = furthest_point(-direction, vertices2, transform2);
    let a = furthest_point(direction, vertices1, transform1);
    a - b
}
pub fn same_direction<T: Real>(
	direction: &Vector3<T>,
	ao: &Vector3<T>) -> bool
{
	return direction.dot(ao) > T::ZERO;
}
fn dimension2<T: Real>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
    let b = points.points[1];

    let ab = b-a;
    let ao = -a;
    if same_direction(&ab, &ao) {
        *direction = ab.cross(&ao).cross(&ab);
    } else {
        points.initialize(&[a]);
        *direction = ao;
    }
    false
}
fn dimension3<T: Real>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
	let b = points.points[1];
	let c = points.points[2];

	let ab = b - a;
	let ac = c - a;
	let ao = -a;
    
    let abc = ab.cross(&ac);

    if same_direction(&abc.cross(&ac), &ao) {
        if same_direction(&ac, &ao) {
			points.initialize(&[a, c]);
			*direction = ac.cross(&ao).cross(&ac);
		}
		else {
            points.initialize(&[a, b]);
			return dimension2(points, direction);
		}
    }
    else {
        if same_direction(&ab.cross(&abc), &ao) {
			points.initialize(&[a, b]);
			return dimension2(points, direction);
        } else {
            if same_direction(&abc, &ao) {
                *direction = abc;
            }
            else {
                points.initialize(&[a, c, b]);
                *direction = -abc;
            }
        }
    }
    false
}
fn dimension4<T: Real>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
	let b = points.points[1];
	let c = points.points[2];
	let d = points.points[3];

	let ab = b - a;
	let ac = c - a;
	let ad = d - a;
	let ao = -a;
 
	let abc = ab.cross(&ac);
	let acd = ac.cross(&ad);
	let adb = ad.cross(&ab);
 
	if same_direction(&abc, &ao) {
        points.initialize(&[a, b, c]);
		return dimension3(points, direction);
	}
		
	if same_direction(&acd, &ao) {
        points.initialize(&[a, c, d]);
		return dimension3(points, direction);
	}
 
	if same_direction(&adb, &ao) {
        points.initialize(&[a, d, b]);
		return dimension3(points, direction);
	}

	return true;
}


fn next_simplex<T: Real>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    match points.size {
        2 => dimension2(points, direction),
        3 => dimension3(points, direction),
        4 => dimension4(points, direction),
        _ => false
    }
}
fn get_face_normals<T: Real>(polytope: &Vec<Vector3<T>>, faces: &[u32]) -> (Vec<Vector4<T>>, usize) {
    let mut normals = Vec::<Vector4<T>>::new();
    let mut min_triangle = 0;
    let mut min_distance = T::MAX;
    
    let mut i = 0;
    while i < faces.len() {
        let a = polytope[faces[i] as usize];
        let b = polytope[faces[i + 1] as usize];
        let c = polytope[faces[i + 2] as usize];

        let mut normal = (b - a).cross(&(c - a)).normalize();
        let mut distance = normal.dot(&a);

        if distance < T::from_f64(0.0) {
            normal = normal * -T::from_f64(1.0);
            distance = distance * -T::from_f64(1.0);
        }

        normals.push(Vector4::<T>::new(normal.x(), normal.y(), normal.z(), distance));

        if distance < min_distance {
            min_triangle = i / 3;
            min_distance = distance;
        }

        i += 3;
    }
    return (normals, min_triangle)
}
fn add_if_unique_edge(unique_edges: &mut Vec<(u32, u32)>, faces: &[u32], a: usize, b: usize) {
    let mut reverse = 0;
    for (i, edge) in unique_edges.iter().enumerate() {
        reverse = i;
        if faces[b] == edge.0 && faces[a] == edge.1 {
            break;
        }
    }
    if !unique_edges.is_empty() && reverse != (usize::wrapping_sub(unique_edges.len(), 1))  {
        (unique_edges).remove(reverse);
    }
    else {
        unique_edges.push((faces[a], faces[b]));
    }
}
const MAX_EPA_ITER: usize = 16;
fn epa<T: Real>(simplex: Simplex<Vector3<T>, 4>, vertices1: &Vec<Vector3<T>>, vertices2: &Vec<Vector3<T>>, transform1: &Matrix4<T>, transform2: &Matrix4<T>) -> Option<CollisionInfo<Vector3<T>>> {
    let mut polytope = Vec::<Vector3<T>>::new();
    for i in 0..simplex.size {
        polytope.push(simplex.points[i]);
    }
    let mut faces = alloc::vec![
        0u32, 1, 2,
        0, 3, 1,
        0, 2, 3,
        1, 3, 2
    ];

    let (mut normals, mut min_face) = get_face_normals(&polytope, &faces);

    let mut min_normal = Vector3::from(T::from_f64(0.0));
    let mut min_distance = T::MAX;
    let mut inc = 0;

    while min_distance == T::MAX {
        min_normal = normals[min_face].xyz();
        min_distance = normals[min_face].w();
        let support = minkowski_support(min_normal, &vertices1, &vertices2, &transform1, &transform2);
        let distance = min_normal.dot(&support);

        if T::abs(distance - min_distance) > T::from_f64(0.001) {
            min_distance = T::MAX;

            let mut unique_edges = Vec::<(u32, u32)>::new();
            if inc > MAX_EPA_ITER {
                break;
            }
            inc += 1;
            let mut i = 0;
            while i < normals.len() {
                if same_direction(&normals[i].xyz(), &support) {
                    let f = i * 3;

                    add_if_unique_edge(&mut unique_edges, &faces, f, f + 1);
                    add_if_unique_edge(&mut unique_edges, &faces, f + 1, f + 2);
                    add_if_unique_edge(&mut unique_edges, &faces, f + 2, f);
                    faces[f + 2] = faces[faces.len() - 1]; 
                    faces.pop();
                    faces[f + 1] = faces[faces.len() - 1]; 
                    faces.pop();
                    faces[f] = faces[faces.len() - 1]; 
                    faces.pop();
                    normals[i] = normals[normals.len() - 1];
                    normals.pop();

                    i = usize::wrapping_sub(i, 1);
                }
                i = usize::wrapping_add(i, 1);
            }

            let mut new_faces = Vec::<u32>::new();
            for (index1, index2) in unique_edges {
                new_faces.push(index1);
                new_faces.push(index2);
                new_faces.push(polytope.len() as u32);
            }
            polytope.push(support);

            let (new_normals, new_min_face) = get_face_normals(&polytope, &new_faces);

            let mut old_min_distance = T::MAX;
            for i in 0..normals.len() {
                if normals[i].w() < old_min_distance {
                    old_min_distance = normals[i].w();
                    min_face = i;
                }
            }
            if !new_normals.is_empty() {
                if new_normals[new_min_face].w() < old_min_distance {
                    min_face = new_min_face + normals.len();
                }
            }
            for new in new_faces {
                faces.push(new);
            }
            for new in new_normals {
                normals.push(new);
            }
        }
    }
    Some(CollisionInfo { 
        normal: min_normal, 
        distance: min_distance + T::from_f64(0.001),
    })
}
/// from https://winter.dev/articles/gjk-algorithm
fn gjk<T: Real>(vertices1: &Vec<Vector3<T>>, vertices2: &Vec<Vector3<T>>, transform1: &Matrix4<T>, transform2: &Matrix4<T>) -> Option<CollisionInfo<Vector3<T>>> {
    let mut point = minkowski_support(Vector3::right(), vertices1, vertices2, transform1, transform2);
    
    let mut simplex = Simplex::<Vector3<T>, 4>::new();
    simplex.push_front(point);

    let mut direction = -point;
    loop {
        point = minkowski_support(direction, vertices1, vertices2, transform1, transform2);
        if point.dot(&direction) <= T::ZERO {
            return None;
        }
        simplex.push_front(point);

        if next_simplex(&mut simplex, &mut direction) {
            let collision = epa(simplex, &vertices1, &vertices2, transform1, transform2);
            return collision;
        }
    }
}

#[derive(Clone)]
pub struct GJKColliderSolid<T: Real> {
    pub vertices: Vec<Vector3<T>>,
    pub transform: Matrix4<T>,
}

impl<T: Real> GJKColliderSolid<T> {
    pub fn new(vertices: Vec<Vector3<T>>, transform: Matrix4<T>) -> Self {
        GJKColliderSolid { vertices, transform }
    }
}

impl<T: Real> Collision<Self> for GJKColliderSolid<T> {
    type CollisionInfo = CollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Self) -> Option<Self::CollisionInfo> {
        gjk(&self.vertices, &object.vertices, &self.transform, &object.transform)
    }
}

pub struct GJKColliderSolidRef<'a, T: Real> {
    pub vertices: Vec<Vector3<T>>,
    transform: &'a Matrix4<T>,
}
impl<'a, T: Real> GJKColliderSolidRef<'a, T> {
    pub fn new(vertices: Vec<Vector3<T>>, transform: &'a Matrix4<T>) -> Self {
        Self { vertices, transform }
    }
}

impl<'a, T: Real> Collision<Self> for GJKColliderSolidRef<'a, T> {
    type CollisionInfo = CollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Self) -> Option<Self::CollisionInfo> {
        gjk(&self.vertices, &object.vertices, self.transform, object.transform)
    }
}