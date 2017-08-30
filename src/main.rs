extern crate libc;
extern crate rand;
extern crate imagefmt;

mod bridge;
mod image;

#[macro_use]
mod geometry;

mod geometry3;
mod render;
mod scene;
mod array2d;
mod matrix;

extern crate log;
use std::path::Path;

use image::*;
use bridge::Application;
use render::*;
use std::rc::Rc;
use std::cell::RefCell;
use scene::Face;
use scene::Model;
//use scene;

use std::fs::File;
use std:: env;
use std::f32;
use std::io::{BufReader};
use std::io::prelude::*;
use imagefmt::{ColFmt};
use array2d::*;

trait GetTuple3 {
    type Item;
    fn get_tuple3(self) -> Option<(Self::Item, Self::Item, Self::Item)>;
}
trait GetTuple2 {
    type Item;
    fn get_tuple2(self) -> Option<(Self::Item, Self::Item)>;
}
impl <T: Clone> GetTuple2 for Vec<T> {
    type Item = T;
    fn get_tuple2(self) -> Option<(T, T)> {
        if let (Some(a), Some(b)) = (self.get(0), self.get(1)) {
            return Some((a.clone(), b.clone()));
        }
        None
    }
}

impl <T: Clone> GetTuple3 for Vec<T> {
    type Item = T;
    fn get_tuple3(self) -> Option<(T, T, T)> {
        let a = get!(self.get(0));
        let b = get!(self.get(1));
        let c = get!(self.get(2));
        Some ( (a.clone(), b.clone(), c.clone()) )
    }
}

//fn getVert(nums: & Vec<f32>) -> Result<(&f32, &f32, &f32), i32> {
//    let (a, b, c) : (&f32, &f32, &f32) =  ( (try!(nums.get(0).ok_or(0)), try!(nums.get(1).ok_or(0)), try!(nums.get(2).ok_or(0))) )
//}


fn read_obj<'a, T: BufRead>(reader: &'a mut T,
                            model: &mut Model) {

    let lines = reader.lines();
    //println!("Parsing {}", lines.clone().count());

    let mut line_number = 0;
    for line in lines.filter_map(|line| line.ok()).into_iter() {
            //println!("got line: {}", line);
            let mut words = line.split(" ");
            line_number = line_number + 1;


            if let Some(first) = words.next() {
                let b_split = RefCell::new(words);

                let read_coord = || -> Option<(f32, f32, f32)> {
                    let mut words_c = b_split.borrow_mut();
                    let numbers = words_c.by_ref()
                        .filter(|&c| c != "")
                        .take(3)
                        .map(|v| match v.parse::<f32>() {
                            Ok(x) => x,
                            Err(e) => {
                                println!("first word {}", first);
                                panic!("parsing error at line {}, reason={}", line_number, e); }
                            })
                        .collect::<Vec<f32>>();
                    if let Some(t) = numbers.get_tuple3() {
                        println!("Add new vertex {} {} {}", t.0, t.1, t.2 );
                        return Some(t)
                    };
                    Some( (0., 0., 0.) )
                };
                match first {
                    "v" => {
                        let coord = read_coord();
                        if let Some(c) = coord {
                            model.vertexs.push(c);
                        }
                    },
                    "vt" => {
                        let coord = read_coord();
                        if let Some(c) = coord {
                            model.texture_vertexs.push(c);
                        }
                    },
                    "f" => {
                        print!("");
                        let mut words_c = b_split.borrow_mut();
                        let numbers = words_c.by_ref()
                            .take(3)
                            .filter_map(|v| {
                                v.split('/')
                                    .take(2)
                                    .filter_map(|v| v.parse::<usize>().ok()) 
                                    .collect::<Vec<usize>>()
                                    .get_tuple2()
                            })
                            .collect::<Vec<(usize, usize)>>();
                        if let Some(t) = numbers.get_tuple3() {
                            //println!("Add new triangle {} {} {}", t.0, t.1, t.2 );
                            let face = Face {
                                v_index:  ((t.0).0, (t.1).0, (t.2).0),
                                vt_index: ((t.0).1, (t.1).1, (t.2).1),
                            };
                            model.faces.push(face);
                        };
                    },
                    _ => ()
                }
            } else {
                continue;
            }
      }

//      let i: usize = 0;
//      model.vertexs.into_iter().inspect(|& v| {
//        let (a,b,c) = v; //println!("vertex {}: {} {} {}", i, a, b, c); i = i + 1;
//      }).count();

//      model.faces.into_iter().inspect(|f| {
//          //println!("triangle : {} {} {}", v.0, v.1, v.2);
//      }).count();

}
fn parse_img(img: &imagefmt::Image<u8> ) -> Image
{
    let mut buffer = Array2d::<u32>::new(0, img.w as usize, img.h as usize);
    for y in 0..img.h {
        for x in 0..img.w {
            let offset = y * img.w + x;
            let color =
                Color::from_rgb(img.buf[3 * offset + 0] as u32,
                                img.buf[3 * offset + 1] as u32,
                                img.buf[3 * offset + 2] as u32);
            if let Some(p) = buffer.get_mut(x, y) {
                *p = color.c;
            }
        }
    }

    Image::from_buffer(img.w as u32, img.h as u32, buffer)
}
fn main() {
    println!("Starting");

    let path_buf: std::path::PathBuf = std::env::current_dir().unwrap();

    println!("{}", path_buf.to_str().unwrap());

    let mut app = Application::new();
    

    let diffuse_map =
        imagefmt::read(Path::new("data/african_head_diffuse.tga"),
        ColFmt::RGB).unwrap();
    let diffuse_map_img: Image = parse_img(&diffuse_map);

    println!("Open file");
    let f = File::open(
            &env::args().last()
                .unwrap()).unwrap();

    let mut reader = BufReader::new(f);

    let mut model = Model {
        vertexs: Vec::new(),
        texture_vertexs: Vec::new(),
        faces: Vec::new(),
        texture_id: 0
    };
    read_obj(&mut reader, &mut model);

    let (mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN, f32::MAX, f32::MIN);

    for e in model.vertexs.iter() {
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

    let mut scn = scene::Scene::new(w, h);
    {
        let zbuffer = scn.get_zbuffer();
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

    let mut renderer = Render::new(image);

    if true {
        let mut counter = 0;
        for ver in &mut model.vertexs {
            let xf = w as f32 / (xmax - xmin);
            let yf = h as f32 / (ymax - ymin);

            let factor = if xf < yf {xf} else {yf};

            ver.0 = ((ver.0 as f32 - xmin as f32) * factor) as f32;
            ver.1 = ((ver.1 as f32 - ymin as f32) * factor) as f32;
            ver.2 = ((ver.2 as f32 - zmin as f32) * factor) as f32;
            counter = counter + 1;
        }
        println!("There are {} vertex", counter);


        scn.add_object(Rc::new(RefCell::new(model)));

        //scn.render(&mut image);
        
    }
    scn.register_resourse(Rc::new(RefCell::new(diffuse_map_img)));
    renderer.draw_scene(&mut scn);

    app.show_image(&renderer.image);
    //app.show_image(&diffuse_map_img);
    

    println!("Launching widget: ");
    app.run();
}
