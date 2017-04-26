use std;
use std::mem::swap;
use std::collections::HashMap;
use std::f32;
use std::fmt::Display;
use std::cmp::{max, min};
use geometry::*;
use geometry3::*;
use geometry;
use array2d::*;
use std::convert::*;


#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ColorE {
    Green,
    Yellow,
    White,
    Black,
    Blue,
    Red,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Color {
    pub c: u32,
}
impl Color {
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self{
        let color: u32 = 0xff000000 | r << 16 | g << 8 | b;
        Color {
            c: color,
        }
    }
}
impl Convert<Color> for ColorE {
    fn convert(self) -> Color {
        let color_array = [
            (ColorE::Green, 0, 255, 0),
            (ColorE::Yellow, 255, 255, 0),
            (ColorE::White, 255, 255, 255),
            (ColorE::Black, 0, 0, 0),
            (ColorE::Blue, 0, 0, 255),
            (ColorE::Red, 255, 0, 0)
        ];

        let mut colors = HashMap::new();

        for x in color_array.iter() {
            colors.insert(x.0, (x.1, x.2, x.3));
        }
        let mut c: Color;
        if let Some(c) = colors.get(&self) {
            return Color::from_rgb(c.0, c.1, c.2);
        };
        Color::from_rgb(0, 0, 0)
    }
}



pub struct Image {
    pub data: Array2d<u32>,
    pub w: u32,
    pub h: u32,
}

impl Image {
    pub fn new<C>(w: u32, h: u32, color: C) -> Self
        where C: Convert<Color>
    {
        let data = Array2d::new(color.convert().c, w as usize, h as usize);
        Image {
            data: data,
            w: w,
            h: h
        }
    }
    pub fn draw_line<T1, T2, C>(&mut self, p1: T1, p2: T2, color: C,
        zbuffer: &mut Array2d<f32>)
        where T1: Convert<Point3<i32>>, T2: Convert<Point3<i32>>, C: Convert<Color> + Copy
    {
        let (p1_, p2_) = (p1.convert(), p2.convert());
        self.draw_line_c(p1_.x, p1_.y, p1_.z,
                         p2_.x, p2_.y, p2_.z,
                         color.convert(), zbuffer);
    }
    pub fn draw_line_c<C>(&mut self, x0_: i32, y0_: i32, z0_: i32, x1_: i32, y1_: i32, z1_: i32, color: C, zbuffer: &mut Array2d<f32>)
        where C: Convert<Color> + Copy
    {

        println!("draw_line x0={} y0={} z0={} x1={} y1={} z1={}",
                x0_, y0_, z0_, x1_, y1_, z0_);
        let (mut x0, mut y0, mut x1, mut y1)
            = (max(x0_, 0), max(y0_, 0), max(x1_, 0), max(y1_, 0));

        let (mut z0, mut z1) = (z0_, z1_);



        x1 = if x1 >= self.w as i32{ self.w as i32 - 1 } else { x1 };
        y1 = if y1 >= self.h as i32{ self.h as i32 - 1 } else { y1 };
        let mut steep = false;
        if (x0 as i32 - x1 as i32).abs() < (y0  as i32 - y1  as i32 ).abs() {
            println!("draw_line steep-1 x0={} y0={} z0={} x1={} y1={} z1={}",
                    x0_, y0_, z0_, x1_, y1_, z1_);
            swap(&mut x0, &mut y0);
            println!("draw_line steep-2 x0={} y0={} z0={} x1={} y1={} z1={}",
                    x0_, y0_, z0_, x1_, y1_, z1_);
            swap(&mut x1, &mut y1);
            println!("draw_line steep-3 x0={} y0={} z0={} x1={} y1={} z1={}",
                    x0_, y0_, z0_, x1_, y1_, z1_);
            steep = true;
        }
        println!("draw_line steep1 x0={} y0={} z0={} x1={} y1={} z1={}",
                x0_, y0_, z0_, x1_, y1_, z1_);
        if x0 >x1 { // make it left-to-right
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
            swap(&mut z0, &mut z1);
        }
        println!("draw_line steep2 x0={} y0={} z0={} x1={} y1={} z1={}",
                x0_, y0_, z0_, x1_, y1_, z1_);

        let delta_z =
            if x1 != x0 { (z1 - z0) / (x1 - x0) }
        else
            {0};

        let mut z = z0;
        for x in x0..x1+1 {
            let t = (x-x0) as f32/ (x1 - x0) as f32;
            let y = y0 as f32* (1.- t) + y1 as f32 * t;
            let w = self.w;
            z = z + delta_z;
            /*println!("t={} x={} y={} x0={} y0={} x1={} y1={} steep={} ",
                t, x, y, x0, y0, x1, y1, steep);*/
            if steep {
                self.draw_pixel(y as i32, x as i32, z, color.convert(), zbuffer);
            } else {
                self.draw_pixel(x as i32, y as i32, z, color.convert(), zbuffer);
            }
        }
    }


    pub fn draw_pixel(&mut self, x_: i32, y_: i32, z_: i32, color: Color, zbuffer: &mut Array2d<f32>) {
        let (x0, y0) = (max(x_, 0), max(y_, 0));
        let (x, y) = (min(self.w as i32 - 1, x0), min(self.h as i32, y0));
        //println!("draw pixel {} {}", x, y);
        if let Some(v) = self.data.get_mut(x, self.h as usize - y as usize - 1) {
            if let Some(zb) = zbuffer.get_mut(x, self.h as usize - y as usize - 1) {
                if (*zb as i32) < z_ {
                    *v = color.c;
                    *zb = z_ as f32;
                }
            }
        }
    }

