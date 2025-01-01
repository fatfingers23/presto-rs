#![allow(non_snake_case)]
use crate::{
    Irqs,
    audio::Audio,
    leds::Leds,
    st7701::{self, ST7701},
};
pub use embassy_rp::peripherals::*;
use embassy_rp::{
    config::Config,
    gpio::{Level, Output},
    pio::Pio,
};
use embassy_time::Timer;

#[allow(dead_code)]
pub struct Peripherals {
    pub PIN_0: PIN_0,
    pub PIN_1: PIN_1,
    pub PIN_3: PIN_3,
    pub PIN_4: PIN_4,
    pub PIN_5: PIN_5,
    pub PIN_6: PIN_6,
    pub PIN_7: PIN_7,
    pub PIN_8: PIN_8,
    pub PIN_9: PIN_9,

    //RM2 pins
    pub PIN_23: PIN_23,
    pub PIN_24: PIN_24,
    pub PIN_25: PIN_25,
    pub PIN_29: PIN_29,

    pub PIN_27: PIN_27,
    pub PIN_28: PIN_28,

    pub PIN_40: PIN_40,
    pub PIN_41: PIN_41,

    pub PIN_QSPI_SCLK: PIN_QSPI_SCLK,
    pub PIN_QSPI_SS: PIN_QSPI_SS,
    pub PIN_QSPI_SD0: PIN_QSPI_SD0,
    pub PIN_QSPI_SD1: PIN_QSPI_SD1,
    pub PIN_QSPI_SD2: PIN_QSPI_SD2,
    pub PIN_QSPI_SD3: PIN_QSPI_SD3,

    pub UART0: UART0,
    pub UART1: UART1,

    pub SPI0: SPI0,
    pub SPI1: SPI1,

    pub I2C0: I2C0,
    pub I2C1: I2C1,

    //DMA channel for the RM2
    pub DMA_CH1: DMA_CH1,

    pub DMA_CH2: DMA_CH2,
    pub DMA_CH3: DMA_CH3,
    pub DMA_CH4: DMA_CH4,
    pub DMA_CH5: DMA_CH5,
    pub DMA_CH6: DMA_CH6,
    pub DMA_CH7: DMA_CH7,
    pub DMA_CH8: DMA_CH8,
    pub DMA_CH9: DMA_CH9,
    pub DMA_CH10: DMA_CH10,
    pub DMA_CH11: DMA_CH11,
    pub DMA_CH12: DMA_CH12,
    pub DMA_CH13: DMA_CH13,
    pub DMA_CH14: DMA_CH14,
    pub DMA_CH15: DMA_CH15,

    pub PWM_SLICE0: PWM_SLICE0,
    pub PWM_SLICE1: PWM_SLICE1,
    pub PWM_SLICE2: PWM_SLICE2,
    pub PWM_SLICE3: PWM_SLICE3,
    pub PWM_SLICE4: PWM_SLICE4,
    pub PWM_SLICE5: PWM_SLICE5,
    pub PWM_SLICE7: PWM_SLICE7,
    pub PWM_SLICE8: PWM_SLICE8,
    // pub PWM_SLICE9: PWM_SLICE9,
    // pub PWM_SLICE10: PWM_SLICE10,
    pub PWM_SLICE11: PWM_SLICE11,

    pub USB: USB,

    pub RTC: RTC,

    pub FLASH: FLASH,

    pub ADC: ADC,
    pub ADC_TEMP_SENSOR: ADC_TEMP_SENSOR,

    pub CORE1: CORE1,

    // pub PIO0: PIO0,
    //RM2 PIO
    pub PIO1: PIO1,
    pub PIO2: PIO2,

    pub WATCHDOG: WATCHDOG,
    pub BOOTSEL: BOOTSEL,

    pub TRNG: TRNG,
    //Hey Presto
    pub LEDS: Leds<'static>,
    pub BUZZER: Audio<'static>,
    pub ST7701: ST7701,
}

