
use affogato_core::{groups::vector_spaces::{NormedVectorSpace, VectorSpace}, num::Zero, sets::Real};
use affogato_math::{algebra::Quaternion, geometry::{CalculateCentroid, Rect, Rect3D}, matrix::{Matrix3, SquareMatrix}, vector::{Vector2, Vector3}, Rotation, Translation};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{collision::Collision, Inertia};
/// A rigidbody is a physics object that moves according to Newton-Euler laws of rigid
/// body motion.
pub trait RigidBody {
    type Vector: NormedVectorSpace;
    type Rotor;
    type Torque;
    fn velocity(&self) -> Self::Vector;
    fn angular_velocity(&self) -> Self::Torque;
    fn mass(&self) -> <Self::Vector as VectorSpace>::Scalar;
    fn set_mass(&mut self, mass: <Self::Vector as VectorSpace>::Scalar);
    fn apply_force(&mut self, force: Self::Vector);
    fn apply_force_at(&mut self, force: Self::Vector, at: Self::Vector);
    fn apply_torque(&mut self, torque: Self::Torque);
    fn step(&mut self, deltatime: <Self::Vector as VectorSpace>::Scalar, gravity: Self::Vector, transform: &mut (impl Rotation<Self::Rotor> + Translation<Self::Vector>));
}

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct RigidBody3D<T: Real> {
    pub center_of_mass: Vector3<T>,
    pub velocity: Vector3<T>,
    pub angular_velocity: Vector3<T>,
    pub mass: T,
    net_force: Vector3<T>,
    net_torque: Vector3<T>,
    pub inertia: Matrix3<T>,
}

impl<T: Real> RigidBody for RigidBody3D<T> {
    type Vector = Vector3<T>;
    type Rotor = Quaternion<T>;
    type Torque = Self::Vector;
    fn angular_velocity(&self) -> Self::Vector {
        self.angular_velocity
    }
    fn velocity(&self) -> Self::Vector {
        self.velocity
    }
    /// Applies a force to the [`RigidBody`]. This does not affect
    /// torque.
    /// 
    /// * `force`: The amount of force exerted in a specific axis.
    fn apply_force(&mut self, force: Self::Vector) {
        self.net_force += force;
    }
    /// Applies a force to the [`RigidBody`] at some point relative
    /// to the center of mass.
    /// 
    /// * `force`: The amount of force exerted in a specific axis
    /// 
    /// * `at`: The position relative to the center of mass in which
    /// the force has been exerted. If you don't want to apply torque
    /// to the object, you can make this equal to the center of mass,
    /// although it's reccomended to just use `apply_force` function
    /// instead.
    fn apply_force_at(&mut self, force: Self::Vector, at: Self::Vector) {
        self.net_force += force;
        // L = r * p 
        // L = Angular Momentun
        // r = vector perpendicular to
        let angular_momentum = (at - self.center_of_mass).cross(&force);
        self.apply_torque(angular_momentum);
    }
    fn apply_torque(&mut self, torque: Self::Vector) {
        self.net_torque += torque;
    }
    fn mass(&self) -> <Self::Vector as VectorSpace>::Scalar {
        self.mass
    }
    fn set_mass(&mut self, mass: <Self::Vector as VectorSpace>::Scalar) {
        self.mass = mass;
    }
    fn step(&mut self, deltatime: <Self::Vector as VectorSpace>::Scalar, gravity: Self::Vector, transform: &mut (impl Rotation<Self::Rotor> + Translation<Self::Vector>)) {
        // apply gravity
        self.net_force += gravity * self.mass;
        self.velocity += self.net_force / self.mass * deltatime;

        // get angular velocity 
        self.angular_velocity += self.inertia*(self.net_torque * deltatime);

        let vec = if self.angular_velocity == T::ZERO {
            Vector3::forward()
        } else {
            self.angular_velocity.normalize()
        };
        let rotation = Quaternion::angle_axis(
            self.angular_velocity.length() * deltatime, 
            vec);
            
        // end
        self.net_force = Vector3::ZERO;
        self.net_torque = Vector3::ZERO;
        // transform
        transform.rotate(&rotation);
        transform.translate(self.velocity * deltatime);
    }
}

impl<T: Real> RigidBody3D<T> {
    pub fn new(mass: T, center_of_mass: Vector3<T>) -> Self {
        Self::new_with_inertia(mass, center_of_mass, Matrix3::identity() )
    }
    pub fn from_rect(rect: Rect3D<T>, density: T) -> Self {
        Self::new_with_inertia(density*rect.volume(), rect.centroid(), rect.inertia(density))
    }
    pub fn new_with_inertia(mass: T, center_of_mass: Vector3<T>, inertia: Matrix3<T>) -> Self {
        Self { 
            center_of_mass,
            velocity: Vector3::ZERO, 
            angular_velocity: Vector3::ZERO, 
            net_force: Vector3::ZERO, 
            net_torque: Vector3::ZERO, 
            mass, 
            inertia
        }
    }
}

pub struct RigidBody2D<T: Real> {
    pub center_of_mass: Vector2<T>,
    velocity: Vector2<T>,
    angular_velocity: T,
    mass: T,
    pub net_force: Vector2<T>,
    pub net_torque: T,
    pub inertia: T,
}

impl<T: Real> RigidBody2D<T> {
    pub fn new(mass: T, center_of_mass: Vector2<T>) -> Self {
        Self::new_with_inertia(mass, center_of_mass, T::ONE)
    }
    pub fn from_rect(rect: Rect<T>, density: T) -> Self {
        Self::new_with_inertia(density*rect.area(), rect.centroid(), rect.inertia(density))
    }
    pub fn new_with_inertia(mass: T, center_of_mass: Vector2<T>, inertia: T) -> Self {
        Self { 
            center_of_mass,
            velocity: Vector2::ZERO, 
            angular_velocity: T::ZERO, 
            net_force: Vector2::ZERO, 
            net_torque: T::ZERO, 
            mass, 
            inertia
        }
    }
}

impl<T: Real> RigidBody for RigidBody2D<T> {
    type Vector = Vector2<T>;
    type Rotor = T;
    type Torque = T;
    fn velocity(&self) -> Self::Vector {
        self.velocity
    }
    fn angular_velocity(&self) -> Self::Torque {
        self.angular_velocity
    }
    fn apply_force(&mut self, force: Self::Vector) {
        self.net_force += force;
    }
    fn apply_force_at(&mut self, force: Self::Vector, at: Self::Vector) {
        self.net_force += force;
        let radius = (at-self.center_of_mass);
        let mut angular_momentum = radius.cross(&force);
        self.apply_torque(angular_momentum);
    }
    fn apply_torque(&mut self, torque: Self::Torque) {
        self.net_torque += torque
    }
    fn mass(&self) -> <Self::Vector as VectorSpace>::Scalar {
        self.mass
    }
    fn set_mass(&mut self, mass: <Self::Vector as VectorSpace>::Scalar) {
        self.mass = mass;
    }
    fn step(&mut self, deltatime: <Self::Vector as VectorSpace>::Scalar, gravity: Self::Vector, transform: &mut (impl Rotation<Self::Rotor> + Translation<Self::Vector>)) {
        self.net_force += gravity * self.mass;
        self.velocity += self.net_force / self.mass * deltatime;

        // get angular velocity 
        self.angular_velocity += self.inertia*(self.net_torque * deltatime);
        
        let rotation = self.angular_velocity;

        self.net_force = Vector2::ZERO;
        self.net_torque = T::ZERO;
        
        transform.rotate(&(rotation * deltatime));
        transform.translate(self.velocity * deltatime);
    }
}