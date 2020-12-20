extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub enum DisplayEvent {
    Quit,
    Keypress,
}

pub struct Display {
    canvas: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump,

    pixel_width: u32,
    pixel_height: u32,

    buffer: [[bool; 64]; 32],
}

impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip-8", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let pixel_width = width / 64;
        let pixel_height = height / 32;

        let buffer = [[false; 64]; 32];

        Display {
            canvas,
            event_pump,
            pixel_width,
            pixel_height,
            buffer,
        }
    }

    pub fn draw_frame(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        for (y, row) in self.buffer.iter_mut().enumerate() {
            for (x, element) in row.iter_mut().enumerate() {
                if *element {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }

                self.canvas
                    .fill_rect(Rect::new(
                        x as i32 * self.pixel_width as i32,
                        y as i32 * self.pixel_height as i32,
                        self.pixel_width,
                        self.pixel_height,
                    ))
                    .unwrap();
            }
        }

        self.canvas.present();
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.buffer[y][x] = !self.buffer[y][x];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.buffer[y][x] = value;
    }

    pub fn iter_events(&mut self) -> std::vec::IntoIter<DisplayEvent> {
        // Polls all events in event loop and returns iterator containing events to be
        // processed by chip-8
        let mut event_queue: Vec<DisplayEvent> = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => event_queue.push(DisplayEvent::Quit),
                _ => {}
            }
        }

        event_queue.into_iter()
    }
}
