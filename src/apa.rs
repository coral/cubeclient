use blinkt;
use rppal::spi;

use crate::util;

pub struct Manager {
    num_pixels: u16,
    channels: u8,
    strips: Vec<blinkt::Blinkt>,
    pipe: tokio::sync::broadcast::Receiver<Vec<util::Color>>,
}

impl Manager {
    pub fn bootstrap(
        num_pixels: u16,
        channels: u8,
        clock_speed: u32,
        pipe: tokio::sync::broadcast::Receiver<Vec<util::Color>>,
    ) -> Manager {
        let wrk = [spi::Bus::Spi0, spi::Bus::Spi1, spi::Bus::Spi2];

        let mut tp = Vec::new();
        let pixels_per_strip = (num_pixels / 2) as usize;
        for n in 0..channels {
            let b = blinkt::BlinktSpi {
                spi: spi::Spi::new(
                    wrk[n as usize],
                    spi::SlaveSelect::Ss0,
                    clock_speed,
                    spi::Mode::Mode0,
                )
                .unwrap(),
            };

            tp.push(blinkt::Blinkt {
                serial_output: Box::new(b),
                pixels: vec![blinkt::Pixel::default(); pixels_per_strip],
                clear_on_drop: true,
                end_frame: vec![
                    0u8;
                    4 + (((pixels_per_strip as f32 / 16.0f32) + 0.94f32) as usize)
                ],
            });
        }
        return Manager {
            num_pixels,
            channels,
            strips: tp,
            pipe,
        };
    }

    pub async fn spin(&mut self) {
        loop {
            let d = self.pipe.recv().await.unwrap();
            for (di, strip) in self.strips.iter_mut().enumerate() {
                for (i, pixel) in strip.pixels.iter_mut().enumerate() {
                    pixel.value = [255, d[di * i].r, d[di * i].g, d[di * i].b];
                }
            }
            for strip in &mut self.strips {
                strip.show();
            }
        }
    }
}
