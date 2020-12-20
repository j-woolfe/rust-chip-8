mod display;

use crate::display::Display;
use crate::display::DisplayEvent;
use std::time::Duration;

pub fn main() {
    let mut display = Display::new(640, 320);

    'running: loop {
        let mut toggle = false;

        for x in 0..64 {
            for y in 0..32 {
                display.set_pixel(x, y, toggle);
                toggle = !toggle;
            }
            toggle = !toggle;
        }

        for event in display.iter_events() {
            match event {
                DisplayEvent::Quit => break 'running,
                DisplayEvent::Keypress => {},
                _ => {}
            }
        }

        // Rest of game loop goes here

        display.draw_frame();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
