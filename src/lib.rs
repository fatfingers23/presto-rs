#![no_std]

use embassy_rp::{bind_interrupts, peripherals::PIO1, pio::InterruptHandler};

pub mod leds;
pub mod peripherals;

bind_interrupts!(pub struct Irqs {
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
});

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
