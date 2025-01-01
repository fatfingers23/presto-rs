#![no_std]

use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, PIO1},
    pio::InterruptHandler,
};

pub mod audio;
pub mod io;
pub mod leds;
pub mod peripherals;
pub mod rm2_driver;
mod st7701;

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;

});
