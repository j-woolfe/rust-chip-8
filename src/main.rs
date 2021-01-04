mod constants;
mod display;

use crate::constants::*;
use crate::display::Display;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;



#[allow(dead_code)]

struct Chip8 {
    // 4kb RAM
    pub ram: [u8; 4096],

    // General purpose registers
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,

    // Flag register
    vf: u8,

    // Index register
    i: u16,

    // Program counter
    pc: u16,

    // Stack and stack pointer
    stack: [u16; 16],
    sp: u8,

    // Delay and sound timers
    dt: u8,
    st: u8,

    // Display
    display: Display,
}

impl Chip8 {
    fn new() -> Chip8 {
        // Initialise empty Chip8
        let mut cpu = Chip8 {
            ram: [0; 4096],
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            display: Display::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        };

        // Write font to 0x050 - 0x09F
        static FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        cpu.write_ram(&FONT, 0x050);

        // Return cpu
        cpu
    }

    fn write_ram(&mut self, array: &[u8], start_address: u16) {
        let mut index = start_address;
        for byte in array {
            self.ram[index as usize] = *byte;
            index += 1;
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn load_ram_from_file(&mut self, path: &Path) -> io::Result<()> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;

        // Check file length is less than 4096 - 512 = 0xDFF bytes
        match file.read_to_end(&mut buffer) {
            Ok(1..=3583) => self.write_ram(&buffer, 0x200),
            Ok(0) => panic!("Input file appears empty"),
            Ok(_) => panic!("Input file is too large, max size is 0xDFF bytes"),
            Err(m) => return Err(m)
        }
            
        Ok(())
    }

    fn execute_loop(&mut self) {
        let mut start_time: Instant;
        let mut since_last_step = Duration::new(0, 0);
        let mut since_last_frame = Duration::new(0, 0);

        let step_dt = Duration::new(0, 1_000_000_000u32 / INSTRUCT_PER_SEC);
        let frame_dt = Duration::new(0, 1_000_000_000u32 / FRAME_RATE);

        // // Draw checkerboard
        // let mut toggle = false;
        // for x in 0..64 {
        //     for y in 0..32 {
        //         self.display.set_pixel(x, y, toggle);
        //         toggle = !toggle;
        //     }
        //     toggle = !toggle;
        // }

        'execution: loop {
            start_time = Instant::now();

            if self.display.check_quit() {
                break 'execution;
            };

            since_last_step += start_time.elapsed();

            if since_last_step >= step_dt {
                self.step_cpu();
                since_last_step -= step_dt;
            }

            since_last_frame += start_time.elapsed();

            if since_last_frame >= frame_dt {
                // // Flip checkerboard
                // for x in 0..64 {
                //     for y in 0..32 {
                //         self.display.flip_pixel(x, y);
                //     }
                // }
                self.display.draw_frame();
                since_last_frame -= frame_dt;
            }
        }
    }

    fn step_cpu(&mut self) {
        // Fetch

        // Decode

        // Execute
    }
}

fn main() {
    let mut cpu = Chip8::new();
    
    let program_path = Path::new("ibm_logo.ch8");

    cpu.load_ram_from_file(program_path).expect("Failed to read file");
    cpu.execute_loop();

    // println!("{:x?}", cpu.ram);
}
