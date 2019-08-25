extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

use crate::process::Process;

pub enum SysCall {
    Fork,
    Exit,
    Noop,
}

pub struct Kernel {
    sdl_context: sdl2::Sdl,
    processes: Vec<Process>,
    start_point: Point
}

impl Kernel {

    pub fn init(sdl_context: sdl2::Sdl, processes: Vec<Process>, start_point: Point) -> Kernel {
        Kernel {sdl_context, processes, start_point}
    }

    pub fn start(mut self) {
        let video_subsystem = self.sdl_context.video().unwrap();
        let window = video_subsystem.window("rAnimate", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        // Add the first process to the process vector
        self.processes.push(Process::init(self.start_point, Point::new(1,2)));

        // Continuously loop, calling update on every iteration
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            
            // Every iteration the kernel gives control to each process
            self.update(&mut canvas);
            
            canvas.present();
            
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn update(&mut self, canvas: &mut Canvas<Window>) {

        for process in &mut self.processes {
            let syscall = process.run(canvas);
            canvas.set_draw_color(Color::RGB(255, 210, 0));
        }

    }

}