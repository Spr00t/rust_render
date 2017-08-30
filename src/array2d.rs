extern crate rand;

use geometry::*;
#[allow(dead_code)]
pub struct Array2d<T>
    where T: Copy + Clone
{
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T> Array2d<T>
    where T: Copy + Clone
{
    pub fn raw_data(&self) -> * const T {
        self.data.as_ptr()
    }
    pub fn new(a: T, w: usize, h: usize) -> Self {
        let obj = Array2d::<T> {
            width: w,
            height: h,
            data: vec![a; w * h]
        };
        obj
    }
    pub fn get_mut<A, B>(&mut self, x: A, y: B) -> Option<&mut T>
        where A: Convert<usize>, B: Convert<usize>
    {
        self.data.get_mut(y.convert() * self.width + x.convert())
    }
    
    #[allow(dead_code)]
    pub fn get<A, B>(&self, x: A, y: B) -> Option<&T>
        where A: Convert<usize>, B: Convert<usize>
    {
        self.data.get(y.convert() * self.width + x.convert())
    }
}
