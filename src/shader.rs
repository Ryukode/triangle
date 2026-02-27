use std::fs;
use wgpu::{include_wgsl, ShaderModuleDescriptor};
use crate::camera::Camera;
use crate::color::Color;
use crate::model::Model;
use crate::vector::Vector3;

pub enum AnyShader {
    FlatShader(FlatShader),
    PhongShader(PhongShader)
}
impl AnyShader {
    fn call_method<Function, Return>(&self, f: Function) -> Return
    where
        Function: Fn(&dyn BaseShader) -> Return {
        match self {
            AnyShader::PhongShader(s) => {
                f(s)
            }
            AnyShader::FlatShader(s) => {
                f(s)
            }
        }
    }
}

pub trait BaseShader {
    fn activate(&self);
    fn deactivate(&self);
    fn as_vec(&self, model: &Model, cam: &Camera) -> Vec<f32>;
}

impl BaseShader for AnyShader {
    fn activate(&self){
        self.call_method(|s| s.activate());
    }
    fn deactivate(&self){
        self.call_method(|s| s.deactivate());
    }
    fn as_vec(&self, model: &Model, cam: &Camera) -> Vec<f32> {
        self.call_method(|s| s.as_vec(model, cam))
    }
}

pub struct PhongShader {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    light_direction: Vector3<f32>,
    eye_pos: Vector3<f32>,
}

impl BaseShader for PhongShader {
    fn activate(&self) {
        println!("{}", "Activate PhongShader")
    }

    fn deactivate(&self) {
        println!("{}", "Deactivate PhongShader")
    }

    fn as_vec(&self, model: &Model, cam: &Camera) -> Vec<f32> {
        let v = [
            self.ambient.as_vec(),
            self.diffuse.as_vec(),
            self.specular.as_vec(),
            model.transform.get_transform().as_vec(), 
            cam.transform.get_transform().as_vec(), 
            cam.get_projection_matrix().as_vec(),
            self.light_direction.as_vec(),
            vec![0.],
            self.eye_pos.as_vec(),
            vec![0.]].concat();
        v
    }
}

impl PhongShader {
    pub fn set_ambient(&mut self, ambient: Color) {
        self.ambient = ambient;
    }
    pub fn set_diffuse(&mut self, diffuse: Color) {
        self.diffuse = diffuse;
    }
    pub fn set_specular(&mut self, specular: Color) {
        self.specular = specular;
    }
    pub fn set_light_dir(&mut self, dir: Vector3<f32>) {
        self.light_direction = dir;
    }
    pub fn set_eye_pos(&mut self, pos: Vector3<f32>) {
        self.eye_pos = pos;
    }
}

impl Default for PhongShader {
    fn default() -> Self {
        Self {
            ambient: Color::default(),
            diffuse: Color::default(),
            specular: Color::default(),
            light_direction: Vector3::default(),
            eye_pos: Vector3::default(),
        }
    }
}

pub struct FlatShader {
}

impl BaseShader for FlatShader {
    fn activate(&self) {
        println!("{}", "Activate FlatShader")
    }

    fn deactivate(&self) {
        println!("{}", "Deactivate FlatShader")
    }

    fn as_vec(&self, model: &Model, cam: &Camera) -> Vec<f32> {
        let v = [model.transform.get_transform().as_vec(), cam.transform.get_transform().as_vec(), cam.get_projection_matrix().as_vec()].concat();
        v
    }
}

impl Default for FlatShader {
    fn default() -> Self {
        Self {
        }
    }
}