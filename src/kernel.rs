extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowSurfaceRef;

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use std::time::Duration;

use crate::process::Process;

pub enum SysCall {
    Fork,
    Exit,
    Noop,
}

pub fn x_window_max() -> u32 {
    800
}

pub fn y_window_max() -> u32 {
    600
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
        let x = x_window_max();
        let y = y_window_max();
        let window = video_subsystem.window("rAnimate", x, y)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().software().build().unwrap();

        let mut surface = Surface::new(x, y, PixelFormatEnum::RGB24).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        // Add the first process to the process vector
        self.processes.push(Process::init(self.start_point, 2, Color::RGB(200, 90, 200)));

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
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            let mut canvas_surface = canvas.window().surface(&event_pump).unwrap();
            self.update(&mut canvas_surface, &mut surface);

            canvas.present();
            
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn update(&mut self, canvas_surface: &mut WindowSurfaceRef, surface: &mut Surface) {

        let mut i = 0;
        let mut new_procs: Vec<Process> = Vec::new();
        let mut remove_procs: Vec<usize> = Vec::new();
        for process in &mut self.processes {
            //canvas.set_draw_color(Color::RGB(255, 210, 0));
            let syscall = process.run(canvas_surface, surface);

            match syscall {
                SysCall::Fork => {
                    new_procs.push(Process::init(process.get_location(), 2, Color::RGB(200, 90, 200)));
                }
                SysCall::Exit => {
                    remove_procs.push(i);
                }
                SysCall::Noop => {
                    
                }

            }

            i += 1;
            
        }

        for i in remove_procs {
            self.processes.remove(i);
        }

        for process in new_procs {
            self.processes.push(process);
        }

    }

}