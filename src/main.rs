mod display;

use crate::display::Display;
use crate::display::DisplayEvent;
use std::time::Duration;



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

    // Stack and stack pointer
    stack: [u8; 16],
    sp: u8,

    // Delay and sound timers
    dt: u8,
    st: u8,
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
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
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
            0xF0, 0x80, 0xF0, 0x80, 0x80,  // F
            ];

        cpu.write_array(&FONT, 0x050);

        // Return cpu
        cpu
    }

    fn write_array(&mut self, array: &[u8], start_address: u16) {
        let mut index = start_address;
        for byte in array {
            self.ram[index as usize] = *byte;
            index += 1;
        }
        
    }
}




fn main() {

    let cpu = Chip8::new();

    println!("{:x?}", cpu.ram);
    



    // static WINDOW_WIDTH: u32 = 640; 
    // static WINDOW_HEIGHT: u32 = 320; 

    // static FRAME_RATE: u32 = 60;


    // // Initialise display
    // let mut display = Display::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // 'running: loop {
    //     // Draw checkerboard
    //     let mut toggle = false;
    //     for x in 0..64 {
    //         for y in 0..32 {
    //             display.set_pixel(x, y, toggle);
    //             toggle = !toggle;
    //         }
    //         toggle = !toggle;
    //     }

    //     // Process renderer events
    //     for event in display.iter_events() {
    //         match event {
    //             DisplayEvent::Quit => break 'running,
    //             DisplayEvent::Keypress => {},
    //             _ => {}
    //         }
    //     }

    //     // Rest of game loop goes here

    //     display.draw_frame();

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
    // }
}
