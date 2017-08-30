use std::mem::swap;
use std::collections::HashMap;
use std::f32;
use geometry::*;
use geometry3::*;

use array2d::*;

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
    pub fn from_buffer(w: u32, h: u32, array: Array2d<u32>) -> Self
    {

        Image {
            data: array,
            w: w,
            h: h
        }
    }
    pub fn draw_line<T1, T2, C>(&mut self, p1: T1, p2: T2, color: C,
        zbuffer: &mut Array2d<f32>)
        where T1: Convert<Point3<f32>>, T2: Convert<Point3<f32>>, C: Convert<Color> + Copy
    {
        let (p1_, p2_) = (p1.convert(), p2.convert());
        self.draw_line_c(p1_.x, p1_.y, p1_.z,
                         p2_.x, p2_.y, p2_.z,
                         color.convert(), zbuffer);
    }
    pub fn draw_line_c<C>(&mut self, x0_: f32, y0_: f32, z0_: f32, x1_: f32, y1_: f32, z1_: f32, color: C, zbuffer: &mut Array2d<f32>)
        where C: Convert<Color> + Copy
    {

//        println!("draw_line x0={} y0={} z0={} x1={} y1={} z1={}",
//                x0_, y0_, z0_, x1_, y1_, z0_);
        let (mut x0, mut y0, mut x1, mut y1)
            = (mx(x0_, 0.),
                mx(y0_, 0.),
                mx(x1_, 0.),
                mx(y1_, 0.));

        let (mut z0, mut z1) = (z0_, z1_);



        x1 = if x1 >= self.w as f32 { self.w as f32 - 1. } else { x1 };
        y1 = if y1 >= self.h as f32 { self.h as f32 - 1. } else { y1 };
        let mut steep = false;
        if (x0 as i32 - x1 as i32).abs() < (y0  as i32 - y1  as i32 ).abs() {
            //println!("draw_line steep-1 x0={} y0={} z0={} x1={} y1={} z1={}",
            //        x0_, y0_, z0_, x1_, y1_, z1_);
            swap(&mut x0, &mut y0);
            //println!("draw_line steep-2 x0={} y0={} z0={} x1={} y1={} z1={}",
            //        x0_, y0_, z0_, x1_, y1_, z1_);
            swap(&mut x1, &mut y1);
            //println!("draw_line steep-3 x0={} y0={} z0={} x1={} y1={} z1={}",
            //        x0_, y0_, z0_, x1_, y1_, z1_);
            steep = true;
        }
//        println!("draw_line steep1 x0={} y0={} z0={} x1={} y1={} z1={}",
//                x0_, y0_, z0_, x1_, y1_, z1_);
        if x0 >x1 { // make it left-to-right
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
            swap(&mut z0, &mut z1);
        }
//        println!("draw_line steep2 x0={} y0={} z0={} x1={} y1={} z1={}",
//                x0_, y0_, z0_, x1_, y1_, z1_);

        let delta_z =
            if x1 != x0 { (z1 - z0) / (x1 - x0) }
        else
            {0.};

        let mut z = z0;
        for x in x0.round() as i32 ..(x1+1.0).round() as i32 {
            let t = (x as f32-x0) as f32/ (x1 - x0) as f32;
            let y = y0 as f32* (1.- t) + y1 as f32 * t;
            z = z + delta_z;
            println!("t={} x={} y={} x0={} y0={} x1={} y1={} steep={} ",
                t, x, y, x0, y0, x1, y1, steep);
            if steep {
                self.draw_pixel(y, x as f32, z, color.convert(), zbuffer);
            } else {
                self.draw_pixel(x as f32, y, z, color.convert(), zbuffer);
            }
        }
    }


    pub fn draw_pixel<C>(&mut self, x_: f32, y_: f32, z_: f32, color: C, zbuffer: &mut Array2d<f32>)
        where C: Convert<Color> + Copy
    {
        let (x, y) = (x_.round() as i32, y_.round() as i32);
        if x < 0 || x > self.w as i32 - 1 || y <= 0 || y > self.h as i32 - 1 {
            return
        }
        //if x < 5 && y > 500 {panic!();}

        //println!("draw pixel {} {}", x, y);
        if let Some(v) = self.data.get_mut(x, self.h as usize - y as usize - 1) {
            if let Some(zb) = zbuffer.get_mut(x, self.h as usize - y as usize - 1) {
                if (*zb) < z_ {
                    *v = color.convert().c;
                    *zb = z_;
                } else {
                    println!("draw pixel not succeeded: {} {} {} zbuffer={}", x, y, z_, (*zb));
                }
            }
        }

    }

    fn fill_bottom(&mut self,
        xtop: f32,
        ytop: f32,
        ztop: f32,

        mut xleftbottom: f32,
        mut xrightbottom: f32,
        ybottom: f32,
        mut zleftbottom: f32,
        mut zrightbottom: f32,
        _color: Color, fill_color_: Color, zbuffer: &mut Array2d<f32>)
    {
        let direction = if ytop < ybottom { 1 } else {-1};

        println!("fillup start");
        if xleftbottom > xrightbottom {
            swap(&mut xleftbottom, &mut xrightbottom);
            swap(&mut zleftbottom, &mut zrightbottom);
        }

        println!("xtop={}\nytop={}\nztop={}\nxleftbottom={}\nxrightbottom={}\nybottom={}\nzleftbottom={}\nzrightbottom={}\n",
            xtop, ytop, ztop,
            xleftbottom,
            xrightbottom,
            ybottom,
            zleftbottom,
            zrightbottom,
            );

        let diff_y = -ybottom.round()  + ytop.round();

        let (delta_zleft, delta_zright) = if ytop - ybottom == 0. { (0., 0.) } else {
            ((ztop - zleftbottom) / (direction as f32 * diff_y),
             (ztop - zrightbottom) / (direction as f32 * diff_y))
        };

        let mut zleft  = ztop;
        let mut zright = ztop;

        let mut xleft  = xtop;
        let mut xright = xtop;

        println!("diff_y={} ", diff_y);

        let (delta_xleft, delta_xright) = if ytop - ybottom == 0. { (0., 0.) } else {
            ((xtop - xleftbottom) / (direction as f32 * diff_y),
             (xtop - xrightbottom) / (direction as f32  * diff_y))
        };

        println!("delta_xleft={} delta_xright={}", delta_xleft, delta_xright);

        let mut y: i32 = ytop.round() as i32;
        while direction * y < direction * ybottom.round() as i32 + 1{

            let mut z = zleft;
            let delta_z = if xleft == xright
                {0.}
            else
                { (zright - zleft) / (xright - xleft) };


            println!("y={}\nxleft={} xright={}\nzleft={} zright{}", y, xleft, xright, zleft, zright);

            for x in xleft.round() as i32..xright.round() as i32+1 {
                println!("pixel x={} y={} z={}", x, y, z);
                self.draw_pixel(x as f32, y as f32, z, fill_color_, zbuffer);
                z = z + delta_z
            }
            zleft  = zleft + delta_zleft;
            zright = zright + delta_zright;

            xleft  = xleft + delta_xleft;
            xright = xright + delta_xright;
            y = y + direction;
        }
        println!("xleft={} xright={}", xleft, xright);
        println!("zleft={} zright={}", zleft, zright);
        println!("fillup end");
    }
    pub fn draw_triangle<T, C1, C2>(&mut self,
                                    t: T,
                                    color_: C1,
                                    fill_color_: C2,
                                    zbuffer: &mut Array2d<f32>,
                                    _texture: Option<&Image>)
        where T: Convert<Triangle3<f32>>, C1: Convert<Color>, C2: Convert<Color>
    {
        let mut triangle : Triangle3<f32> = t.convert();
        let color: Color = color_.convert();
        let fill: Color = fill_color_.convert();

        triangle.normalize_by_y();

        //let point_left;
        //let point_right;

        println!("draw_triangle: \nAx={} Ay={} Zy={}\nBx={} By={} Bz={}\nCx={} Cy={} Cz={}",
                 triangle.a.x, triangle.a.y, triangle.a.y,
                 triangle.b.x, triangle.b.y, triangle.b.z,
                 triangle.c.x, triangle.c.y, triangle.c.z);

        let mut has_middle = false;
        let k;
        let mut middle = triangle.b; // after normalization b is in the middle
        let (a, b, c) = (triangle.a, triangle.b, triangle.c);

        if b.y != a.y {
            k = (c.y - b.y) as f32 / (b.y - a.y) as f32;
            middle.x = (c.x as f32 + k * a.x as f32) / (1. as f32 + k);
            middle.z = (c.z as f32 + k * a.z as f32) / (1. as f32 + k);
            //println!("k={} middle={}", k, x_middle);

            has_middle = true;
            self.draw_line(b, middle, color, zbuffer);
        }

        if has_middle {
            self.fill_bottom(a.x, a.y, a.z,
                         b.x, middle.x,
                         b.y,
                         b.z, middle.z,
                color, fill, zbuffer);

            self.fill_bottom(c.x, c.y, c.z,
                middle.x, b.x,
                b.y,
                middle.z, b.z,
                color, fill, zbuffer);
        } else {
        self.fill_bottom(c.x, c.y, c.z,
                     b.x, c.x,
                     b.y,
                     b.x, c.x,
            color, fill, zbuffer);
        }

        // Draw perimeter
        println!("{} ab", line!());
        self.draw_line(triangle.a, triangle.b, color, zbuffer);

        println!("{} bc", line!());
        self.draw_line(triangle.b, triangle.c, color, zbuffer);

        println!("{} ac", line!());
        self.draw_line(triangle.a, triangle.c, color, zbuffer);



    }

}



