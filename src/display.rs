extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;



pub enum ValidHex {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
}

pub struct Display {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,

    pixel_width: u32,
    pixel_height: u32,

    buffer: [[bool; 64]; 32],
}

#[allow(dead_code)]
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let pixel_width = width / 64;
        let pixel_height = height / 32;

        let buffer = [[false; 64]; 32];

        Display {            canvas,
            event_pump,
            pixel_width,
            pixel_height,
            buffer,
        }
    }

    pub fn draw_frame(&mut self) {
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        for (y, row) in self.buffer.iter_mut().enumerate() {
            for (x, element) in row.iter_mut().enumerate() {
                if *element {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
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

    pub fn flip_pixel(&mut self, x: u8, y: u8) -> bool {
        let x = x as usize;
        let y = y as usize;
        self.buffer[y][x] = !self.buffer[y][x];

        // Return true if pixel was turned off (is now on)
        !self.buffer[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.buffer[y][x] = value;
    }

    // pub enum DisplayEvent {
    //     Quit,
    //     Keypress,
    // }
    // pub fn iter_events(&mut self) -> std::vec::IntoIter<DisplayEvent> {
    //     // Polls all events in event loop and returns iterator containing events to be
    //     // processed by chip-8
    //     let mut event_queue: Vec<DisplayEvent> = Vec::new();

    //     for event in self.event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => event_queue.push(DisplayEvent::Quit),
    //             _ => {}
    //         }
    //     }

    //     event_queue.into_iter()
    // }

    pub fn clear(&mut self) {
        self.buffer = [[false; 64]; 32];
        self.draw_frame();
    }

    pub fn check_key(&self, key: u8) -> bool{
        match key {
            0x0 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num0),
            0x1 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num1),
            0x2 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num2),
            0x3 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num3),
            0x4 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num4),
            0x5 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num5),
            0x6 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num6),
            0x7 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num7),
            0x8 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num8),
            0x9 => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::Num9),
            0xA => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::A),
            0xB => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::B),
            0xC => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::C),
            0xD => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::D),
            0xE => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::E),
            0xF => self.event_pump.keyboard_state().is_scancode_pressed(Scancode::F),
            _ => panic!("Invalid hex value passed to check_key()")
        }
    }

    pub fn get_keys(&self) -> Vec<ValidHex> {
        let mut pressed_keys = Vec::new();

        for key in self.event_pump.keyboard_state().pressed_scancodes() {
            match key {
                Scancode::Num0 => pressed_keys.push(ValidHex::Num0),
                Scancode::Num1 => pressed_keys.push(ValidHex::Num1),
                Scancode::Num2 => pressed_keys.push(ValidHex::Num2),
                Scancode::Num3 => pressed_keys.push(ValidHex::Num3),
                Scancode::Num4 => pressed_keys.push(ValidHex::Num4),
                Scancode::Num5 => pressed_keys.push(ValidHex::Num5),
                Scancode::Num6 => pressed_keys.push(ValidHex::Num6),
                Scancode::Num7 => pressed_keys.push(ValidHex::Num7),
                Scancode::Num8 => pressed_keys.push(ValidHex::Num8),
                Scancode::Num9 => pressed_keys.push(ValidHex::Num9),
                Scancode::A => pressed_keys.push(ValidHex::A),
                Scancode::B => pressed_keys.push(ValidHex::B),
                Scancode::C => pressed_keys.push(ValidHex::C),
                Scancode::D => pressed_keys.push(ValidHex::D),
                Scancode::E => pressed_keys.push(ValidHex::E),
                Scancode::F => pressed_keys.push(ValidHex::F),
                _ => {}
            }
        }

        pressed_keys
    }

    pub fn check_quit(&mut self) -> bool {
        // NOTE: Clears event_queue, only use if quit and esc are the
        // only sdl events being used
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }
        false
    }
}
