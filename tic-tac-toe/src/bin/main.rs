#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::{delay, gpio, main, spi, time};
use esp_println as _;
use game::Game;
use st7735_lcd::{Orientation, ST7735};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let spi_bus = spi::master::Spi::new(
        peripherals.SPI2,
        spi::master::Config::default().with_frequency(time::Rate::from_mhz(5)),
    )
    .unwrap()
    .with_sck(peripherals.GPIO1)
    .with_mosi(peripherals.GPIO2);
    let cs = gpio::Output::new(
        peripherals.GPIO40,
        gpio::Level::High,
        gpio::OutputConfig::default(),
    );
    let spi_dev = ExclusiveDevice::new(spi_bus, cs, delay::Delay::new()).unwrap();

    let dc = gpio::Output::new(
        peripherals.GPIO42,
        gpio::Level::Low,
        gpio::OutputConfig::default(),
    );
    let rst = gpio::Output::new(
        peripherals.GPIO41,
        gpio::Level::Low,
        gpio::OutputConfig::default(),
    );

    let rgb = true;
    let inverted = false;
    let mut disp = ST7735::new(spi_dev, dc, rst, rgb, inverted, 160, 128);

    let mut delay = delay::Delay::new();
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();

    info!("Display setup finished!");

    let game = Game::new();
    game.draw(&mut disp).unwrap();

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
