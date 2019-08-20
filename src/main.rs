extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

mod kernel;
mod process;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let processes: Vec<process::Process> = Vec::new();
    let start_point = Point::new(100,100);
    let kernel = kernel::Kernel::init(sdl_context, processes, start_point);
    kernel.start();
    
}

