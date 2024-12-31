use crate::Irqs;
use cyw43::Control;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net_wiznet::Device as NetDevice;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH1, PIO1};
use embassy_rp::peripherals::{PIN_23, PIN_24, PIN_25, PIN_29};
use embassy_rp::pio::Pio;
use static_cell::StaticCell;

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn rm2_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO1, 0, DMA_CH1>>,
) -> ! {
    runner.run().await
}

pub async fn setup_rm2<'a>(
    pio0: PIO1,
    p_23: PIN_23,
    p_24: PIN_24,
    p_25: PIN_25,
    p_29: PIN_29,
    dma_ch0: DMA_CH1,
    spawner: Spawner,
) -> (NetDevice<'a>, Control<'a>) {
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p_23, Level::Low);
    let cs = Output::new(p_25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p_24,
        p_29,
        dma_ch0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    spawner.must_spawn(rm2_task(runner));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;
    (net_device, control)
}
