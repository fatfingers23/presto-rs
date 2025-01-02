///Heavily WIP and only being developed for the ST7701 with the presto display
/// and the embassy framework
use byte_slice_cast::AsByteSlice;
use commands::LcdCommand;
use embassy_rp::{
    gpio::Output,
    peripherals::{PIN_45, PWM_SLICE10, SPI1},
    pwm::{Config, Pwm, SetDutyCycle},
    spi::{Async, Spi},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

mod commands;

pub struct ST7701 {
    pwm_backlight: Pwm<'static>,
    spi: &'static mut Mutex<NoopRawMutex, Spi<'static, SPI1, Async>>,
    cs: Output<'static>,
}

pub fn setup_backlight_pwm(lcd_bl: PIN_45, lcd_pwm_slice: PWM_SLICE10) -> Pwm<'static> {
    let mut c = Config::default();
    //What the data sheet says is max and embassy example says to find the frequency
    // let desired_freq_hz = 25_000;
    // let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    // let period: u16 = (clock_freq_hz / desired_freq_hz) as u16 - 1;

    //What is in the c library
    c.top = 6_200;
    c.divider = divider.into();

    Pwm::new_output_b(lcd_pwm_slice, lcd_bl, c.clone())
}

impl ST7701 {
    pub fn new(
        lcd_bl: PIN_45,
        lcd_pwm_slice: PWM_SLICE10,
        spi_bus: &'static mut Mutex<NoopRawMutex, Spi<'static, SPI1, Async>>,
        lcd_cs: Output<'static>,
    ) -> Self {
        //Setup brightness pwm and turn it off asap as we setup the display.
        //By default it's lit up
        let mut pwm = setup_backlight_pwm(lcd_bl, lcd_pwm_slice);
        let _ = pwm.set_duty_cycle_percent(0);

        Self {
            pwm_backlight: pwm,
            spi: spi_bus,
            cs: lcd_cs,
        }
    }

