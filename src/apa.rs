use blinkt::Blinkt;

pub struct Manager {
    channels: u8,
}

impl Manager {
    fn bootstrap(channels: u8, pipe: tokio::sync::broadcast::Receiever<Vec<util::Color>>) {
        let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;
    }
}
