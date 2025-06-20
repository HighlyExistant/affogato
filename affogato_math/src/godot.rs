use godot::builtin::Basis;

use crate::{algebra::Quaternion, geometry::{Plane, Rect, Rect3D}, matrix::Matrix3, vector::{FVec2, FVec3, FVec4, IVec2, IVec3, IVec4}};

impl From<FVec2> for godot::builtin::Vector2 {
    fn from(value: FVec2) -> Self {
        Self { x: value.x(), y: value.y() }
    }
}

impl From<FVec3> for godot::builtin::Vector3 {
    fn from(value: FVec3) -> Self {
        Self { x: value.x(), y: value.y(), z: value.z() }
    }
}

impl From<FVec4> for godot::builtin::Vector4 {
    fn from(value: FVec4) -> Self {
        Self { x: value.x(), y: value.y(), z: value.z(), w: value.w() }
    }
}

impl From<IVec2> for godot::builtin::Vector2i {
    fn from(value: IVec2) -> Self {
        Self { x: value.x(), y: value.y() }
    }
}

impl From<IVec3> for godot::builtin::Vector3i {
    fn from(value: IVec3) -> Self {
        Self { x: value.x(), y: value.y(), z: value.z() }
    }
}

impl From<IVec4> for godot::builtin::Vector4i {
    fn from(value: IVec4) -> Self {
        Self { x: value.x(), y: value.y(), z: value.z(), w: value.w() }
    }
}

impl From<Rect<f32>> for godot::builtin::Rect2 {
    fn from(value: Rect<f32>) -> Self {
        Self { 
            position: value.min.into(), 
            size: value.size().into()
        }
    }
}
impl From<Rect<i32>> for godot::builtin::Rect2i {
    fn from(value: Rect<i32>) -> Self {
        Self { 
            position: value.min.into(), 
            size: value.size().into()
        }
    }
}

impl From<Quaternion<f32>> for godot::builtin::Quaternion {
    fn from(value: Quaternion<f32>) -> Self {
        Self { 
            w: value.w,
            x: value.i,
            y: value.j,
            z: value.k
        }
    }
}
/// affogato matrices are column major, while godot's Basis is row major.
impl From<Matrix3<f32>> for godot::builtin::Basis {
    fn from(value: Matrix3<f32>) -> Self {
        Basis {
            rows: [
                godot::builtin::Vector3 { 
                    x: value.x.x(), 
                    y: value.y.x(), 
                    z: value.z.x() 
                },
                godot::builtin::Vector3 { 
                    x: value.x.y(), 
                    y: value.y.y(), 
                    z: value.z.y() 
                },
                godot::builtin::Vector3 { 
                    x: value.x.z(), 
                    y: value.y.z(), 
                    z: value.z.z() 
                },
            ]
        }
    }
}
impl From<Plane<f32>> for godot::builtin::Plane {
    fn from(value: Plane<f32>) -> Self {
        Self { 
            normal: value.normal().into(), 
            d: value.distance()
        }
    }
}

impl From<Rect3D<f32>> for godot::builtin::Aabb {
    fn from(value: Rect3D<f32>) -> Self {
        
        Self { 
            position: value.min.into(), 
            size: value.size().into()
        }
    }
}