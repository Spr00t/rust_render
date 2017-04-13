use geometry::*;
use std::ops::*;
use std::mem::swap;
use std::fmt::{Display, Formatter, Result};
use std::fmt;
use std::cmp::Ordering::*;
pub trait Sqr<T> {
    fn sqr(self) -> T;
}
impl Sqr<f32> for f32 {
    fn sqr(self) -> f32 {
        self.sqrt()
    }
}
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vec3<T>
    where T: Copy + Clone
{
    pub dx: T,
    pub dy: T,
    pub dz: T,
}
impl<T> Display for Vec3<T>
    where T: Display + Copy + Clone
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.dx, self.dy, self.dz)
    }
}
impl<T> Vec3<T>
    where T: Copy + Clone + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sqr<T>
{
    pub fn from<T1, T2, T3>(_dx: T1, _dy: T2, _dz: T3) -> Vec3<T>
        where T1: Convert<T>, T2: Convert<T>, T3: Convert<T>
    {
        Vec3::<T> {
            dx: _dx.convert(),
            dy: _dy.convert(),
            dz: _dz.convert()
        }
    }
    pub fn norm (&self) -> T
    {
        (self.dx*self.dx+self.dy*self.dy+self.dz*self.dz).sqr()
    }
    pub fn normalize(&mut self)
    {
        let n: T = self.norm();
        self.dx = self.dx / n;
        self.dy = self.dy / n;
        self.dz = self.dz / n;
    }
}
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point3<T>
    where T: Copy + Clone
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Convert<Point3<T> > for (T, T, T)
    where T: Copy + Clone
{
    fn convert(self) -> Point3<T> {
        Point3 {x: self.0, y: self.1, z: self.2 }
    }
}

impl Convert<Point3<f32> > for Point3<i32>
{
    fn convert(self) -> Point3<f32> { Point3 {x: self.x as f32, y: self.y as f32, z: self.z as f32} }
}

impl Convert<Point3<i32> > for Point3<f32>
{
    fn convert(self) -> Point3<i32> { Point3 {x: self.x as i32, y: self.y as i32, z: self.z as i32} }
}

impl<T> Sub for Point3<T>
    where T: Sub<Output=T> + Copy
{
    type Output = Vec3<T>;
    fn sub(self, rhs: Self) -> Vec3<T> {
        Vec3::<T> {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
            dz: self.z - rhs.z,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Line3<T>
    where T: Copy + Clone
{
    pub a: Point3<T>,
    pub b: Point3<T>,
}


impl<T> BitXor for Vec3<T>
    where T: Copy + Mul<Output=T> + Sub<Output=T>
{
    type Output = Vec3<T>;
    fn bitxor(self, rhs: Self) -> Vec3<T>
    {
        Vec3::<T>
        {
            dx: self.dz * rhs.dy - self.dy * rhs.dz,
            dy: self.dx * rhs.dz - self.dz * rhs.dx,
            dz: self.dy * rhs.dx - self.dx * rhs.dy,
        }
    }
}

impl<T> Mul for Vec3<T>
    where T: Copy + Mul<Output=T> + Add<Output=T>
{
    type Output = T;
    fn  mul(self, rhs: Self) -> T
    {
        self.dz * rhs.dz + self.dy * rhs.dy + self.dx * rhs.dx
    }
}

impl<T> Mul<T> for Vec3<T>
    where T: Copy + Mul<Output=T>
{
    type Output = Vec3<T>;
    fn  mul(self, f: T) -> Vec3<T>
    {
        Vec3::<T> {
            dx: self.dx * f,
            dy: self.dy * f,
            dz: self.dz * f
        }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Triangle3<T>
    where T: Copy + Clone + PartialOrd
{
    pub a: Point3<T>,
    pub b: Point3<T>,
    pub c: Point3<T>,
}

impl<T> Triangle3<T>
    where T: Copy + Clone + PartialOrd + Mul<Output=T>
{
    pub fn from<T1, T2, T3>(a_: T1, b_: T2, c_: T3) -> Triangle3<T>
        where T1: Convert<Point3<T>>, T2: Convert<Point3<T>>, T3: Convert<Point3<T>>,
    {
        Triangle3 { a: a_.convert(), b: b_.convert(), c: c_.convert() }
    }
    pub fn normalize_by_y(&mut self) {
        if self.a.y.partial_cmp(&self.b.y) == Some(Greater) {
            swap(&mut self.a, &mut self.b);
        }

        if self.b.y.partial_cmp(&self.c.y) == Some(Greater) {
            swap(&mut self.b, &mut self.c);
        }

        if self.a.y.partial_cmp(&self.b.y) == Some(Greater) {
            swap(&mut self.a, &mut self.b);
        }
    }
}
impl<T1, T2, T3, T> Convert<Triangle3<T> > for (T1, T2, T3)
    where T1: Convert<Point3<T>>, T2: Convert<Point3<T>>, T3: Convert<Point3<T>>, T: Copy + Clone + PartialOrd + Mul<Output=T>
{
    fn convert(self) -> Triangle3<T>
    {
        Triangle3::from(self.0.convert(), self.1.convert(), self.2.convert())
    }
}
impl Convert<Triangle3<f32> > for Triangle3<i32>
{
    fn convert(self) -> Triangle3<f32> {
        Triangle3::<f32>::from(self.a,
                              self.b,
                              self.c)
    }
}

impl Convert<Triangle3<i32> > for Triangle3<f32>
{
    fn convert(self) -> Triangle3<i32> {
        Triangle3::<i32>::from(self.a,
                              self.b,
                              self.c)
    }
}
