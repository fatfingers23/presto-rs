use embassy_rp::{
    peripherals::{PIN_43, PWM_SLICE9},
    pwm::{Config, Pwm, SetDutyCycle},
};
use embassy_time::Timer;

fn calc_note(freq: f32) -> u16 {
    (12_000_000 as f32 / freq) as u16
}

#[derive(Clone, Copy)]
pub enum Notes {
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    Space,
}

impl Notes {
    pub fn get_note_freq(&self) -> f32 {
        match self {
            Notes::C4 => 261.63,
            Notes::D4 => 293.66,
            Notes::E4 => 329.63,
            Notes::F4 => 349.23,
            Notes::G4 => 392.00,
            Notes::A4 => 440.00,
            Notes::B4 => 493.88,
            Notes::Space => 0.0,
        }
    }

    pub fn note(&self) -> u16 {
        match self {
            Notes::C4 => calc_note(261.63),
            Notes::D4 => calc_note(293.66),
            Notes::E4 => calc_note(329.63),
            Notes::F4 => calc_note(349.23),
            Notes::G4 => calc_note(392.00),
            Notes::A4 => calc_note(440.00),
            Notes::B4 => calc_note(493.88),
            Notes::Space => 0,
        }
    }
}

pub struct Audio<'d> {
    pwm: Pwm<'d>,
}

impl<'d> Audio<'d> {
    pub fn new(pwm_slice: PWM_SLICE9, buzzer_pin: PIN_43) -> Audio<'d> {
        let c = Config::default();
        let pwm = Pwm::new_output_b(pwm_slice, buzzer_pin, c.clone());
        Audio { pwm }
    }

    pub async fn play(&mut self, note: Notes) {
        let note = note.note();
        let mut c = Config::default();
        c.compare_b = 573;
        c.top = note;
        // c.divider = FixedU16::from_bits(2);
        self.pwm.set_config(&c);
        let _ = self.pwm.set_duty_cycle_percent(50);

        Timer::after_millis(500).await;

        let _ = self.pwm.set_duty_cycle_percent(0);
        Timer::after_millis(100).await;
    }
}

//Songs

pub const TWINKLE_TWINKLE: [Notes; 48] = [
    Notes::C4,
    Notes::C4,
    Notes::G4,
    Notes::G4,
    Notes::A4,
    Notes::A4,
    Notes::G4,
    Notes::Space,
    Notes::F4,
    Notes::F4,
    Notes::E4,
    Notes::E4,
    Notes::D4,
    Notes::D4,
    Notes::C4,
    Notes::Space,
    Notes::G4,
    Notes::G4,
    Notes::F4,
    Notes::F4,
    Notes::E4,
    Notes::E4,
    Notes::D4,
    Notes::Space,
    Notes::G4,
    Notes::G4,
    Notes::F4,
    Notes::F4,
    Notes::E4,
    Notes::E4,
    Notes::D4,
    Notes::Space,
    Notes::C4,
    Notes::C4,
    Notes::G4,
    Notes::G4,
    Notes::A4,
    Notes::A4,
    Notes::G4,
    Notes::Space,
    Notes::F4,
    Notes::F4,
    Notes::E4,
    Notes::E4,
    Notes::D4,
    Notes::D4,
    Notes::C4,
    Notes::Space,
];
