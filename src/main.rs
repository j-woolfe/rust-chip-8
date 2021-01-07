mod constants;
mod display;

extern crate rand;

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
    sp: usize,

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
        let x = ((instruction & 0b0000_1111_0000_0000) >> 8) as usize;
        let y = ((instruction & 0b0000_0000_1111_0000) >> 4) as usize;
        let n = instruction & 0b0000_0000_0000_1111;
        let nn = (instruction & 0b0000_0000_1111_1111) as u8;
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
                        self.ret();
                    }
                    _ => panic!("Reached unimplemented instruction {:#04X}", instruction),
                }
            }

            0x1 => {
                // JP
                self.jp(nnn);
            }

            0x2 => {
                // CALL
                self.call(nnn);
            }

            0x3 => {
                // SE immediate
                self.se_imm(x, nn);
            }

            0x4 => {
                // SNE immediate
                self.sne_imm(x, nn);
            }

            0x5 => {
                // SE
                self.se(x, y)
            }

            0x6 => {
                // LD immediate
                self.ld_imm(x, nn);
            }

            0x7 => {
                // ADD immediate
                self.add_imm(x, nn);
            }

            0x8 => {
                match n {
                    0x0 => {
                        // LD
                        self.ld(x, y);
                    }

                    0x1 => {
                        // OR
                        self.or(x, y);
                    }

                    0x2 => {
                        // AND
                        self.and(x, y);
                    }

                    0x3 => {
                        // XOR
                        self.xor(x, y);
                    }

                    0x4 => {
                        // ADD
                        self.add(x, y);
                    }

                    0x5 => {
                        // SUB
                        self.sub(x, y);
                    }

                    0x6 => {
                        // SHR
                        self.shr(x);
                    }

                    0x7 => {
                        // SUBN
                        self.subn(x, y);
                    }

                    0xE => {
                        // SHL
                        self.shl(x);
                    }

                    _ => panic!("Reached unimplemented instruction {:#04X}", instruction),
                }
            }

            0x9 => {
                // SNE
                self.sne(x, y);
            }

            0xA => {
                // LD Index
                self.ld_i_imm(nnn);
            }

            0xB => {
                // JP to nnn + v0
                self.jp_offset(nnn);
            }

            0xC => {
                // RAND
                self.rand(x, nn);
            }

            0xD => {
                // DRW
                self.drw(x, y, n);
            }

            0xE => match nn {
                0x9E => {
                    // SKP
                    self.skp(x);
                }

                0xA1 => {
                    // SKNP
                    self.sknp(x);
                }
                _ => panic!("Reached unimplemented instruction {:#04X}", instruction),
            },

            0xF => {
                match nn {
                    0x1E => {
                        // ADD I
                        self.add_i(x);
                    }

                    0x29 => {
                        // LD Font
                        self.ld_f(x);
                    }

                    0x33 => {
                        // LD bcd
                        self.ld_bcd(x);
                    }

                    0x55 => {
                        // LD into I
                        self.ld_into_i(x);
                    }

                    0x65 => {
                        // LD from I
                        self.ld_from_i(x);
                    }

                    _ => panic!("Reached unimplemented instruction {:#04X}", instruction),
                }
            }

            _ => panic!("Reached unimplemented instruction {:#04X}", instruction),
        }

        // Execute
    }

    fn cls(&mut self) {
        // 00E0 - CLS
        // Clears the display
        self.display.clear();
    }

    fn ret(&mut self) {
        // 00EE - RET
        // Set PC to top value in Stack, Decrement SP
        self.pc = self.stack[self.sp];
        self.sp -= 1;
    }

    fn jp(&mut self, address: u16) {
        // 1nnn - JP nnn
        // Sets program counter to nnn
        self.pc = address;
    }

    fn call(&mut self, address: u16) {
        // 1nnn - CALL nnn
        // Increment SP, Push current PC to stack, Set program counter to nnn
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = address;
    }

    fn se_imm(&mut self, target_register: usize, imm_value: u8) {
        // 3xnn - SE Vx, nn
        // Skip next instruction if Vx = nn
        if self.v[target_register] == imm_value {
            self.pc += 2;
        }
    }

    fn sne_imm(&mut self, target_register: usize, imm_value: u8) {
        // 4xnn - SNE Vx, nn
        // Skip next instruction if Vx != nn
        if self.v[target_register] != imm_value {
            self.pc += 2;
        }
    }

    fn se(&mut self, x_register: usize, y_register: usize) {
        // 5xy0 - SE Vx, Vy
        // Skip next instruction if Vx = Vy
        if self.v[x_register] == self.v[y_register] {
            self.pc += 2;
        }
    }

    fn ld_imm(&mut self, target_register: usize, imm_value: u8) {
        // 6xnn - LD Vx, nn
        // Loads the immediate value nn into register Vx
        self.v[target_register] = imm_value;
    }

    fn add_imm(&mut self, x_register: usize, imm_value: u8) {
        // 7xnn - ADD Vx, nn
        // Adds the value nn to register Vx and stores it in Vx
        // Note: doesn't affect overflow flag
        self.v[x_register] = self.v[x_register].wrapping_add(imm_value);
    }

    fn ld(&mut self, x_register: usize, y_register: usize) {
        // 8xy0 - LD Vx, Vy
        // Set Vx = Vy
        self.v[x_register] = self.v[y_register];
    }

    fn or(&mut self, x_register: usize, y_register: usize) {
        // 8xy1 - OR Vx, Vy
        // Set Vx = Vx OR Vy
        self.v[x_register] |= self.v[y_register];
    }

    fn and(&mut self, x_register: usize, y_register: usize) {
        // 8xy2 - AND Vx, Vy
        // Set Vx = Vx AND Vy
        self.v[x_register] &= self.v[y_register];
    }

    fn xor(&mut self, x_register: usize, y_register: usize) {
        // 8xy3 - XOR Vx, Vy
        // Set Vx = Vx XOR Vy
        self.v[x_register] ^= self.v[y_register];
    }

    fn add(&mut self, x_register: usize, y_register: usize) {
        // 8xy4 - ADD Vx, Vy
        // Set Vx = Vx + Vy
        // Set VF = carry
        let (value, overflow) = self.v[x_register].overflowing_add(self.v[y_register]);

        if overflow {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.v[x_register] = value;
    }

    fn sub(&mut self, x_register: usize, y_register: usize) {
        // 8xy5 - SUB Vx, Vy
        // Set Vx = Vx - Vy
        // Set VF = Not borrow (Vx <= Vy)
        if self.v[x_register] > self.v[y_register] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.v[x_register] = self.v[x_register].wrapping_sub(self.v[y_register]);
    }

    fn shr(&mut self, x_register: usize) {
        // 8xy6 - SHR Vx
        // Set Vx = Vx >> 1
        // Set VF = LSB of X = 1

        if (self.v[x_register] & 0b0000_0001) != 0 {
            self.v[0xF] = 1
        } else {
            self.v[0xF] = 0;
        }

        self.v[x_register] >>= 1;
    }

    fn subn(&mut self, x_register: usize, y_register: usize) {
        // 8xy7 - SUBN Vx, Vy
        // Set Vx = Vy - Vx
        // Set VF = Not borrow (Vx <= Vy)

        if self.v[y_register] > self.v[x_register] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.v[x_register] = self.v[y_register].wrapping_sub(self.v[x_register]);
    }

    fn shl(&mut self, x_register: usize) {
        // 8xy7 - SHL Vx
        // Set Vx = Vx << 1
        // Set VF = MSB of X = 1

        if (self.v[x_register] & 0b1000_0000) != 0 {
            self.v[0xF] = 1
        } else {
            self.v[0xF] = 0;
        }

        self.v[x_register] >>= 1;
    }

    fn sne(&mut self, x_register: usize, y_register: usize) {
        // 9xy0 - SNE Vx, Vy
        // Skip next instruction if Vx != Vy
        if self.v[x_register] != self.v[y_register] {
            self.pc += 2;
        }
    }

    fn ld_i_imm(&mut self, address: u16) {
        // Annn - LD I, addr
        // Sets Index register to addr
        self.i = address;
    }

    fn jp_offset(&mut self, address: u16) {
        // Bnnn - JP to nnn + V0
        // Set PC to nnn + V0
        self.pc = address + (self.v[0x0] as u16);
    }

    fn rand(&mut self, x_register: usize, imm_value: u8) {
        // Cxnn - RAND Vx, nn
        // Set Vx to a random 8-bit number ANDed with nn
        let rand_val: u8 = rand::random();
        self.v[x_register] = rand_val & imm_value;
    }

    fn drw(&mut self, x_register: usize, y_register: usize, n_bytes: u16) {
        // Dxyn - DRW Vx, Vy, n
        // Display n-byte sprite starting at memory location I at (Vx, Vy)
        // Set VF = collision

        // Get starting coordinates with appropriate wrapping
        let x_coord = self.v[x_register] % 64;
        let y_coord = self.v[y_register] % 32;

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
                    // set_vf will be true if any writes turned a pixel off
                    set_vf |= self.display.flip_pixel(x_coord + bit, y_coord + n as u8);
                }

                // Check that sprite doesn't go off side of display
                if x_coord + bit > 63 {
                    break;
                }
            }

            // Check that sprite doesn't go off bottom of display
            if y_coord + (n as u8) > 31 {
                break;
            }
        }

        // If needed, set VF
        if set_vf {
            self.v[0xF] = 1;
        }
    }

    fn skp(&mut self, x_register: usize) {
        // Ex9E - SKP Vx
        // Skip next instruction if key with value Vx is pressed
        if self.display.check_key(self.v[x_register]) {
            self.pc += 2;
        }
    }

    fn sknp(&mut self, x_register: usize) {
        // ExA1 - SKNP Vx
        // Skip next instruction if key with value Vx is not pressed
        if !self.display.check_key(self.v[x_register]) {
            self.pc += 2;
        }
    }

    fn add_i(&mut self, x_register: usize) {
        // Fx1E - ADD I, Vx
        // Set I = I + Vx

        self.i += self.v[x_register] as u16;
    }

    fn ld_f(&mut self, x_register: usize) {
        // Fx29 - LD F, Vx
        // Set I = location of sprite for digit Vx

        // Index = base(0x050) + Vx * offset(0x5)
        self.i = 0x050 + (self.v[x_register] as u16) * 0x005;
    }

    fn ld_bcd(&mut self, x_register: usize) {
        // Fx33 - LD B, Vx
        // Store BCD representation of Vx in memory locations I, I+1 and I+2

        let mut value = self.v[x_register];

        // Store least significant digit in I+2
        self.ram[self.i as usize + 2] = value % 10;
        value /= 10;

        // Store second digit in I+1
        self.ram[self.i as usize + 1] = value % 10;
        value /= 10;

        // Store most significant digit in I
        self.ram[self.i as usize] = value % 10;
    }

    fn ld_into_i(&mut self, x_register: usize) {
        // Fx55 - LD [I], Vx
        // Stores registers V0 to Vx into memory starting at I

        let start_address = self.i as usize;

        for i in 0..=x_register {
            self.ram[start_address + i] = self.v[i];
        }
    }

    fn ld_from_i(&mut self, x_register: usize) {
        // Fx65 - LD Vx, [I]
        // Reads registers V0 to Vx from memory starting at I

        let start_address = self.i as usize;

        for i in 0..=x_register {
            self.v[i] = self.ram[start_address + i];
        }
    }
}

fn main() {
    let mut cpu = Chip8::new();

    // let program_path = Path::new("ibm_logo.ch8");
    // let program_path = Path::new("BC_test.ch8");
    let program_path = Path::new("test_opcode.ch8");

    cpu.load_ram_from_file(program_path)
        .expect("Failed to read file");
    cpu.execute_loop();

    // println!("{:x?}", cpu.ram);
}
