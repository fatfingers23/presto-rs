///Heavily WIP and only being developed for the ST7701 with the presto display
/// and the embassy framework.
use embassy_rp::{
    peripherals::{PIN_45, PWM_SLICE10},
    pwm::{Config, Pwm, SetDutyCycle},
};
use embassy_time::Timer;

const BACKLIGHT_PWM_TOP: u16 = 6_200;

pub struct ST7701 {
    pwm_backlight: Pwm<'static>,
}

impl ST7701 {
    pub fn new(lcd_bl: PIN_45, lcd_pwm_slice: PWM_SLICE10) -> Self {
        let mut c = Config::default();
        let desired_freq_hz = 25_000;
        let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
        let divider = 16u8;
        let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;
        c.top = period;
        c.divider = divider.into();
        // Initialize PWM with the given
        let mut pwm = Pwm::new_output_b(lcd_pwm_slice, lcd_bl, c.clone());

        Self { pwm_backlight: pwm }
    }

    pub async fn set_backlight(&mut self, brightness: u8) {
        let _ = self.pwm_backlight.set_duty_cycle_percent(brightness);
        Timer::after_millis(10).await;
    }
}
