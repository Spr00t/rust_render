use std;
use std::mem::swap;
use std::collections::HashMap;
use std::cmp::Ordering::*;
use std::cmp::*;
use std::f32;
use std::fmt::*;
use std::convert::*;
use std::hash::*;
use std::ops::*;

pub fn mx<T>(a: T, b: T) -> T
    where T: Copy + Clone + PartialOrd
{
    if let Some(x) = a.partial_cmp(&b) {
        if x == Ordering::Greater {
            return a
        }
    }
    b
}
pub fn mn<T>(a: T, b: T) -> T
    where T: Copy + Clone + PartialOrd
{
    if let Some(x) = a.partial_cmp(&b) {
        if x == Ordering::Less {
            return a
        }
    }
    b
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point<T>
    where T: Copy + Clone
{
    pub x: T,
    pub y: T
}


impl<T> Display for Point<T>
    where T: Display + Copy + Clone
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Line<T>
    where T: Copy + Clone
{
    pub a: Point<T>,
    pub b: Point<T>,
}


pub trait Convert<T> {
    fn convert(self) -> T;
}

impl<T> Convert<T> for T
{
    fn convert(self) -> T { self }
}

impl Convert<Point<f32> > for Point<i32>
{
    fn convert(self) -> Point<f32> {Point{x: self.x as f32, y: self.y as f32} }
}

impl Convert<Point<i32> > for Point<f32>
{
    fn convert(self) -> Point<i32> {Point{x: self.x as i32, y: self.y as i32} }
}

impl Convert<usize> for i32
{
    fn convert(self) -> usize { self as usize }
}


impl<T> Convert<Point<T> > for (T, T)
    where T: Copy + Clone
{
    fn convert(self) -> Point<T> {
        Point {x: self.0, y: self.1 }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Triangle<T>
    where T: Copy + Clone + PartialOrd
{
    pub a: Point<T>,
    pub b: Point<T>,
    pub c: Point<T>,
}

impl<T> Triangle<T>
    where T: Copy + Clone + PartialOrd
{
    pub fn from<T1, T2, T3>(a_: T1, b_: T2, c_: T3) -> Triangle<T>
        where T1: Convert<Point<T>>, T2: Convert<Point<T>>, T3: Convert<Point<T>>,
    {
        Triangle{a: a_.convert(), b: b_.convert(), c: c_.convert() }
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

impl<T1, T2, T3, T> Convert<Triangle<T> > for (T1, T2, T3)
    where T1: Convert<Point<T>>, T2: Convert<Point<T>>, T3: Convert<Point<T>>, T: Copy + Clone + PartialOrd
{
    fn convert(self) -> Triangle<T>
    {
        Triangle::from(self.0.convert(), self.1.convert(), self.2.convert())
    }
}

impl Convert<Triangle<f32> > for Triangle<i32>
{
    fn convert(self) -> Triangle<f32> {
        Triangle::<f32>::from(self.a,
                              self.b,
                              self.c)
    }
}

impl Convert<Triangle<i32> > for Triangle<f32>
{
    fn convert(self) -> Triangle<i32> {
        Triangle::<i32>::from(self.a,
                       self.b,
                       self.c)
    }
}
