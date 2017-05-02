extern crate libc;
extern crate rand;

mod bridge;
mod image;
mod geometry;
mod geometry3;
mod render;
mod scene;
mod array2d;
mod tgacanvas;

#[macro_use]
extern crate log;

use geometry::*;
use geometry3::*;
use image::*;
use bridge::Application;
use render::*;
use rand::*;
use std::rc::Rc;
use std::cell::RefCell;

//use scene;

use std::fs::File;
use std:: env;
use std::f32;
use std::cmp::*;
use std::io::{BufReader};
use std::io::prelude::*;
use tgacanvas::*;



fn getTuple3_h<T: Clone>(nums: Vec<T>) -> Result<(T, T, T), i32> {
    let a = try!(nums.get(0).ok_or(0));
    let b = try!(nums.get(1).ok_or(0));
    let c = try!(nums.get(2).ok_or(0));
    Ok ( (a.clone(), b.clone(), c.clone()) )
}
trait GetTuple3 {
    type Item;
    fn getTuple3(self) -> Option<(Self::Item, Self::Item, Self::Item)>;
}


impl <T: Clone> GetTuple3 for Vec<T> {
    type Item = T;
    fn getTuple3(self) -> Option<(T, T, T)> {
        getTuple3_h(self).ok()
    }
}

//fn getVert(nums: & Vec<f32>) -> Result<(&f32, &f32, &f32), i32> {
//    let (a, b, c) : (&f32, &f32, &f32) =  ( (try!(nums.get(0).ok_or(0)), try!(nums.get(1).ok_or(0)), try!(nums.get(2).ok_or(0))) )
//}


