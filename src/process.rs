extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::kernel::SysCall;
use sdl2::video::Window;
use sdl2::video::WindowSurfaceRef;

use sdl2::surface::Surface;

pub struct Process {
    location: Point,
    direction: Point,
}

impl Process {

    pub fn init(location: Point, direction: Point) -> Process {
        Process {location, direction}
    }

    pub fn run(&mut self, canvas_surface: &mut WindowSurfaceRef, surface: &mut Surface) -> SysCall {
        self.location.x += self.direction.x;
        self.location.y += self.direction.y;
        //surface.draw_point(self.location).unwrap();
        surface.fill_rect(sdl2::rect::Rect::new(self.location.x,self.location.y,10,10), Color::RGB(255, 210, 0)).unwrap();
        surface.blit(None, canvas_surface, None).unwrap();

        
        SysCall::Fork
    }
}