    fn fill_down<T>(&mut self, t: T, color_: Color, fill_color_: Color, zbuffer: &mut Array2d<f32>)
        where T: geometry::Convert<Triangle3<i32>>
    {
        let mut tr = t.convert();

        if tr.a.x > tr.b.x {
            swap(&mut tr.a, &mut tr.b);
        }
        let (a, b, c): (Point3<i32>, Point3<i32>, Point3<i32>) = (tr.a, tr.b, tr.c);

        //println!("fill_down: a={} b={}, c={}", a, b, c);

        let x = a.x;
        let delta_left  = (c.x - a.x) as f32 / (c.y - a.y) as f32;
        let delta_right = (b.x - c.x) as f32 / (c.y - a.y) as f32;

        let delta_z_left  = (c.z - a.z) as f32 / (c.y - a.y) as f32;
        let delta_z_right = (b.z - c.z) as f32 / (c.y - a.y) as f32;

        //println!("fill_down: delta_left={} delta_right={}", delta_left, delta_right);
        let mut x_left = a.x as f32;
        let mut x_right = b.x as f32;

        let mut z_left = a.z as f32;
        let mut z_right = b.z as f32;

        let delta_z = (z_right - z_left) as f32 / (c.y - a.y) as f32;
        let mut z;
        for y in a.y..c.y {
            //println!("fill_down: y={} x_left={}, x_right={}", y, x_left, x_right);
            z = z_left;
            for x in x_left as i32..x_right as i32+1 {
                //println!("fill_down: x={} y={} x_left={}, x_right={}", x, y, x_left, x_right);
                z = z + delta_z;
                self.draw_pixel(x as i32, y as i32, z as i32, fill_color_.into(), zbuffer);
            }
            z_left = z_left + delta_z_left;
            x_left  = x_left + delta_left;
            x_right = x_right - delta_right;
        }
    }

    fn fill_up<T>(&mut self, t: T, color_: Color, fill_color_: Color, zbuffer: &mut Array2d<f32>)
        where T: Convert<Triangle3<i32>>
    {
        let mut tr = t.convert();

        if tr.b.x > tr.c.x {
            swap(&mut tr.b, &mut tr.c);
        }
        let (a, b, c) = (tr.a, tr.b, tr.c);

        //println!("fill_up: a={} b={}, c={}", a, b, c);

        let x = a.x;
        let delta_left  = (a.x - b.x) as f32 / (c.y - a.y) as f32;
        let delta_right = (c.x - a.x) as f32 / (c.y - a.y) as f32;


        let delta_z_left  = (a.z - b.z) as f32 / (c.y - a.y) as f32;
        let delta_z_right = (c.z - a.z) as f32 / (c.y - a.y) as f32;

        //println!("fill_up: delta_left={} delta_right={}", delta_left, delta_right);
        let mut x_left = x as f32;
        let mut x_right = x as f32;
        let mut z_left = b.z as f32;
        let mut z_right = c.z as f32;
        for y in a.y..b.y {
            //println!("fill_up:y={} x_left={}, x_right={}", y, x_left, x_right);
            let mut z = z_left;
            let diff_x = x_right - x_left;
            let delta_z = (z_right - z_left) /
                        if diff_x < 1e-8 {1.} else {diff_x}; //avoid division by zero
            for x in x_left as i32..x_right as i32+1 {
                //println!("fill_up:x={} y={} x_left={}, x_right={}", x, y, x_left, x_right);
                self.draw_pixel(x as i32, y as i32, z as i32, fill_color_.into(), zbuffer);
                z = z + delta_z;
            }
            z_left  = z_left  - delta_z_left;
            z_right = z_right + delta_z_left;

            x_left  = x_left - delta_left;
            x_right = x_right + delta_right;
        }
    }
    pub fn draw_triangle<T, C1, C2>(&mut self,
                                    t: T,
                                    color_: C1,
                                    fill_color_: C2,
                                    zbuffer: &mut Array2d<f32>)
        where T: Convert<Triangle3<i32>>, C1: Convert<Color>, C2: Convert<Color>
    {
        let mut triangle : Triangle3<i32> = t.convert();
        let color: Color = color_.convert();
        let fill: Color = fill_color_.convert();

        triangle.normalize_by_y();

        //let point_left;
        //let point_right;
        //println!("Ax={} Ay={} Bx={} By={} Cx={} Cy={}", triangle.a.x, triangle.a.y,
        //                                                triangle.b.x, triangle.b.y,
        //                                                triangle.c.x, triangle.c.y);

        let mut has_middle = false;
        let mut k = 0.;
        let mut middle = triangle.b; // after normalization b is in the middle
        let (a, b, c) = (triangle.a, triangle.b, triangle.c);

        if b.y != a.y {
            k = (c.y - b.y) as f64 / (b.y - a.y) as f64;
            middle.x = ((c.x as f64 + k * a.x as f64) / (1. as f64 + k) ) as i32;
            middle.z = ((c.z as f64 + k * a.z as f64) / (1. as f64 + k) ) as i32;
            //println!("k={} middle={}", k, x_middle);

            has_middle = true;
            self.draw_line(b, middle, color, zbuffer);
        }

        if has_middle {
            let tr : Triangle3<i32> = Triangle3::from(a, b, middle);
            self.fill_up(tr, color, fill, zbuffer);

            let tr : Triangle3<i32> = Triangle3::from(b, middle, c);
            self.fill_down(tr, color, fill, zbuffer);
        } else {
            self.fill_down( (a, b, c), color, fill, zbuffer);
        }

        // Draw perimeter
        self.draw_line(triangle.a, triangle.b, color, zbuffer);

        self.draw_line(triangle.b, triangle.c, color, zbuffer);

        self.draw_line(triangle.a, triangle.c, color, zbuffer);



    }

}



