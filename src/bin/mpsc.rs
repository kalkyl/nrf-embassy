#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy::blocking_mutex::raw::NoopRawMutex;
use embassy::util::Forever;
use nrf_embassy as _; // global logger + panicking-behavior + memory layout

use defmt::{info, unwrap, Format};
use embassy::channel::mpmc::{Channel, Sender};
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{AnyPin, Input, Pin as _, Pull};
use embassy_nrf::Peripherals;

static CHANNEL: Forever<Channel<NoopRawMutex, ButtonEvent, 1>> = Forever::new();

#[embassy::task(pool_size = 9)]
async fn button_task(
    sender: Sender<'static, NoopRawMutex, ButtonEvent, 1>,
    id: Button,
    mut pin: Input<'static, AnyPin>,
) {
    loop {
        pin.wait_for_low().await;
        Timer::after(Duration::from_millis(25)).await; // Debounce
        if pin.is_low() {
            let _ = sender.send(ButtonEvent::Pressed(id)).await;
            pin.wait_for_high().await;
            let _ = sender.send(ButtonEvent::Released(id)).await;
        }
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Press a button");

    let channel = CHANNEL.put(Channel::new());

    let btn1 = Input::new(p.P1_13.degrade(), Pull::Up);
    let btn2 = Input::new(p.P1_15.degrade(), Pull::Up);
    let btn3 = Input::new(p.P1_09.degrade(), Pull::Up);
    let btn4 = Input::new(p.P0_12.degrade(), Pull::Up);

    let nav_up = Input::new(p.P0_02.degrade(), Pull::Up);
    let nav_l = Input::new(p.P0_10.degrade(), Pull::Up);
    let nav_dn = Input::new(p.P0_29.degrade(), Pull::Up);
    let nav_r = Input::new(p.P0_09.degrade(), Pull::Up);
    let nav_c = Input::new(p.P1_00.degrade(), Pull::Up);

    unwrap!(spawner.spawn(button_task(channel.sender(), Button::A, btn1)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::B, btn2)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::C, btn3)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::D, btn4)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::Up, nav_up)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::Left, nav_l)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::Down, nav_dn)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::Right, nav_r)));
    unwrap!(spawner.spawn(button_task(channel.sender(), Button::Center, nav_c)));

    loop {
        match channel.receiver().recv().await {
            ButtonEvent::Pressed(id) => {
                info!("Btn {} pressed", id);
            }
            ButtonEvent::Released(id) => {
                info!("Btn {} released", id);
            }
        }
    }
}

#[derive(Clone, Copy, Format)]
enum ButtonEvent {
    Pressed(Button),
    Released(Button),
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
