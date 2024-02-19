use esp_idf_svc::hal::gpio;

#[derive(Clone, Copy, Debug)]
pub struct Button {
    pub pressed: bool,
    pub held: bool,
    name: &'static str,
}

impl Button {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            pressed: false,
            held: false,
        }
    }

    pub fn update(&mut self, level: gpio::Level) {
        match level {
            gpio::Level::Low if self.pressed => {
                self.pressed = false;
                log::info!("release {}", self.name)
            }
            gpio::Level::High if !self.pressed => {
                self.pressed = true;
                log::info!("pressed {}", self.name)
            }
            _ => {}
        }

        self.held = level == gpio::Level::High;
    }
}
