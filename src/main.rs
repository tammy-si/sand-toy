#![no_std]
#![no_main]

mod max7291;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{Config, gpio::{Level, Output, Speed}};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

defmt::timestamp!("{=u32:us}", {
    0u32
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let spi_config = embassy_stm32::spi::Config::default();
    let spi = embassy_stm32::spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi_config);
    let max_cs = Output::new(p.PA4, Level::High, Speed::VeryHigh);
    let mut display= max7291::Max7291::new(spi, max_cs);
    info!("inited");
    // test
    display.init().unwrap();
    display.set_pixel(0, 0, true);
    display.set_pixel(0, 1, true);
    // display.set_pixel(0, 2, true);
    display.set_pixel(1, 1, true);
    // display.set_pixel(1, 3, true);

    display.flush_display().unwrap();

    info!("wrote to display");
    loop {

    }
}
