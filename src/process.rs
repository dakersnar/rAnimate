extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::kernel::SysCall;
use sdl2::video::Window;
use sdl2::video::WindowSurfaceRef;
use sdl2::surface::Surface;

use rand::prelude::*;

pub struct Process {
    location: Point,
    direction_idx: usize,
    directions: [Point; 16],
    color: Color,
    weights: [f32; 4]
}

impl Process {

    pub fn init(location: Point, direction_idx: usize, color: Color) -> Process {
        let directions = [
            Point::new(1,0),
            Point::new(2,1),
            Point::new(1,1),
            Point::new(1,2),
            Point::new(0,1),
            Point::new(-1,2),
            Point::new(-1,1),
            Point::new(-2,1),
            Point::new(-1,0),
            Point::new(-2,-1),
            Point::new(-1,-1),
            Point::new(-1,-2),
            Point::new(0,-1),
            Point::new(1,-2),
            Point::new(1,-1),
            Point::new(2,-1),
        ];
        let weights = [0.5, 0.8, 0.9, 1.0];

        Process {location, direction_idx, directions, color, weights}
    }

    pub fn run(&mut self, canvas_surface: &mut WindowSurfaceRef, surface: &mut Surface) -> SysCall {


        

        //let mut surface_to_canvas = surface.into_canvas().unwrap();

        let choice: f32 = random();

        // TODO: make a match statement
        if choice <= self.weights[0] { 
            self.move_forward();
        } else if choice <= self.weights[1] {
            self.change_color();
        } else if choice <= self.weights[2] {
            self.turn_cw();
        } else if choice <= self.weights[3] {
            self.turn_ccw();
        }


        //surface.draw_point(self.location).unwrap();
        surface.fill_rect(sdl2::rect::Rect::new(self.location.x,self.location.y,3,3), self.color).unwrap();

        surface.blit(None, canvas_surface, None).unwrap();

        
        SysCall::Fork
    }

    pub fn get_direction(&self) -> Point {
        self.directions[self.direction_idx]
    }

    fn move_forward(&mut self) {
        self.location.x += self.get_direction().x;
        self.location.y += self.get_direction().y;
    }

    fn turn_cw(&mut self) {
        if self.direction_idx == 0 { 
            self.direction_idx = self.directions.len() - 1;
        } else {
            self.direction_idx -= 1;
        }

    }

    fn turn_ccw(&mut self) {
        if self.direction_idx == self.directions.len() - 1 { 
            self.direction_idx = 0;
        } else {
            self.direction_idx += 1;
        }
    }

    fn change_color(&mut self) {
        
    }

}