pub async fn init(config: Config) -> Peripherals {
    let p = embassy_rp::init(config);

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    // Output::new(p.PIN_45, Level::Low);
    let mut display = ST7701::new(p.PIN_45, p.PWM_SLICE10);

    // test.set_backlight(1);

    Peripherals {
        PIN_0: p.PIN_0,
        PIN_1: p.PIN_1,
        PIN_3: p.PIN_3,
        PIN_4: p.PIN_4,
        PIN_5: p.PIN_5,
        PIN_6: p.PIN_6,
        PIN_7: p.PIN_7,
        PIN_8: p.PIN_8,
        PIN_9: p.PIN_9,
        //RM2 pins
        PIN_23: p.PIN_23,
        PIN_24: p.PIN_24,
        PIN_25: p.PIN_25,
        PIN_29: p.PIN_29,

        PIN_27: p.PIN_27,
        PIN_28: p.PIN_28,

        PIN_40: p.PIN_40,
        PIN_41: p.PIN_41,

        PIN_QSPI_SCLK: p.PIN_QSPI_SCLK,
        PIN_QSPI_SS: p.PIN_QSPI_SS,
        PIN_QSPI_SD0: p.PIN_QSPI_SD0,
        PIN_QSPI_SD1: p.PIN_QSPI_SD1,
        PIN_QSPI_SD2: p.PIN_QSPI_SD2,
        PIN_QSPI_SD3: p.PIN_QSPI_SD3,

        UART0: p.UART0,
        UART1: p.UART1,

        SPI0: p.SPI0,
        SPI1: p.SPI1,

        I2C0: p.I2C0,
        I2C1: p.I2C1,

        // DMA_CH0: p.DMA_CH0,
        DMA_CH1: p.DMA_CH1,
        DMA_CH2: p.DMA_CH2,
        DMA_CH3: p.DMA_CH3,
        DMA_CH4: p.DMA_CH4,
        DMA_CH5: p.DMA_CH5,
        DMA_CH6: p.DMA_CH6,
        DMA_CH7: p.DMA_CH7,
        DMA_CH8: p.DMA_CH8,
        DMA_CH9: p.DMA_CH9,
        DMA_CH10: p.DMA_CH10,
        DMA_CH11: p.DMA_CH11,
        DMA_CH12: p.DMA_CH12,
        DMA_CH13: p.DMA_CH13,
        DMA_CH14: p.DMA_CH14,
        DMA_CH15: p.DMA_CH15,

        PWM_SLICE0: p.PWM_SLICE0,
        PWM_SLICE1: p.PWM_SLICE1,
        PWM_SLICE2: p.PWM_SLICE2,
        PWM_SLICE3: p.PWM_SLICE3,
        PWM_SLICE4: p.PWM_SLICE4,
        PWM_SLICE5: p.PWM_SLICE5,
        PWM_SLICE7: p.PWM_SLICE7,
        PWM_SLICE8: p.PWM_SLICE8,
        // PWM_SLICE9: p.PWM_SLICE9,
        // PWM_SLICE10: p.PWM_SLICE10,
        PWM_SLICE11: p.PWM_SLICE11,

        USB: p.USB,

        RTC: p.RTC,

        FLASH: p.FLASH,

        ADC: p.ADC,
        ADC_TEMP_SENSOR: p.ADC_TEMP_SENSOR,

        CORE1: p.CORE1,

        // PIO0: p.PIO0,
        //RM2 PIO
        PIO1: p.PIO1,
        PIO2: p.PIO2,

        WATCHDOG: p.WATCHDOG,
        BOOTSEL: p.BOOTSEL,

        TRNG: p.TRNG,
        //Hey Presto
        LEDS: Leds::new(&mut common, sm0, p.DMA_CH0, p.PIN_33),
        BUZZER: Audio::new(p.PWM_SLICE9, p.PIN_43),
        ST7701: display,
    }
}
