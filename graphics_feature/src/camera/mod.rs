use affogato_math::{matrix::SquareMatrix, vector::{FMat3, FMat4, FVec3, OuterProduct, Vector}, Zero};
pub struct Camera {
    projection: FMat4,
    view: FMat4,
}
pub trait CameraProjection {
    fn projection(&self) -> FMat4;
}
impl Camera {
    pub fn identity() -> Self {
        Self { projection: FMat4::identity(), view: FMat4::identity() }
    }
    pub fn new(projection: impl CameraProjection) -> Self {
        Self { projection: projection.projection(), view: FMat4::identity() }
    }
    pub fn orthographic(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let right_left_dif = right-left;
        let bottom_top_dif = bottom-top;
        let far_near_dif = far-near;
        let right_left_sum = right+left;
        let bottom_top_sum = bottom+top;
        Self { 
            projection: FMat4::new(
                2.0/right_left_dif, 0.0, 0.0, 0.0, 
                0.0, 2.0/bottom_top_dif, 0.0, 0.0, 
                0.0, 0.0, 2.0/far_near_dif, 0.0, 
                (-right_left_sum)/right_left_dif, (-bottom_top_sum)/bottom_top_dif, (-near)/far_near_dif, 1.0),
            view: FMat4::identity(),
        }
    }
    pub fn perspective(near: f32, far: f32, fovy: f32, aspect_ratio: f32) -> Self {
        let tan_half_fovy = f32::tan(fovy / 2.0);
        
        let mut projection = FMat4::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        projection.x.set_x(1.0 / (aspect_ratio * tan_half_fovy));
        projection.y.set_y(1.0 / (tan_half_fovy));
        projection.z.set_z(far / (far - near));
        projection.z.set_w(1.0);
        projection.w.set_z(-(far * near) / (far - near));
        Self { projection, view: FMat4::identity() }
    }
    pub fn set_view(&mut self, view: FMat4) {
        self.view = view;
    }
    pub fn set_projection(&mut self, projection: FMat4) {
        self.projection = projection;
    }
    pub fn set_view_target(&mut self, position: FVec3, target: FVec3, up: FVec3) {
      self.set_view_direction(position, target - position, up);
    }
    pub fn set_view_direction(&mut self, position: FVec3, direction: FVec3, up: FVec3) {
        let w = direction.normalize();
        let u = w.cross(&up).normalize();
        let v = w.cross(&u);

        self.view = FMat4::identity();
        self.view.x.set_x(u.x());
        self.view.y.set_x(u.y());
        self.view.z.set_x(u.z());
        self.view.x.set_y(v.x());
        self.view.y.set_y(v.y());
        self.view.z.set_y(v.z());
        self.view.x.set_z(w.x());
        self.view.y.set_z(w.y());
        self.view.z.set_z(w.z());
        self.view.w.set_x(-u.dot(&position));
        self.view.w.set_y(-v.dot(&position));
        self.view.w.set_z(-w.dot(&position));
    }
    pub fn set_view_from_transform(&mut self, position: FVec3, rotation: FVec3) {
        let c3 = rotation.z().cos();
        let s3 = rotation.z().sin();
        let c2 = rotation.x().cos();
        let s2 = rotation.x().sin();
        let c1 = rotation.y().cos();
        let s1 = rotation.y().sin();
        let u = FVec3::new(c1 * c3 + s1 * s2 * s3, 2.0 * s3, c1 * s2 * s3 - c3 * s1);
        let v = FVec3::new(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = FVec3::new(c2 * s1, -s2, c1 * c2);
        let mut view = FMat4::identity();
        view.x.set_x(u.x());
        view.y.set_x(u.y());
        view.z.set_x(u.z());
        view.x.set_y(v.x());
        view.y.set_y(v.y());
        view.z.set_y(v.z());
        view.x.set_z(w.x());
        view.y.set_z(w.y());
        view.z.set_z(w.z());
        view.w.set_x(-u.dot(&position));
        view.w.set_y(-v.dot(&position));
        view.w.set_z(-w.dot(&position));
        self.view = view;
    }
    pub fn get_camera_matrix(&self) -> FMat4 {
        self.projection*self.view
    }
    pub fn get_view(&self) -> FMat4 {
        self.view.clone()
    }
    pub fn get_projection(&self) -> FMat4 {
        self.projection.clone()
    }
}
pub struct PerspectiveCameraProjection {
    near: f32, 
    far: f32, 
    fovy: f32, 
    aspect_ratio: f32,
}
impl PerspectiveCameraProjection {
    pub fn new(near: f32, far: f32, fovy: f32, aspect_ratio: f32) -> Self {
        Self { near, far, fovy, aspect_ratio }
    }
}
impl CameraProjection for PerspectiveCameraProjection {
    fn projection(&self) -> FMat4 {
        let tan_half_fovy = f32::tan(self.fovy / 2.0);
        
        let mut projection = FMat4::ZERO;
        projection.x.set_x(1.0 / (self.aspect_ratio * tan_half_fovy));
        projection.y.set_y(1.0 / (tan_half_fovy));
        projection.z.set_z(self.far / (self.far - self.near));
        projection.z.set_w(1.0);
        projection.w.set_z(-(self.far * self.near) / (self.far - self.near));
        projection
    }
}
pub struct OrthographicCameraProjection {
    left: f32, 
    right: f32, 
    top: f32, 
    bottom: f32,
    near: f32, 
    far: f32,
}
impl OrthographicCameraProjection {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32,near: f32, far: f32) -> Self {
        Self { left, right, top, bottom, near, far }
    }
}
impl CameraProjection for OrthographicCameraProjection {
    fn projection(&self) -> FMat4 {
        let right_left_dif = self.right-self.left;
        let bottom_top_dif = self.bottom-self.top;
        let far_near_dif = self.far-self.near;
        let right_left_sum = self.right+self.left;
        let bottom_top_sum = self.bottom+self.top;
        FMat4::new(
            2.0/right_left_dif, 0.0, 0.0, 0.0, 
            0.0, 2.0/bottom_top_dif, 0.0, 0.0, 
            0.0, 0.0, 2.0/far_near_dif, 0.0, 
            (-right_left_sum)/right_left_dif, (-bottom_top_sum)/bottom_top_dif, (-self.near)/far_near_dif, 1.0)
    }
}


pub struct IsometricCameraProjection {
    rotate_x_axis: u8,
    rotate_y_axis: u8,
}
impl Default for IsometricCameraProjection {
    fn default() -> Self {
        Self { 
            rotate_x_axis: 0, 
            rotate_y_axis: 0 
        }
    }
}
impl IsometricCameraProjection {
    pub fn new(rotate_x_90deg: u8, rotate_y_90deg: u8) -> Self {
        Self { rotate_x_axis: rotate_x_90deg%4, rotate_y_axis: rotate_y_90deg%4 }
    }
}
impl CameraProjection for IsometricCameraProjection {
    fn projection(&self) -> FMat4 {
        let rot_a = (35.264389701728f32+(90.0*self.rotate_y_axis as f32)).to_radians();
        let rot_b = (45.0f32+(90.0*self.rotate_x_axis as f32)).to_radians();
        let mat = FMat3::new(
            rot_b.cos(), rot_b.sin()*rot_a.sin(), 0.0, 
            0.0, rot_a.cos(), 0.0, 
            rot_b.sin(), -rot_a.sin()*rot_b.cos(), 0.0
        );

        FMat4::from(mat)
    }
}