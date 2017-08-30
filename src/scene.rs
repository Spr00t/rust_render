extern crate rand;

use geometry3::*;
use image::*;
use array2d::*;

use std::f32;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct Face {
    pub v_index: (usize, usize, usize),
    pub vt_index: (usize, usize, usize)
}

pub struct Model {
    pub vertexs: Vec<(f32, f32, f32)>,
    pub texture_vertexs: Vec<(f32, f32, f32)>,
    pub faces: Vec<Face>,
    pub texture_id: u32
}
#[allow(dead_code)]
pub struct Scene {
    width: u32,
    height: u32,
    objects: Vec<Rc<RefCell<Model> > >,
    light_direction: Vec3<f32>,
    zbuffer: Rc<RefCell<Array2d<f32>>>,
    textures_map: BTreeMap<u32, Rc<RefCell<Image>>>,
    counter: u32,

}



impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            width: width,
            height: height,
            objects: Vec::<Rc<RefCell<Model>>>::new(),
            light_direction: Vec3::<f32> {dx: 0., dy: 0., dz: 0.},
            zbuffer: Rc::new(RefCell::new(
                Array2d::<f32>::new(f32::MIN, width as usize, height as usize))),
            textures_map: BTreeMap::new(),
            counter: 1
        }
    }
    
    pub fn get_zbuffer(&self) -> Rc<RefCell<Array2d<f32>>> {
        self.zbuffer.clone()
    }
    pub fn add_object(&mut self, object: Rc<RefCell<Model>> ) {
        self.objects.push(object);
    }
    pub fn get_objects(&mut self) -> &mut Vec<Rc<RefCell<Model> > >{
        &mut self.objects
    }
    pub fn register_resourse(&mut self, img: Rc<RefCell<Image>>) -> u32 {
        self.counter = self.counter + 1;
        self.textures_map.insert(self.counter, img);
        self.counter
    }
    pub fn get_texture(&self, id: u32) -> Option<Rc<RefCell<Image>>> {
        /*
        let x1 : Option<&Rc<RefCell<Image>>> = self.textures_map.get(&id);
        let x2 : &Rc<RefCell<Image>> = get!(x1);
        let x3 : std::cell::Ref<Image> = get!(x2.try_borrow().ok());
        Some(x3.deref())
        */
        if let Some(res) = self.textures_map.get(&id) {
            return Some(res.clone());
        }
        None
        //let x3 : & Image = get!(x2.try_borrow().ok());
        
        //x3
    }
    

}
