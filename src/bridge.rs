extern crate libc;
use std::os::raw::*;
use std::ffi::CString;
use std;
use libc::strdup;
use std::ptr;
use std::env;
use std::vec::*;
use std::mem;

use image::Image;

#[link(name = "helper")]
extern {
    fn application(argc: c_int, argv: *const *const c_char) -> *mut c_void;
    pub fn showImage(img: *mut c_void, width: c_int, height: c_int);
    pub fn exec(app: *mut c_void) -> c_int;
}

pub struct Application
{
    app: *mut c_void
}

impl Application {
    pub fn new() -> Application {
        for argument in env::args() {
            println!("{}", argument);
        }

        let args = env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
        let arr = vec![ptr::null() as *const i8];
        let finish: std::vec::IntoIter<*const i8>= arr.into_iter();
        let c_args = args.iter().map(|arg| {
            unsafe {
            let s: * const i8 = strdup(arg.as_ptr());
            //libc::printf("%s\n".as_ptr() as * const i8, arg.as_ptr(), s);
            s
            }
        }
        ).chain(finish).collect::<Vec<*const i8>>();

        let app: * mut c_void;
        unsafe {
            let ptr_args: * const * const i8 = c_args.as_ptr();


            for i in 0..(env::args().count() + 1) {
                let ptr: * const i8 = *ptr_args.offset(i as isize);
                libc::printf("argument %d=<%s>\n".as_ptr() as * const i8, i, ptr);
            }
            app = application(env::args().count() as i32, c_args.as_ptr());
        }
        Application {
            app: app
        }
    }
    pub fn show_image(&mut self, img: &Image) {
        unsafe {
            showImage(mem::transmute(img.data.raw_data()), img.w as c_int, img.h as c_int);
        }
    }
    pub fn run(&mut self) {
        unsafe {
            exec(self.app);
        }
    }
}