fn read_obj<'a, T: BufRead>(reader: &'a mut T,
        vertexs: &'a mut Vec<(f32, f32, f32)>,
        triangles: &'a mut Vec<(usize, usize, usize)>
        ) {


    let lines = reader.lines();
    //println!("Parsing {}", lines.clone().count());

    let mut line_number = 0;
    for line in lines.filter_map(|line| line.ok()).into_iter() {
            //println!("got line: {}", line);

            let mut words = line.split(" ");
            line_number = line_number + 1;
            if let Some(first) = words.next() {
                match first {
                    "v" => {
                        //let numbers = line.split(" ").skip(1).take(3).filter_map(|v| v.parse::<f32>().ok()).collect::<Vec<f32>>();
                        let numbers = words
                            .take(3)
                            .map(|v| match v.parse::<f32>() {
                                Ok(x) => x,
                                Err(e) => {panic!("parsing error at line {}, reason={}", line_number, e); }
                                })
                            .collect::<Vec<f32>>();
                        if let Some(t) = numbers.getTuple3() {
                            println!("Add new vertex {} {} {}", t.0, t.1, t.2 );
                            vertexs.push(t)
                        };
                    },
                    "f" => {
                        print!("");
                        let numbers = words
                            .take(3)
                            .filter_map(|v| {
                                v.split('/')
                                    .next()
                                    .and_then(|n| n.parse::<usize>().ok())
                            })
                            .collect::<Vec<usize>>();

                        if let Some(t) = numbers.getTuple3() {
                            //println!("Add new triangle {} {} {}", t.0, t.1, t.2 );
                            triangles.push(t)
                        };
                    },
                    _ => ()
                }
            } else {
                continue;
            }
      }

      let mut i: usize = 0;
      vertexs.into_iter().inspect(|&&mut v| {
        let (a,b,c) = v; //println!("vertex {}: {} {} {}", i, a, b, c); i = i + 1;
      }).count();

      triangles.into_iter().inspect(|v| {
          //println!("triangle : {} {} {}", v.0, v.1, v.2);
      }).count();

}
fn main() {
    println!("Starting");

    let mut app = Application::new();

    let diffuse_map = TgaCanvas::read("data/african_head_diffuse.tga");

    println!("Open file");
    let f = File::open(
            &env::args().last()
                .unwrap()).unwrap();

    let mut reader = BufReader::new(f);
    let mut vertex = Vec::<(f32, f32, f32)>::new();
    let mut triangles = Vec::<(usize, usize, usize)>::new();

    read_obj(&mut reader, &mut vertex, &mut triangles);

    let (mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN, f32::MAX, f32::MIN);

    for e in vertex.iter() {
        if e.0 < xmin {
            xmin = e.0;
        }
        if e.0 > xmax{
            xmax = e.0;
        }
        if e.1 < ymin {
            ymin = e.1;
        }
        if e.1 > ymax {
            ymax = e.1;
        }
        if e.2 < zmin {
            zmin = e.2;
        }
        if e.2 > zmax {
            zmax = e.2;
        }
    }

    println!("xmin={} xmax={} ymin={} ymax={}", xmin, xmax, ymin, ymax);


    let w: u32 = 800;
    let h: u32 = 800;

    let mut image = Image::new(w , h, ColorE::Black);




    //image.draw_line( (0, 6), (w as i32, 10), ColorE::Green);
    let mut counter = 0;

    let mut scn = scene::Scene::new(w, h);
    {
        let mut zbuffer = scn.get_zbuffer();
//        {
//            let triangle = Triangle3::from(
//                                            (288.91104, 668.4588, 668.4588),
//                                            (265.33224, 668.5404, 475.0368),
//                                            (265.31586, 711.676, 445.1092));
//            image.draw_triangle(triangle, ColorE::Green, ColorE::Green, &mut zbuffer.borrow_mut());
//        }

//        let mut zbuffer = scn.get_zbuffer();
//        {
//            let triangle = Triangle3::from(
//                                            (100, 50, 0),
//                                            (150, 100, 0),
//                                            (100, 100, 0));
//            image.draw_triangle(triangle, ColorE::Green, ColorE::Green, &mut zbuffer.borrow_mut());
//        }

//        let mut zbuffer = scn.get_zbuffer();
//        {
//            let triangle = Triangle3::from(
//                                            (300, 50, 0),
//                                            (300, 100, 0),
//                                            (250, 100, 0));
//            image.draw_triangle(triangle, ColorE::Green, ColorE::Green, &mut zbuffer.borrow_mut());
//        }


//        let mut zbuffer = scn.get_zbuffer();
//        {
//            let triangle = Triangle3::from(
//                                            (200, 50, 0),
//                                            (200, 150, 0),
//                                            (150, 100, 0));
//            image.draw_triangle(triangle, ColorE::Green, ColorE::Green, &mut zbuffer.borrow_mut());
//        }

//        let mut zbuffer = scn.get_zbuffer();
//        {
//            let triangle = Triangle3::from(
//                                            (500, 150, 0),
//                                            (550, 200, 0),
//                                            (500, 50, 0));
//            image.draw_triangle(triangle, ColorE::Green, ColorE::Green, &mut zbuffer.borrow_mut());
//        }



//        {
//            let triangle = Triangle3::from((200, 400, 150),
//                                           (600, 600, 200),
//                                           (600, 200, 200));
//            image.draw_triangle(triangle, ColorE::White, ColorE::White, &mut zbuffer.borrow_mut());
//        }

//    image.draw_line((200., 300., 400.),
//                    (600., 300., 300.), ColorE::Red, &mut zbuffer.borrow_mut());


    for y in 200..205+1 {
        image.draw_line((250., y as f32, 200.),
                        (200., y as f32, 150.), ColorE::Blue, &mut zbuffer.borrow_mut());
    }

        //delta = -delta;
        
        //let triangle = Triangle3::from((600 - delta, 400 - delta, 0), (600 - delta, 200 - delta, 0), (200 - delta, 750 - delta, 0));
        //image.draw_triangle(triangle, ColorE::Red, ColorE::Red, &mut zbuffer.borrow_mut());
    }
    let mut blue = 0;

    let mut renderer = Render::new(image);


    if true {
        let mut counter = 0;
        for ver in &mut vertex {
            let xf = w as f32 / (xmax - xmin);
            let yf = h as f32 / (ymax - ymin);

            let factor = if xf < yf {xf} else {yf};

            ver.0 = ((ver.0 as f32 - xmin as f32) * factor) as f32;
            ver.1 = ((ver.1 as f32 - ymin as f32) * factor) as f32;
            ver.2 = ((ver.2 as f32 - zmin as f32) * factor) as f32;
            counter = counter + 1;
        }
        println!("There are {} vertex", counter);

        let object = scene::Object::new(vertex, triangles);
        scn.add_object(Rc::new(RefCell::new(object)));

        //scn.render(&mut image);
        //draw(&mut image, &triangles, &vertex);
    }
    renderer.draw_scene(&mut scn);

    app.show_image(&renderer.image);

    println!("Launching widget: ");
    app.run();
}