    pub async fn init(&mut self) {
        // Software reset
        self.command(LcdCommand::SWRESET, None).await;
        Timer::after(Duration::from_millis(150)).await;

        // Command 2 BK0 - Page select
        self.command(
            LcdCommand::CND2BKxSEL,
            Some(&[0x77, 0x01, 0x00, 0x00, 0x10]),
        )
        .await;

        // self-specific configuration
        self.command(LcdCommand::MADCTL, Some(&[0x00])).await; // Normal scan direction and RGB pixels
        self.command(LcdCommand::LNESET, Some(&[0x3B, 0x00])).await; // (59 + 1) * 8 = 480 lines
        self.command(LcdCommand::PORCTRL, Some(&[0x0D, 0x05])).await; // 13 VBP, 5 VFP
        self.command(LcdCommand::INVSET, Some(&[0x32, 0x05])).await;
        self.command(LcdCommand::COLCTRL, Some(&[0x08])).await; // LED polarity reversed
        self.command(
            LcdCommand::PVGAMCTRL,
            Some(&[
                0x00, 0x11, 0x18, 0x0E, 0x11, 0x06, 0x07, 0x08, 0x07, 0x22, 0x04, 0x12, 0x0F, 0xAA,
                0x31, 0x18,
            ]),
        )
        .await;
        self.command(
            LcdCommand::NVGAMCTRL,
            Some(&[
                0x00, 0x11, 0x19, 0x0E, 0x12, 0x07, 0x08, 0x08, 0x08, 0x22, 0x04, 0x11, 0x11, 0xA9,
                0x32, 0x18,
            ]),
        )
        .await;

        // Command 2 BK1 - Voltages and power settings
        self.command(
            LcdCommand::CND2BKxSEL,
            Some(&[0x77, 0x01, 0x00, 0x00, 0x11]),
        )
        .await;
        self.command(LcdCommand::PVGAMCTRL, Some(&[0x60])).await; // 4.7375v
        self.command(LcdCommand::NVGAMCTRL, Some(&[0x32])).await; // 0.725v
        self.command(LcdCommand::VGHSS, Some(&[0x07])).await; // 15v
        self.command(LcdCommand::TESTCMD, Some(&[0x80])).await; // y tho?
        self.command(LcdCommand::VGLS, Some(&[0x49])).await; // -10.17v
        self.command(LcdCommand::PWCTRL1, Some(&[0x85])).await; // Middle/Min/Min bias
        self.command(LcdCommand::PWCTRL2, Some(&[0x21])).await; // 6.6 / -4.6
        self.command(LcdCommand::PORCTRL, Some(&[0x78])).await; // 1.6uS
        self.command(LcdCommand::INVSET, Some(&[0x78])).await; // 6.4uS

        // Begin Forbidden Knowledge
        // This sequence is probably specific to TL040WVS03CT15-H1263A.
        // It is not documented in the ST7701s datasheet.
        // TODO: 👇 W H A T ! ? 👇
        self.command(LcdCommand::SRECTRL, Some(&[0x00, 0x1B, 0x02]))
            .await;
        self.command(
            LcdCommand::NRCTRL,
            Some(&[
                0x08, 0xA0, 0x00, 0x00, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x44, 0x44,
            ]),
        )
        .await;
        self.command(
            LcdCommand::SECTRL,
            Some(&[
                0x11, 0x11, 0x44, 0x44, 0xED, 0xA0, 0x00, 0x00, 0xEC, 0xA0, 0x00, 0x00,
            ]),
        )
        .await;
        self.command(LcdCommand::SECTRL, Some(&[0x00, 0x00, 0x11, 0x11]))
            .await;
        self.command(LcdCommand::SKCTRL, Some(&[0x44, 0x44])).await;
        self.command(
            LcdCommand::FORBIDDEN6,
            Some(&[
                0x0A, 0xE9, 0xD8, 0xA0, 0x0C, 0xEB, 0xD8, 0xA0, 0x0E, 0xED, 0xD8, 0xA0, 0x10, 0xEF,
                0xD8, 0xA0,
            ]),
        )
        .await;
        self.command(LcdCommand::FORBIDDEN7, Some(&[0x00, 0x00, 0x11, 0x11]))
            .await;
        self.command(LcdCommand::FORBIDDEN8, Some(&[0x44, 0x44]))
            .await;
        self.command(
            LcdCommand::FORBIDDEN9,
            Some(&[
                0x09, 0xE8, 0xD8, 0xA0, 0x0B, 0xEA, 0xD8, 0xA0, 0x0D, 0xEC, 0xD8, 0xA0, 0x0F, 0xEE,
                0xD8, 0xA0,
            ]),
        )
        .await;
        self.command(
            LcdCommand::FORBIDDEN10,
            Some(&[0x02, 0x00, 0xE4, 0xE4, 0x88, 0x00, 0x40]),
        )
        .await;
        self.command(LcdCommand::FORBIDDEN11, Some(&[0x3C, 0x00]))
            .await;
        self.command(
            LcdCommand::FORBIDDEN12,
            Some(&[
                0xAB, 0x89, 0x76, 0x54, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x20, 0x45, 0x67,
                0x98, 0xBA,
            ]),
        )
        .await;
        // end Forbidden Knowledge

        self.command(LcdCommand::MADCTL, Some(&[0x00])).await;

        // Command 2 BK3
        self.command(
            LcdCommand::CND2BKxSEL,
            Some(&[0x77, 0x01, 0x00, 0x00, 0x13]),
        )
        .await;
        self.command(LcdCommand::COLMOD, Some(&[0x66])).await; // 18 bits per pixel

        self.command(LcdCommand::INVON, None).await;
        Timer::after(Duration::from_millis(1)).await;
        self.command(LcdCommand::SLPOUT, None).await;
        Timer::after(Duration::from_millis(120)).await;
        self.command(LcdCommand::DISPON, None).await;
        Timer::after(Duration::from_millis(50)).await;
    }

    pub async fn command(&mut self, command: LcdCommand, data: Option<&[u8]>) {
        const CMD: u16 = 0b0 << 8; // 9th bit as 0 for command
        const DATA: u16 = 0b1 << 8; // 9th bit as 1 for data

        let mut buffer = [0u16; 20]; // Adjust size as needed

        // Pull CS low to start the SPI transaction
        self.cs.set_low();

        // Send the command with the CMD flag
        let command_frame = CMD | (command as u16);
        self.spi
            .get_mut()
            .write(&[command_frame].as_byte_slice())
            .await
            .unwrap();
        // self.spi.write(&[command_frame]).unwrap();

        if let Some(data) = data {
            // Encode the data bytes with the DATA flag
            for (i, &byte) in data.iter().enumerate() {
                buffer[i] = DATA | (byte as u16);
            }
            self.spi
                .get_mut()
                .write(buffer[..data.len()].as_byte_slice())
                .await
                .unwrap();
        }

        // Pull CS high to end the SPI transaction
        self.cs.set_high();
    }

    pub fn set_backlight(&mut self, brightness: u8) {
        let _ = self.pwm_backlight.set_duty_cycle_percent(brightness);
    }
}
