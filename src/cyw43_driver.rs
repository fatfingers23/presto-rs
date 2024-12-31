use crate::Irqs;
use cyw43::Control;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net_wiznet::Device as NetDevice;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::peripherals::{PIN_23, PIN_24, PIN_25, PIN_29};
use embassy_rp::pio::{self, Common, Irq, Pio, StateMachine};
use static_cell::StaticCell;

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 1, DMA_CH1>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

pub async fn setup_cyw43<'d>(
    mut common: &mut Common<'static, PIO0>,
    sm1: StateMachine<'static, PIO0, 1>,
    irq0: Irq<'static, PIO0, 0>,
    p_23: PIN_23,
    p_24: PIN_24,
    p_25: PIN_25,
    p_29: PIN_29,
    dma: DMA_CH1,
    spawner: Spawner,
) -> (NetDevice<'d>, Control<'d>) {
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    // let btfw = include_bytes!("../cyw43-firmware/43439A0_btfw.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    //     probe-rs download 43439A0_btfw.bin --format bin --chip RP2040 --base-address 0x10141400
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };
    //let btfw = unsafe { core::slice::from_raw_parts(0x10141400 as *const u8, 6164) };

    let pwr = Output::new(p_23, Level::Low);
    let cs = Output::new(p_25, Level::High);

    let spi = PioSpi::new(
        &mut common,
        sm1,
        RM2_CLOCK_DIVIDER,
        irq0,
        cs,
        p_24,
        p_29,
        dma,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    spawner.must_spawn(cyw43_task(runner));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;
    (net_device, control)
}
