#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::{info, unwrap, Format};
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy::traits::gpio::{WaitForHigh, WaitForLow};
use embassy_nrf::gpio::{AnyPin, Input, Pin as _, Pull};
use embassy_nrf::Peripherals;
use embedded_hal::digital::v2::InputPin;

#[embassy::task(pool_size = 9)]
async fn button_task(id: Button, mut pin: Input<'static, AnyPin>) {
    loop {
        pin.wait_for_low().await;
        Timer::after(Duration::from_millis(25)).await; // Debounce
        if unwrap!(pin.is_low()) {
            info!("Button {} was pressed", id);
            pin.wait_for_high().await;
            info!("Button {} was released", id);
        }
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Press a button");
    let btn1 = Input::new(p.P1_13.degrade(), Pull::Up);
    let btn2 = Input::new(p.P1_15.degrade(), Pull::Up);
    let btn3 = Input::new(p.P1_09.degrade(), Pull::Up);
    let btn4 = Input::new(p.P0_12.degrade(), Pull::Up);

    let nav_up = Input::new(p.P0_02.degrade(), Pull::Up);
    let nav_l = Input::new(p.P0_10.degrade(), Pull::Up);
    let nav_dn = Input::new(p.P0_29.degrade(), Pull::Up);
    let nav_r = Input::new(p.P0_09.degrade(), Pull::Up);
    let nav_c = Input::new(p.P1_00.degrade(), Pull::Up);

    unwrap!(spawner.spawn(button_task(Button::A, btn1)));
    unwrap!(spawner.spawn(button_task(Button::B, btn2)));
    unwrap!(spawner.spawn(button_task(Button::C, btn3)));
    unwrap!(spawner.spawn(button_task(Button::D, btn4)));
    unwrap!(spawner.spawn(button_task(Button::Up, nav_up)));
    unwrap!(spawner.spawn(button_task(Button::Left, nav_l)));
    unwrap!(spawner.spawn(button_task(Button::Down, nav_dn)));
    unwrap!(spawner.spawn(button_task(Button::Right, nav_r)));
    unwrap!(spawner.spawn(button_task(Button::Center, nav_c)));
}

#[derive(Clone, Copy, Format)]
enum Button {
    A,
    B,
    C,
    D,
    Up,
    Left,
    Down,
    Right,
    Center,
}
