use super::State;
use esp_idf_svc::hal::{
    adc, adc::attenuation, gpio, gpio::AnyIOPin, gpio::IOPin, peripherals::Peripherals, timer,
};
use esp_idf_svc::sys::EspError;

pub struct Devices<'a> {
    buttons: [gpio::PinDriver<'a, AnyIOPin, gpio::Input>; 2],

    x_driver: adc::AdcChannelDriver<'a, { attenuation::DB_11 }, gpio::Gpio3>,
    y_driver: adc::AdcChannelDriver<'a, { attenuation::DB_11 }, gpio::Gpio2>,

    adc: adc::AdcDriver<'a, adc::ADC1>,
}

impl<'a> Devices<'a> {
    pub fn init(per: Peripherals) -> Result<(Self, timer::TIMER00), EspError> {
        let mut buttons = [per.pins.gpio4.downgrade(), per.pins.gpio8.downgrade()]
            .try_map(gpio::PinDriver::input)?;

        for button in &mut buttons {
            button.set_pull(gpio::Pull::Down)?;
        }

        let adcconfig = adc::config::Config::new().calibration(true);
        let adc = adc::AdcDriver::new(per.adc1, &adcconfig)?;

        let x_driver = adc::AdcChannelDriver::new(per.pins.gpio3)?;
        let y_driver = adc::AdcChannelDriver::new(per.pins.gpio2)?;

        Ok((
            Self {
                buttons,
                x_driver,
                y_driver,
                adc,
            },
            per.timer00,
        ))
    }

    pub fn poll(&mut self, state: &mut State) -> Result<(), EspError> {
        for (device, button) in self.buttons.iter_mut().zip(&mut state.buttons) {
            let level = device.get_level();
            button.update(level);
        }

        state.joystick[0] = self.adc.read(&mut self.x_driver)?;
        state.joystick[1] = self.adc.read(&mut self.y_driver)?;

        Ok(())
    }
}
