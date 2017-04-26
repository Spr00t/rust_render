extern crate rand;

use geometry::*;
use geometry3::*;
use image::*;
use bridge::Application;
use array2d::*;
use rand::*;
use std::f32;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Object {
    pub vertex: Vec<(f32, f32, f32)>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl Object {
    pub fn new(v: Vec<(f32, f32, f32)>, t: Vec<(usize, usize, usize)>)
        -> Object
    {
        Object {
            vertex: v,
            triangles: t,
        }
    }
}
pub struct Scene {
    width: u32,
    height: u32,
    pub objects: Vec<Rc<RefCell<Object> > >,
    light_direction: Vec3<f32>,
    zbuffer: Rc<RefCell<Array2d<f32>>>,
}

impl Scene {

    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            width: width,
            height: height,
            objects: Vec::<Rc<RefCell<Object>>>::new(),
            light_direction: Vec3::<f32> {dx: 0., dy: 0., dz: 0.},
            zbuffer: Rc::new(RefCell::new(
                Array2d::<f32>::new(f32::MIN, width as usize, height as usize))),
        }
    }
    //pub fn get_zbuffer(&mut self) -> &mut Array2d<f32> {
    //    &mut self.zbuffer
    //}
    pub fn get_zbuffer(&self) -> Rc<RefCell<Array2d<f32>>> {
        self.zbuffer.clone()
    }
    pub fn add_object(&mut self, object: Rc<RefCell<Object>> ) {
        self.objects.push(object);
    }
}
