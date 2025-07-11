
use affogato_core::{groups::vector_spaces::{NormedVectorSpace, VectorSpace}, num::Zero, sets::Real};
use affogato_math::{algebra::Quaternion, matrix::{Matrix3, SquareMatrix}, vector::Vector3, Rotation, Translation};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
/// A rigidbody is a physics object that moves according to Newton-Euler laws of rigid
/// body motion.  
pub trait RigidBody {
    type Vector: NormedVectorSpace;
    type Rotor;
    fn velocity(&self) -> Self::Vector;
    fn angular_velocity(&self) -> Self::Vector;
    fn mass(&self) -> <Self::Vector as VectorSpace>::Scalar;
    fn apply_force(&mut self, force: Self::Vector, pos: Self::Vector);
    fn apply_torque(&mut self, torque: Self::Vector);
    fn step(&mut self, deltatime: <Self::Vector as VectorSpace>::Scalar, gravity: Self::Vector, transform: &mut (impl Rotation<Self::Rotor> + Translation<Self::Vector>));
}

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct RigidBody3D<T: Real> {
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
    fn angular_velocity(&self) -> Self::Vector {
        self.angular_velocity
    }
    fn velocity(&self) -> Self::Vector {
        self.velocity
    }
    fn apply_force(&mut self, force: Self::Vector, pos: Self::Vector) {
        self.net_force += force;
        // L = r * p 
        // L = Angular Momentun
        // r = vector perpendicular to
        let angular_momentum = pos.cross(&force);
        self.apply_torque(angular_momentum);
    }
    fn apply_torque(&mut self, torque: Self::Vector) {
        self.net_torque += torque;
    }
    fn mass(&self) -> <Self::Vector as VectorSpace>::Scalar {
        self.mass
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
    pub fn new(mass: T) -> Self {
        Self::new_with_inertia(mass, Matrix3::identity() )
    }
    pub fn new_with_inertia(mass: T, inertia: Matrix3<T>) -> Self {
        Self { 
            velocity: Vector3::ZERO, 
            angular_velocity: Vector3::ZERO, 
            net_force: Vector3::ZERO, 
            net_torque: Vector3::ZERO, 
            mass, 
            inertia
        }
    }
}