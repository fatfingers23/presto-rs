use embassy_rp::{
    peripherals::{DMA_CH0, PIN_33, PIO0},
    pio::{Common, StateMachine},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use smart_leds::RGB8;

pub struct Leds<'d> {
    pub ws2812: PioWs2812<'d, PIO0, 0, 7>,
    pub lights: [RGB8; 7],
}

//TODO write documentation
impl<'d> Leds<'d> {
    pub fn new(
        mut common: &mut Common<'d, PIO0>,
        sm0: StateMachine<'d, PIO0, 0>,
        dma: DMA_CH0,
        pin: PIN_33,
    ) -> Self {
        // let Pio {
        //     mut common, sm0, ..
        // } = Pio::new(pio, Irqs);

        const NUM_LEDS: usize = 7;

        let program = PioWs2812Program::new(common);
        let ws2812 = PioWs2812::new(&mut common, sm0, dma, pin, &program);
        Self {
            ws2812,
            lights: [RGB8::default(); NUM_LEDS],
        }
    }

    pub fn set_light(&mut self, index: usize, color: RGB8) {
        self.lights[index] = color;
    }

    pub async fn update(&mut self) {
        self.ws2812.write(&self.lights).await;
    }

    pub async fn off(&mut self, index: usize) {
        self.set_light(index, RGB8::default());
        self.update().await;
    }

    pub async fn all_off(&mut self) {
        for i in 0..self.lights.len() {
            self.off(i).await;
        }
    }

    pub fn set_light_brightness(&mut self, index: usize, brightness: u8) {
        self.set_light(
            index,
            self.get_color_brightness(self.lights[index], brightness),
        );
    }

    pub fn set_light_with_brightness(&mut self, index: usize, color: RGB8, brightness: u8) {
        self.set_light(index, self.get_color_brightness(color, brightness));
    }

    fn get_color_brightness(&self, color: RGB8, brightness: u8) -> RGB8 {
        RGB8 {
            r: (color.r as u16 * (brightness as u16 + 1) / 256) as u8,
            g: (color.g as u16 * (brightness as u16 + 1) / 256) as u8,
            b: (color.b as u16 * (brightness as u16 + 1) / 256) as u8,
        }
    }
}
