#![feature(array_try_map)]

mod button;
use button::Button;

mod devices;
use devices::Devices;

mod run;
use run::State;

use esp_idf_svc::hal::{delay, peripherals::Peripherals, task::notification::Notification, timer};
use esp_idf_svc::sys::EspError;
use std::num::NonZeroU32;

const POLLING_RATE: u64 = 500;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let per = Peripherals::take()?;
    let (mut devices, timer00) = Devices::init(per)?;

    let notification = Notification::new();

    let timer_conf = timer::config::Config::new().auto_reload(true);
    let mut timer = timer::TimerDriver::new(timer00, &timer_conf)?;

    timer.set_alarm(timer.tick_hz() / POLLING_RATE)?;

    let notifier = notification.notifier();

    unsafe {
        timer.subscribe(move || {
            let bitset = 0b10001010101;
            notifier.notify_and_yield(NonZeroU32::new(bitset).unwrap());
        })?;
    }

    timer.enable_interrupt()?;
    timer.enable_alarm(true)?;
    timer.enable(true)?;

    let mut state = State::new(["A", "B"]);

    loop {
        if let Some(_) = notification.wait(delay::BLOCK) {
            if let Err(err) = devices.poll(&mut state) {
                log::error!("{err}");
                continue;
            }

            state = state.run();
        }
    }
}
