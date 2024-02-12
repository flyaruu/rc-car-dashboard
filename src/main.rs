#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(const_option)]

extern crate alloc;


use core::mem::MaybeUninit;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{Publisher, PubSubChannel, Subscriber};

use embassy_time::Timer;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

use esp_backtrace as _;
use esp_hal_common::Rtc;
use esp_println::println;
use esp_wifi::esp_now::{EspNow, EspNowReceiver};
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay,   embassy::{self}, IO, gdma::Gdma, spi::master::Spi, gpio::NO_PIN, dma::DmaPriority};
use hal::spi::master::prelude::*;

use esp_wifi::{initialize, EspWifiInitFor};

use hal::{timer::TimerGroup, Rng};
use log::{error, info};
use protocol::{Message, TelemetryMessage};
use static_cell::make_static;
use t_display_s3_amoled::rm67162::Orientation;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::graphics::graphics_task;
mod graphics;
mod types;
mod gauge;
mod dashboard;
mod status_screen;

pub type MessageChannel = PubSubChannel<NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;
pub type MessageSubscriber = Subscriber<'static, NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;
pub type MessagePublisher = Publisher<'static, NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;

pub const MAX_PUBS: usize = 10;
pub const MAX_SUBS: usize = 10;
pub const MAX_MESSAGES: usize = 10;



pub const BUFFER_WIDTH: usize = 300;
pub const BUFFER_HEIGHT: usize = 180;
pub const BUFFER_SIZE: usize = embedded_graphics::framebuffer::buffer_size::<Rgb565>(BUFFER_WIDTH, BUFFER_HEIGHT);

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();



fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}


#[main]
async fn main(spawner: Spawner) {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let rtc = make_static!(Rtc::new(peripherals.LPWR));
    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio38.into_push_pull_output();
    info!("Setting led high");
    led.set_high().unwrap();

    //===================

    println!("GPIO init OK");


    println!("init display");



    println!("Hello world!");
    let wifi_timer_group = TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    println!("Hello world2");
    let init = initialize(
        EspWifiInitFor::Wifi,
        wifi_timer_group,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    ).unwrap();



    
    let sclk = io.pins.gpio47;
    let rst = io.pins.gpio17;
    let cs = io.pins.gpio6;

    let d0 = io.pins.gpio18;
    let d1 = io.pins.gpio7;
    let d2 = io.pins.gpio48;
    let d3 = io.pins.gpio5;

    let mut cs = cs.into_push_pull_output();
    cs.set_high().unwrap();

    let mut rst = rst.into_push_pull_output();

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    // Descriptors should be sized as (BUFFERSIZE / 4092) * 3
    let descriptors = [0u32; 12];
    let descriptors = make_static!(descriptors);
    let spi = Spi::new_half_duplex(
        peripherals.SPI2, // use spi2 host
        // NO_PIN,       // Some(cs), NOTE: manually control cs
        75_u32.MHz(), // max 75MHz
        hal::spi::SpiMode::Mode0,
        &clocks)
        .with_pins(Some(sclk),Some(d0),Some(d1),Some(d2),Some(d3),NO_PIN)
        .with_dma(dma_channel.configure(false, descriptors, &mut [], DmaPriority::Priority0));

    
    info!("SPI init");
    let mut display = RM67162Dma::new(spi, cs);
    info!("display created");

    display.reset(&mut rst, &mut delay).unwrap();
    info!("display reset");
    display.init(&mut delay).unwrap();
    info!("display initialized");
    display
        .set_orientation(Orientation::Landscape)
        .unwrap();
    info!("display oriented");
    info!("display clearedx");



    info!("Timers created");
    // let executor: &mut Executor = make_static!(Executor::new());
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group_0);

    // embassy::init(&clocks, timer_group_0);

    display.clear(Rgb565::BLACK).unwrap();

    let wifi = peripherals.WIFI;
    info!("Starting espnow");
    let esp_now = EspNow::new(&init, wifi).unwrap();
    let (_esp_manager, _esp_sender, esp_receiver) = esp_now.split();
    let command_channel: &MessageChannel = make_static!(PubSubChannel::new());
    spawner.spawn(graphics_task(display,command_channel.subscriber().unwrap(),rtc)).unwrap();
    spawner.spawn(receiver(esp_receiver,command_channel.publisher().unwrap())).unwrap();
    // spawner.spawn(test_speedo_telemetry(command_channel.publisher().unwrap())).unwrap();
    // spawner.spawn(test_speedo_odo(command_channel.publisher().unwrap())).unwrap();

}

#[embassy_executor::task]
async fn test_speedo_odo(publisher: MessagePublisher) {
    let mut i = 0;
    loop {
        publisher.publish(Message::Telemetry(TelemetryMessage::Odo(i))).await;
        Timer::after_millis(50).await;
        i+=1;
    }
}

#[embassy_executor::task]
async fn test_speedo_telemetry(publisher: MessagePublisher) {
    loop {
        for i in 0..24 {
            publisher.publish(Message::Telemetry(TelemetryMessage::Rpm(i*10))).await;
            Timer::after_millis(50).await;
        }
        for i in (0..24).rev() {
            publisher.publish(Message::Telemetry(TelemetryMessage::Rpm(i*10))).await;
            Timer::after_millis(50).await;
        }

    }
}
#[embassy_executor::task]
async fn receiver(mut esp_receiver: EspNowReceiver<'static>, publisher: MessagePublisher)->! {
    info!("Starting receiver...");
    loop {
        let msg = esp_receiver.receive_async().await;

        let _sender = msg.info.src_address;
        let msg = Message::from_slice(&msg.data);
        match msg {
            Ok(msg) => {
                publisher.publish(msg).await;
            },
            Err(e) => {
                error!("Problem: {:?}",e);
            },
        }


    }
}
