#![no_main]
#![no_std]

// This template uses MCU-Link as a target.
// MCU-Link only has a single LED to play with, so we'll use that for our demo

use cortex_m_rt::entry;

use hal::{drivers::pins::Level, prelude::*};
use lpc55_hal as hal;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    defmt::info!("LPC55 Rust program start!");
    let hal = hal::new();

    let mut anactrl = hal.anactrl;
    let mut pmc = hal.pmc;
    let mut syscon = hal.syscon;

    let clocks = hal::ClockRequirements::default()
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .unwrap();

    let token = clocks.support_utick_token().unwrap();
    let mut utick = hal.utick.enabled(&mut syscon, &token);

    let mut gpio = hal.gpio.enabled(&mut syscon);
    let mut iocon = hal.iocon.enabled(&mut syscon);
    let pins = hal::Pins::take().unwrap();

    let mut red = pins
        .pio0_5
        .into_gpio_pin(&mut iocon, &mut gpio)
        // on = low, off = high
        .into_output(Level::High);

    loop {
        info!("on!");
        red.set_low().unwrap();
        utick.start(1_000_000u32);
        utick.blocking_wait();
                
        info!("off!");
        red.set_high().unwrap();
        utick.start(1_000_000u32);
        utick.blocking_wait();
    }
}
