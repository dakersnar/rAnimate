extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Point;
use std::time::Duration;

use crate::process::Process;

pub struct Kernel {
    sdl_context: sdl2::Sdl,
    processes: Vec<Process>,
    start_point: Point
}

impl Kernel {

    pub fn init(sdl_context: sdl2::Sdl, processes: Vec<Process>, start_point: Point) -> Kernel {
        return Kernel {sdl_context, processes, start_point}
    }

    pub fn start(self) {
        let video_subsystem = self.sdl_context.video().unwrap();
        let window = video_subsystem.window("rAnimate", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn run(self) {

    }

    
    fn update() {

    }

    fn draw() {

    }

}

