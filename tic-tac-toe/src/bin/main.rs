#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::{Cell, RefCell};

use critical_section::Mutex;
use defmt::info;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcCalBasic, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Io, Pull};
use esp_hal::{delay, gpio, main, spi, time};
use esp_println as _;
use game::logic::{Cmd, Game, GameState, PlayerResult};
use st7735_lcd::{Orientation, ST7735};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static SEL_PIN: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static IS_PIN_PRESS: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

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

    // Joystick's axes
    let mut adc_config = AdcConfig::new();
    let mut adc_pin_x_axis =
        adc_config.enable_pin_with_cal::<_, AdcCalBasic<_>>(peripherals.GPIO9, Attenuation::_11dB);
    let mut adc_pin_y_axis =
        adc_config.enable_pin_with_cal::<_, AdcCalBasic<_>>(peripherals.GPIO10, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    // Joystick's button
    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    let mut sel_pin = Input::new(
        peripherals.GPIO11,
        InputConfig::default().with_pull(Pull::Up),
    );

    critical_section::with(|cs| {
        // Enter critical section before listen to
        // an event to prevent an interrupt firing
        // before the select pin has been set up
        sel_pin.listen(gpio::Event::FallingEdge);
        SEL_PIN.borrow_ref_mut(cs).replace(sel_pin);
    });

    let mut game = Game::new();
    let mut pre_x_axis_state = AxisState::Med;
    let mut pre_y_axis_state = AxisState::Med;

    // Draw first frame
    game.draw(&mut disp).unwrap();

    loop {
        use game::logic::Direction;

        const LOW_LIMIT: u16 = 200;
        const HI_LIMIT: u16 = 2800;

        let mut select = None;
        critical_section::with(|cs| {
            if IS_PIN_PRESS.borrow(cs).get() {
                select = Some(Cmd::Select);
                delay.delay_millis(200);

                SEL_PIN
                    .borrow_ref_mut(cs)
                    .as_mut()
                    .unwrap()
                    .listen(gpio::Event::FallingEdge);

                IS_PIN_PRESS.borrow(cs).replace(false);
            }
        });
        match select {
            Some(select) => {
                match game.get_state() {
                    GameState::Menu | GameState::Result => disp.clear(Rgb565::BLACK).unwrap(),
                    GameState::GamePlay => {}
                }
                game.handle_input(select);
                if let GameState::GamePlay = game.get_state() {
                    match game.check_player_one_result() {
                        PlayerResult::Won | PlayerResult::Lost | PlayerResult::Drew => {
                            disp.clear(Rgb565::BLACK).unwrap();
                            game.show_result();
                        }
                        PlayerResult::OnGoing => {}
                    }
                }
                game.draw(&mut disp).unwrap();
            }
            None => {}
        }

        let x_adc_val = adc.read_blocking(&mut adc_pin_x_axis);
        let cur_x_axis_state = if x_adc_val > HI_LIMIT {
            AxisState::Hi
        } else if x_adc_val < LOW_LIMIT {
            AxisState::Low
        } else {
            AxisState::Med
        };
        if cur_x_axis_state != pre_x_axis_state {
            match cur_x_axis_state {
                AxisState::Hi => {
                    game.handle_input(Cmd::Move(Direction::Down));

                    game.draw(&mut disp).unwrap();
                }
                AxisState::Low => {
                    game.handle_input(Cmd::Move(Direction::Up));
                    game.draw(&mut disp).unwrap();
                }
                AxisState::Med => {}
            }
        };
        pre_x_axis_state = cur_x_axis_state;

        let y_adc_val = adc.read_blocking(&mut adc_pin_y_axis);
        let cur_y_axis_state = if y_adc_val > HI_LIMIT {
            AxisState::Hi
        } else if y_adc_val < LOW_LIMIT {
            AxisState::Low
        } else {
            AxisState::Med
        };
        if cur_y_axis_state != pre_y_axis_state {
            match cur_y_axis_state {
                AxisState::Hi => {
                    game.handle_input(Cmd::Move(Direction::Right));
                    game.draw(&mut disp).unwrap();
                }
                AxisState::Low => {
                    game.handle_input(Cmd::Move(Direction::Left));
                    game.draw(&mut disp).unwrap();
                }
                AxisState::Med => {}
            }
        }
        pre_y_axis_state = cur_y_axis_state;

        delay.delay_millis(10);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}

#[derive(PartialEq, Eq, Debug)]
enum AxisState {
    Low,
    Med,
    Hi,
}

#[esp_hal::handler]
fn handler() {
    critical_section::with(|cs| {
        let mut sel_pin = SEL_PIN.borrow_ref_mut(cs);
        let Some(sel_pin) = sel_pin.as_mut() else {
            // Some other interrupt has occured
            // before the select pin was set up.
            return;
        };

        if sel_pin.is_interrupt_set() {
            IS_PIN_PRESS.borrow(cs).set(true);
            sel_pin.clear_interrupt();
            sel_pin.unlisten();
        }
    })
}
