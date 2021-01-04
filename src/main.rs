mod constants;
mod display;

use crate::constants::*;
use crate::display::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Duration, Instant};

#[allow(dead_code)]

struct Chip8 {
    // 4kb RAM
    pub ram: [u8; 4096],

    // General purpose registers
    v: [u8; 16],

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
            v: [0; 16],
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

    fn read_instruction(&self, start_address: usize) -> u16 {
        let upper = self.ram[start_address] as u16;
        let lower = self.ram[start_address + 1] as u16;

        (upper << 8) | lower
    }

    fn load_ram_from_file(&mut self, path: &Path) -> io::Result<()> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;

        // Check file length is less than 4096 - 512 = 0xDFF bytes
        match file.read_to_end(&mut buffer) {
            Ok(1..=3583) => self.write_ram(&buffer, 0x200),
            Ok(0) => panic!("Input file appears empty"),
            Ok(_) => panic!("Input file is too large, max size is 0xDFF bytes"),
            Err(m) => return Err(m),
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
        let instruction = self.read_instruction(self.pc.into());
        self.pc += 2;

        // Get instruction arguments
        let op = (instruction & 0b1111_0000_0000_0000) >> 12;
        let x = (instruction & 0b0000_1111_0000_0000) >> 8;
        let y = (instruction & 0b0000_0000_1111_0000) >> 4;
        let n = instruction & 0b0000_0000_0000_1111;
        let nn = instruction & 0b0000_0000_1111_1111;
        let nnn = instruction & 0b0000_1111_1111_1111;

        // Decode
        match op {
            0x0 => {
                match instruction {
                    0x00E0 => {
                        // CLS
                        self.cls();
                    }
                    0x00EE => {
                        // RET
                        // TODO
                    }
                    _ => {}
                }
            }

            0x1 => {
                // JP
                self.jp(nnn);
            }

            0x6 => {
                // LD immediate
                self.ld_imm(x, nn as u8);
            }

            0x7 => {
                // ADD immediate
                self.add_imm(x, nn as u8);
            }

            0xA => {
                // SET Index
                self.set_i(nnn);
            }

            0xD => {
                // DRW
                self.drw(x, y, n);
            }

            _ => panic!("Reached unimplemented instruction"),
        }

        // Execute
    }

    fn cls(&mut self) {
        // 00E0 - CLS
        // Clears the display
        self.display.clear();
    }

    fn jp(&mut self, address: u16) {
        // 1nnn - JP addr
        // Sets program counter to nnn
        self.pc = address;
    }

    fn ld_imm(&mut self, target_register: u16, imm_value: u8) {
        // 6xnn - LD Vx, nn
        // Loads the immediate value nn into register Vx
        self.v[target_register as usize] = imm_value;
    }

    fn add_imm(&mut self, target_register: u16, imm_value: u8) {
        // 7xnn - ADD Vx, nn
        // Adds the value nn to register Vx and stores it in Vx
        // Note: doesn't affect overflow flag
        self.v[target_register as usize] += imm_value;
    }

    fn set_i(&mut self, address: u16) {
        // Annn - LD I, addr
        // Sets Index register to addr
        self.i = address;
    }

    fn drw(&mut self, x_register: u16, y_register: u16, n_bytes: u16) {
        // Get starting coordinates with appropriate wrapping
        let x_coord = self.v[x_register as usize] % 64;
        let y_coord = self.v[y_register as usize] % 32;

        // Reset VF
        self.v[0xF] = 0;
        let mut set_vf = false;

        // For every row of sprite data
        for n in 0..n_bytes {
            // Load data from ram
            let sprite_data: u8 = self.ram[(self.i + n) as usize];

            // For each bit in sprite row
            for bit in 0..8 {
                // Mask MSB and if set, flip pixel on display
                if (sprite_data << bit) & 0b1000_0000 != 0 {
                    set_vf |= self.display.flip_pixel(x_coord + bit, y_coord + n as u8);
                }

                if x_coord + bit > 63 {
                    break
                }
            }

            if y_coord + (n as u8) > 31 {
                break
            }
        }

        if set_vf {
            self.v[0xF] = 1;
        }
    }
}

fn main() {
    let mut cpu = Chip8::new();

    let program_path = Path::new("ibm_logo.ch8");

    cpu.load_ram_from_file(program_path)
        .expect("Failed to read file");
    cpu.execute_loop();

    // println!("{:x?}", cpu.ram);
}
