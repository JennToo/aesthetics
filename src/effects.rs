use embedded_hal::blocking::delay::DelayUs;
use smart_leds::{SmartLedsWrite, RGB, RGB8};

pub const MAGENTA: RGB8 = RGB {
    r: 255,
    g: 0,
    b: 255,
};
pub const BLACK: RGB8 = RGB { r: 0, g: 0, b: 0 };

const LED_COUNT: usize = 8 * 2 * 4;
type Frame = [RGB8; LED_COUNT];
pub const FRAME_DURATION: u32 = 16_000;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Effect {
    SolidColor(RGB8),
}

pub type Duration = u32;

pub type EffectScript = [(Effect, Duration)];

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

    pub fn run_script(&mut self, script: &EffectScript) {
        let mut frame = [BLACK; LED_COUNT];
        for (effect, duration) in script.iter().cloned() {
            let mut remaining_duration = duration;

            while remaining_duration > 0 {
                effect.render_frame(&mut frame);
                self.led_driver.write(frame.iter().cloned()).unwrap();

                self.delay.delay_us(FRAME_DURATION);
                remaining_duration = remaining_duration.saturating_sub(FRAME_DURATION);
            }
        }
    }
}

impl Effect {
    fn render_frame(self, frame: &mut Frame) {
        match self {
            Effect::SolidColor(color) => {
                for element in frame.iter_mut() {
                    *element = color;
                }
            }
        }
    }
}
