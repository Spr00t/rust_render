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
        for rc_obj in &mut scn.objects {
            if let Some(obj) = rc_obj.try_borrow_mut().ok() {
                let (triangles, vertex) = (&obj.triangles, &obj.vertex);
                draw_internal(&mut self.image, triangles, vertex, scn.get_zbuffer());
            }
        }
    }
    /*
    pub fn draw(&mut self, triangles: & Vec<(usize, usize, usize)>, vertex: & Vec<(f32, f32, f32)>) {
        draw_internal(&mut self.image, triangles, vertex,  &mut scn.get_zbuffer());
    }
    */
}

pub fn draw_internal(image: &mut Image,
                     triangles: & Vec<(usize, usize, usize)>,
                     vertex: & Vec<(f32, f32, f32)>,
                     zbuffer: &mut Array2d<f32>)
{
    println!("draw_internal");
    for triangle in triangles.into_iter() {
        let (a, b, c) = *triangle;

        if let Some(v1) = vertex.get(a - 1) {
            if let Some(v2) = vertex.get(b - 1) {
                if let Some(v3) = vertex.get(c - 1) {
                    let p3: Point3<f32> = (*v3).convert();
                    let p2: Point3<f32> = (*v2).convert();

                    let mut vec_3d: Vec3<f32> = ( p3 - (*v1).convert() ) ^
                                            ( p2 - (*v1).convert() );

                    println!("before normalize vec3d {}", vec_3d);
                    vec_3d.normalize();

                    let mut light_direction = Vec3::<f32> { dx: 0., dy: 0., dz: -1. };

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

                        //let fill_color = Color::from_rgb(rand::random::<u32>() % 255, random::<u32>() % 255, random::<u32>() % 255);
                        let color_value = (255. * intensity.abs()) as u32;
                        println!("color_value {}", color_value);
                        if true {
                            let fill_color = Color::from_rgb(color_value, color_value, color_value);
                            image.draw_triangle(triangle, fill_color, fill_color, zbuffer);
                        }
                    }

                }
            }
        }
    }
}

