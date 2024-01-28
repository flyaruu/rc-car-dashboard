#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

extern crate alloc;


use core::mem::MaybeUninit;
use embassy_executor::Executor;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{Publisher, PubSubChannel, Subscriber};

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

use esp_backtrace as _;
use esp_println::println;
use esp_wifi::esp_now::{EspNow, EspNowReceiver};
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, embassy, IO, gdma::Gdma, spi::master::Spi, gpio::NO_PIN, dma::DmaPriority};
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

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

pub type MessageChannel = PubSubChannel<NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;
pub type MessageSubscriber = Subscriber<'static, NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;
pub type MessagePublisher = Publisher<'static, NoopRawMutex, Message, MAX_MESSAGES, MAX_SUBS, MAX_PUBS>;

pub const MAX_PUBS: usize = 10;
pub const MAX_SUBS: usize = 10;
pub const MAX_MESSAGES: usize = 10;



fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

const MSG_QUEUE_SIZE: usize = 5;
const SUBSCRIBER_SIZE: usize = 2;
const PUBLISHER_SIZE: usize = 2;


pub const BUFFER_WIDTH: usize = 300;
pub const BUFFER_HEIGHT: usize = 180;
pub const BUFFER_SIZE: usize = embedded_graphics::framebuffer::buffer_size::<Rgb565>(BUFFER_WIDTH, BUFFER_HEIGHT);


#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let wifi_timer_group = TimerGroup::new(peripherals.TIMG1, &clocks);

    let init = initialize(
        EspWifiInitFor::Wifi,
        wifi_timer_group.timer0,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    ).unwrap();



    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio38.into_push_pull_output();
    led.set_high().unwrap();

    //===================

    println!("GPIO init OK");

    println!("init display");

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
        Some(sclk),
        Some(d0),
        Some(d1),
        Some(d2),
        Some(d3),
        NO_PIN,       // Some(cs), NOTE: manually control cs
        75_u32.MHz(), // max 75MHz
        hal::spi::SpiMode::Mode0,
        &clocks,
    )
    .with_dma(dma_channel.configure(false, descriptors, &mut [], DmaPriority::Priority0));

    let mut display = RM67162Dma::new(spi, cs);

    display.reset(&mut rst, &mut delay).unwrap();
    display.init(&mut delay).unwrap();
    display
        .set_orientation(Orientation::Portrait)
        .unwrap();

    display.clear(Rgb565::BLACK).unwrap();
    
    let wifi = peripherals.WIFI;
    let esp_now = EspNow::new(&init, wifi).unwrap();

    let executor = make_static!(Executor::new());

    embassy::init(&clocks, timer_group_0.timer0);
    let (_esp_manager, _esp_sender, esp_receiver) = esp_now.split();
    let command_channel: &MessageChannel = make_static!(PubSubChannel::new());

    executor.run(|spawner| {
        spawner.spawn(graphics_task(display,command_channel.subscriber().unwrap())).unwrap();
        spawner.spawn(receiver(esp_receiver,command_channel.publisher().unwrap())).unwrap();
    })
}

#[embassy_executor::task]
async fn receiver(mut esp_receiver: EspNowReceiver<'static>, publisher: MessagePublisher)->! {
    info!("Starting receiver...");
    loop {
        let msg = esp_receiver.receive_async().await;

        let _sender = msg.info.src_address;
        let msg = Message::from_slice(&msg.data);
        println!("Reeived: {:?}",msg);
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
