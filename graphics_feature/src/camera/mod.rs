use affogato_math::{matrix::{Matrix4, SquareMatrix}, vector::{CrossProduct, FMat4, FVec3, Vector}};
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
        projection.x.x = 1.0 / (aspect_ratio * tan_half_fovy);
        projection.y.y = 1.0 / (tan_half_fovy);
        projection.z.z = far / (far - near);
        projection.z.w = 1.0;
        projection.w.z = -(far * near) / (far - near);
        Self { projection, view: FMat4::identity() }
    }
    pub fn set_view_target(&mut self, position: FVec3, target: FVec3, up: FVec3) {
      self.set_view_direction(position, target - position, up);
    }
    pub fn set_view_direction(&mut self, position: FVec3, direction: FVec3, up: FVec3) {
        let w = direction.normalize();
        let u = w.cross(&up).normalize();
        let v = w.cross(&u);

        self.view = FMat4::identity();
        self.view.x.x = u.x;
        self.view.y.x = u.y;
        self.view.z.x = u.z;
        self.view.x.y = v.x;
        self.view.y.y = v.y;
        self.view.z.y = v.z;
        self.view.x.z = w.x;
        self.view.y.z = w.y;
        self.view.z.z = w.z;
        self.view.w.x = -u.dot(&position);
        self.view.w.y = -v.dot(&position);
        self.view.w.z = -w.dot(&position);
    }
    pub fn set_view(&mut self, position: FVec3, rotation: FVec3) {
        let c3 = rotation.z.cos();
        let s3 = rotation.z.sin();
        let c2 = rotation.x.cos();
        let s2 = rotation.x.sin();
        let c1 = rotation.y.cos();
        let s1 = rotation.y.sin();
        let u = FVec3::new(c1 * c3 + s1 * s2 * s3, 2.0 * s3, c1 * s2 * s3 - c3 * s1);
        let v = FVec3::new(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = FVec3::new(c2 * s1, -s2, c1 * c2);
        let mut view = FMat4::identity();
        view.x.x = u.x;
        view.y.x = u.y;
        view.z.x = u.z;
        view.x.y = v.x;
        view.y.y = v.y;
        view.z.y = v.z;
        view.x.z = w.x;
        view.y.z = w.y;
        view.z.z = w.z;
        view.w.x = -u.dot(&position);
        view.w.y = -v.dot(&position);
        view.w.z = -w.dot(&position);
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
        
        let mut projection = FMat4::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        projection.x.x = 1.0 / (self.aspect_ratio * tan_half_fovy);
        projection.y.y = 1.0 / (tan_half_fovy);
        projection.z.z = self.far / (self.far - self.near);
        projection.z.w = 1.0;
        projection.w.z = -(self.far * self.near) / (self.far - self.near);
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