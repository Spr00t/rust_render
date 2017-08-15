extern crate rand;

use geometry::*;
use geometry3::*;
use image::*;
use bridge::Application;
use scene::*;
use array2d::*;
use rand::*;

use std::fs::File;
use std:: env;
use std::f32;
use std::cmp::*;
use std::io::{BufReader};
use std::io::prelude::*;
use std::rc;
use std::rc::*;
use std::cell::RefCell;



pub struct Render
{
    pub image: Image
}
impl Render {
    pub fn new(img: Image) -> Render {
        Render {
            image: img
        }
    }
    pub fn draw_scene(&mut self, scn: &mut Scene) {
        let mut zbuffer = scn.get_zbuffer();
        let count = scn.get_objects().len();
        for i in 0..count {
            //let rc_obj = scn.get_objects()[i].clone();
            let opt_object: Rc<RefCell<Model>>;
            {
                opt_object = scn.get_objects()[i].clone();
            }
            let obj = opt_object.try_borrow().ok().unwrap();
            {   
                draw_internal(&mut self.image, &obj, &mut zbuffer.borrow_mut(), 
                    scn.get_texture(obj.texture_id));
            }
        }
    }
}
pub fn get_texture_color(intensity: f32, 
                        coords: Triangle3<f32>, 
                        coords_t: Option<Triangle3<f32>>,
                        texture: Option<Rc<RefCell<Image>>>) -> f32{
    return intensity.abs();
}
pub fn draw_internal(image: &mut Image,
                     model: & Model,
                     zbuffer: &mut Array2d<f32>,
                     texture: Option<Rc<RefCell<Image>>>)
{
    println!("draw_internal");
    for face in model.faces.iter() {
        let (a, b, c) = face.v_index;

        if let Some(v1) = model.vertexs.get(a - 1) {
            if let Some(v2) = model.vertexs.get(b - 1) {
                if let Some(v3) = model.vertexs.get(c - 1) {
                    let p3: Point3<f32> = (*v3).convert();
                    let p2: Point3<f32> = (*v2).convert();

                    let mut vec_3d: Vec3<f32> = ( p3 - (*v1).convert() ) ^
                                            ( p2 - (*v1).convert() );

                    println!("before normalize vec3d {}", vec_3d);
                    vec_3d.normalize();

                    let mut light_direction = Vec3::<f32> { dx: 0.5, dy: 0., dz: -1. };

                    light_direction.normalize();

                    let intensity: f32 = vec_3d * light_direction;

                    println!("after normalize vec3d {}", vec_3d);
                    println!("intensity {}", intensity);

                    let mut m: f32 = if vec_3d.dx.partial_cmp(&vec_3d.dy) == Some(Ordering::Less) {
                        vec_3d.dx
                    } else {
                        vec_3d.dy
                    };
                    m =  if m.partial_cmp(&vec_3d.dz) == Some(Ordering::Less) {
                        m
                    } else {
                        vec_3d.dz
                    };

                    if intensity < 0. {
                        let triangle = Triangle3::from((v1.0, v1.1, v1.2), (v2.0, v2.1, v2.2), (v3.0, v3.1, v3.2));
                        let (a_t, b_t, c_t) = face.vt_index;

                        let mut triangle_texture: Option<Triangle3<f32>> = None;
                        if let (Some(&v1_t), Some(&v2_t), Some(&v3_t)) 
                                = (model.texture_vertexs.get(a_t), 
                                    model.texture_vertexs.get(b_t),  
                                    model.texture_vertexs.get(c_t)) {
                            triangle_texture = Some(Triangle3::<f32> {
                                a: v1_t.convert(), b: v2_t.convert(), c: v3_t.convert()
                            });
                        } 
                        //let fill_color = Color::from_rgb(rand::random::<u32>() % 255, random::<u32>() % 255, random::<u32>() % 255);
                        let color_value = (255. * get_texture_color(intensity, triangle, triangle_texture, texture.clone())) as u32;
                        println!("color_value {}", color_value);
                        if true {
                            let fill_color = Color::from_rgb(color_value, color_value, color_value);
                            image.draw_triangle(triangle, fill_color, fill_color, zbuffer, None);
                        }
                    }

                }
            }
        }
    }
}

