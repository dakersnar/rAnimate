extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::kernel::SysCall;
use sdl2::video::Window;

pub struct Process {
    location: Point,
    direction: Point,
}

impl Process {

    pub fn init(location: Point, direction: Point) -> Process {
        Process {location, direction}
    }

    pub fn run(&mut self, canvas: &mut Canvas<Window>) -> SysCall {
        self.location.x += self.direction.x;
        self.location.y += self.direction.y;
        canvas.draw_point(self.location).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(10,10,10,10)).unwrap();
            
        
        SysCall::Fork
    }
}