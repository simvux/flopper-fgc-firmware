use super::Button;

pub struct State {
    pub buttons: [Button; 2],

    // middle: [1416, 1465]
    pub joystick: [u16; 2],

    pub usb_data: [u8; 10],
}

impl State {
    pub fn new<const N: usize>(buttons: [&'static str; N]) -> Self {
        Self {
            usb_data: [0; 10],
            buttons: buttons.map(Button::new),
            joystick: [4096 / 2, 4096 / 2],
        }
    }

    pub fn run(self) -> Self {
        self.debug_print();
        self
    }

    fn debug_print(&self) {
        if self.joystick[0] < 1000 {
            println!("left");
        } else if self.joystick[0] > 2000 {
            println!("right");
        }

        if self.joystick[1] < 1000 {
            println!("down");
        } else if self.joystick[1] > 2000 {
            println!("up");
        }
    }
}
