#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy::traits::gpio::{WaitForHigh, WaitForLow};
use embassy_nrf::gpio::{AnyPin, Input, Pin as _, Pull};
use embassy_nrf::Peripherals;
use embedded_hal::digital::v2::InputPin;

#[embassy::task(pool_size = 4)]
async fn button_task(id: u8, mut pin: Input<'static, AnyPin>) {
    loop {
        pin.wait_for_low().await;
        Timer::after(Duration::from_millis(30)).await;
        if unwrap!(pin.is_low()) {
            info!("Button {} was pressed", id);
            pin.wait_for_high().await;
            info!("Button {} was released", id);
        }
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello world");
    let btn1 = Input::new(p.P1_13.degrade(), Pull::Up);
    let btn2 = Input::new(p.P1_15.degrade(), Pull::Up);
    let btn3 = Input::new(p.P1_09.degrade(), Pull::Up);
    let btn4 = Input::new(p.P0_12.degrade(), Pull::Up);

    unwrap!(spawner.spawn(button_task(1, btn1)));
    unwrap!(spawner.spawn(button_task(2, btn2)));
    unwrap!(spawner.spawn(button_task(3, btn3)));
    unwrap!(spawner.spawn(button_task(4, btn4)));
}
