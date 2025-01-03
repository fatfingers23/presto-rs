///Heavily WIP and only being developed for the ST7701 with the presto display
/// and the embassy framework
use byte_slice_cast::AsByteSlice;
use commands::LcdCommand;
use cyw43::State;
use embassy_rp::{
    gpio::{Level, Output},
    peripherals::*,
    pio::{Common, Direction, Irq, Pio, StateMachine},
    pwm::{Config, Pwm, SetDutyCycle},
    spi::{Async, Spi},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};
use fixed::{traits::ToFixed, types::U56F8};
use pio_proc::pio_file;

use crate::Irqs;

mod commands;

#[derive(PartialEq)]
pub enum Width {
    W240,
    W480,
}

impl Width {
    pub fn number(&self) -> u16 {
        match self {
            Width::W240 => 240,
            Width::W480 => 480,
        }
    }
}

pub struct ST7701 {
    pwm_backlight: Pwm<'static>,
    spi: &'static mut Mutex<NoopRawMutex, Spi<'static, SPI1, Async>>,
    cs: Output<'static>,
    parallel_sm: StateMachine<'static, PIO2, 0>,
    timing_sm: StateMachine<'static, PIO2, 1>,
    common: Common<'static, PIO2>,
    irq3: Irq<'static, PIO2, 3>,
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
        //GPIO 1-16 are rgb data lines
        pin_1: PIN_1,
        pin_2: PIN_2,
        pin_3: PIN_3,
        pin_4: PIN_4,
        pin_5: PIN_5,
        pin_6: PIN_6,
        pin_7: PIN_7,
        pin_8: PIN_8,
        pin_9: PIN_9,
        pin_10: PIN_10,
        pin_11: PIN_11,
        pin_12: PIN_12,
        pin_13: PIN_13,
        pin_14: PIN_14,
        pin_15: PIN_15,
        pin_16: PIN_16,
        //GPIO 17 and 18 are just pulled low for now. Used for 18 bit mode
        pin_17: PIN_17,
        pin_18: PIN_18,
        hsync: PIN_19,
        vsync: PIN_20,
        lcd_de: PIN_21,
        lcd_dot_clk: PIN_22,
        lcd_pwm_slice: PWM_SLICE10,
        spi_bus: &'static mut Mutex<NoopRawMutex, Spi<'static, SPI1, Async>>,
        lcd_cs: Output<'static>,
        pio: PIO2,
    ) -> Self {
        let width = Width::W240;
        //Setup brightness pwm and turn it off asap as we setup the display.
        //By default it's lit up
        let mut pwm = setup_backlight_pwm(lcd_bl, lcd_pwm_slice);
        let _ = pwm.set_duty_cycle_percent(0);

        let sys_clock = embassy_rp::clocks::clk_sys_freq();

        let Pio {
            mut common,
            irq3,
            mut sm0,
            sm1,
            ..
        } = Pio::new(pio, Irqs);

        //Pins setup
        let hsync_pio = common.make_pio_pin(hsync);
        let vsync_pio = common.make_pio_pin(vsync);
        let lcd_de = common.make_pio_pin(lcd_de);
        let lcd_dot_clk = common.make_pio_pin(lcd_dot_clk);

        //Setup data pins

        sm0.set_pin_dirs(Direction::Out, &[
            //Data pins
            &common.make_pio_pin(pin_1),
            &common.make_pio_pin(pin_2),
            &common.make_pio_pin(pin_3),
            &common.make_pio_pin(pin_4),
            &common.make_pio_pin(pin_5),
            &common.make_pio_pin(pin_6),
            &common.make_pio_pin(pin_7),
            &common.make_pio_pin(pin_8),
            &common.make_pio_pin(pin_9),
            &common.make_pio_pin(pin_10),
            &common.make_pio_pin(pin_11),
            &common.make_pio_pin(pin_12),
            &common.make_pio_pin(pin_13),
            &common.make_pio_pin(pin_14),
            &common.make_pio_pin(pin_15),
            &common.make_pio_pin(pin_16),
            //Other pins
            &hsync_pio,
            &vsync_pio,
            &lcd_dot_clk,
        ]);
        //Pull 17 and 18 low for now
        Output::new(pin_17, Level::Low);
        Output::new(pin_18, Level::Low);

        //Setup the Parallel PIO program
        let parallel_program = pio_proc::pio_file!(
            "src/st7701/pio/st7701_parallel.pio",
            select_program("st7701_parallel"), // Optional if only one program in the file
            options(max_program_size = 32)     // Optional, defaults to 32
        );
        let parallel_program = parallel_program.program;
        let mut cfg = embassy_rp::pio::Config::default();

        cfg.use_program(&common.load_program(&parallel_program), &[&lcd_de]);

        let max_pio_clk = 34_000_000;
        let clock_divider = (sys_clock + max_pio_clk - 1) / max_pio_clk;
        if width == Width::W480 {
            cfg.clock_divider = (clock_divider >> 1).to_fixed();
        } else {
            cfg.clock_divider = clock_divider.to_fixed();
        }

        sm0.set_config(&cfg);

        let y_set = pio::InstructionOperands::OUT {
            destination: pio::OutDestination::Y,
            bit_count: 32,
        };
        unsafe { sm0.exec_instr(y_set.encode()) };
        sm0.tx().push((width.number() as u32 >> 1) - 1);
        sm0.set_enable(true);

        //Setup the Timing PIO program

        Self {
            pwm_backlight: pwm,
            spi: spi_bus,
            cs: lcd_cs,
            parallel_sm: sm0,
            timing_sm: sm1,
            common,
            irq3,
        }
    }

    pub async fn init(&mut self) {
        //Init display with the commands
        self.init_spi_commands().await;
    }

    pub async fn init_spi_commands(&mut self) {
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
        self.set_backlight(100);
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
