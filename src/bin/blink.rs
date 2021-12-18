#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::Peripherals;
use embedded_hal::digital::v2::OutputPin;
use nrf_embassy as _; // global logger + panicking-behavior + memory layout

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    info!("Hello world");
    loop {
        unwrap!(led.set_high());
        Timer::after(Duration::from_millis(300)).await;
        unwrap!(led.set_low());
        Timer::after(Duration::from_millis(300)).await;
    }
}
