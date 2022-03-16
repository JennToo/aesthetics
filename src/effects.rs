use embedded_hal::blocking::delay::DelayUs;
use smart_leds::{SmartLedsWrite, RGB, RGB8};

pub const MAGENTA: RGB8 = RGB {
    r: 255,
    g: 0,
    b: 255,
};

const LED_COUNT: usize = 8 * 2 * 4;

pub struct Effector<Driver: SmartLedsWrite, Delayer: DelayUs<u32>> {
    led_driver: Driver,
    delay: Delayer,
}

impl<Driver: SmartLedsWrite<Color = RGB8, Error = ()>, Delayer: DelayUs<u32>>
    Effector<Driver, Delayer>
{
    pub fn new(led_driver: Driver, delay: Delayer) -> Self {
        Effector { led_driver, delay }
    }

    pub fn solid_color(&mut self, color: RGB8, duration: u32) {
        self.led_driver
            .write(core::iter::repeat(color).take(LED_COUNT))
            .unwrap();
        self.delay.delay_us(duration);
    }
}
