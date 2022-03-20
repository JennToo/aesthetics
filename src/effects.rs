use embedded_hal::blocking::delay::DelayMs;
use smart_leds::{SmartLedsWrite, RGB, RGB8};

pub const MAGENTA: RGB8 = RGB {
    r: 255,
    g: 0,
    b: 255,
};
pub const BLACK: RGB8 = RGB { r: 0, g: 0, b: 0 };

const LED_COUNT: usize = 8 * 2 * 4;
type Frame = [RGB8; LED_COUNT];
pub const FRAME_DURATION: u32 = 16;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Effect {
    SolidColor(RGB8),
    AlternatingColors(RGB8, RGB8),
    Fade,
}

pub type Duration = u32;

pub type EffectScript = [(Effect, Duration)];

pub struct Effector<Driver: SmartLedsWrite, Delayer: DelayMs<u32>> {
    led_driver: Driver,
    delay: Delayer,
}

impl<Driver: SmartLedsWrite<Color = RGB8, Error = ()>, Delayer: DelayMs<u32>>
    Effector<Driver, Delayer>
{
    pub fn new(led_driver: Driver, delay: Delayer) -> Self {
        Effector { led_driver, delay }
    }

    pub fn run_script(&mut self, script: &EffectScript) {
        let mut frame = [BLACK; LED_COUNT];
        let mut previous_effect = None;

        let mut effect_iterator = script.iter().cloned().peekable();

        while effect_iterator.peek() != None {
            let (effect, duration) = effect_iterator.next().unwrap();
            let next_effect = effect_iterator.peek().map(|(effect, _)| *effect);
            let mut remaining_duration = duration;

            while remaining_duration > 0 {
                let percent_complete = 100 * remaining_duration / duration;
                effect.render_frame(&mut frame, percent_complete, previous_effect, next_effect);
                self.led_driver.write(frame.iter().cloned()).unwrap();

                self.delay.delay_ms(FRAME_DURATION);
                remaining_duration = remaining_duration.saturating_sub(FRAME_DURATION);
            }
            previous_effect = Some(effect);
        }
    }
}

impl Effect {
    fn render_frame(
        self,
        frame: &mut Frame,
        percent_complete: u32,
        previous_effect: Option<Effect>,
        next_effect: Option<Effect>,
    ) {
        match self {
            Effect::Fade => {
                let mut previous_frame = [BLACK; LED_COUNT];
                let mut next_frame = [BLACK; LED_COUNT];

                if let Some(effect) = previous_effect {
                    effect.render_frame(&mut previous_frame, 100, None, None);
                }
                if let Some(effect) = next_effect {
                    effect.render_frame(&mut next_frame, 0, None, None);
                }
                for (element, (previous_element, next_element)) in frame
                    .iter_mut()
                    .zip(previous_frame.iter().zip(next_frame.iter()))
                {
                    *element =
                        interpolate_color(*next_element, *previous_element, percent_complete);
                }
            }
            Effect::SolidColor(color) => {
                for element in frame.iter_mut() {
                    *element = color;
                }
            }
            Effect::AlternatingColors(color1, color2) => {
                for (i, element) in frame.iter_mut().enumerate() {
                    if i % 2 == 0 {
                        *element = color1;
                    } else {
                        *element = color2;
                    }
                }
            }
        }
    }
}

fn interpolate_color(a: RGB8, b: RGB8, percent: u32) -> RGB8 {
    RGB {
        r: interpolate(a.r as u32, b.r as u32, percent) as u8,
        g: interpolate(a.g as u32, b.g as u32, percent) as u8,
        b: interpolate(a.b as u32, b.b as u32, percent) as u8,
    }
}

fn interpolate(a: u32, b: u32, percent: u32) -> u32 {
    let a_i = a as i32;
    let b_i = b as i32;
    let percent_i = percent as i32;
    let result_i = a_i + (b_i - a_i) * percent_i / 100;
    result_i as u32
}
