use crate::matrix::Matrix4;
use crate::vector::Vector3;
use crate::quaternion::Quaternion;

pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion,
    scale:    Vector3<f32>
}

impl Transform {
    pub fn as_matrix(&self) -> Matrix4 {
        let t = Matrix4::translate(self.position);
        let r = self.rotation.as_mat4();
        let s = Matrix4::scale(self.scale);

        r*s*t
    }

    pub fn look_at(&mut self, target: Vector3<f32>, up: Vector3<f32>){
        let pos = self.position;
        let mut f = target - pos;
        if f.length_squared() == 0. {
            println!("Already looking at target");
            return
        }
        f.normalize();
        let mut u = up.clone();
        u.normalize();
        let mut r = f.cross(&u);
        r.normalize();
        u = r.cross(&f);

        self.set_position(Vector3::new(-r.dot(&pos), -u.dot(&pos), f.dot(&pos)));   
        self.set_rotation(Quaternion::from_euler_angles(f32::asin(-f.y), f32::asin(r.z), f32::asin(u.x)));
        self.set_scale(Vector3::new(r.x, u.y, -f.z));
    }

    pub fn set_position(&mut self, pos: Vector3<f32>){
        self.position = pos;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>){
        self.scale = scale;
    }

    pub fn set_rotation(&mut self, rot: Quaternion){
        self.rotation = rot;
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn get_scale(&self) -> Vector3<f32>{
        self.scale
    }

    pub fn get_rotation(&self) -> Quaternion {
        self.rotation
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            rotation: Quaternion::default(),
            scale: Vector3::new(1., 1., 1.),
        }
    }
}
