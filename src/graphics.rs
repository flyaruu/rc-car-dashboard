

use embassy_sync::pubsub::subscriber;
use embedded_graphics::{draw_target::DrawTarget, framebuffer::Framebuffer, geometry::Point, pixelcolor::{Rgb565, RgbColor}};
use esp_hal_common::Rtc;
use hal::gpio::{Gpio6, Output, PushPull};

use log::info;
use num_traits::ToPrimitive;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::{gauge::{DashboardContext, Gauge}, MessageSubscriber};



#[embassy_executor::task]
pub async fn graphics_task(
    mut display: RM67162Dma<'static,Gpio6<Output<PushPull>>>, 
    mut subscriber: MessageSubscriber,
    rtc: &'static Rtc<'static>,
) {


    info!("Creating framebuffer");

    // let mut fb: DashboardFrameBuffer = Framebuffer::new();
    // info!("FB created");
    // fb.clear(Rgb565::CSS_LIGHT_GRAY).unwrap();
    // info!("FB cleared");

    // Timer::after_millis(500).await;

    // let style = PrimitiveStyleBuilder::new()
    // .fill_color(Rgb565::RED)
    // .stroke_color(Rgb565::GREEN)
    // .stroke_width(6)
    // .build();

    // Circle::new(Point::new(200, 200), 20).draw_styled(&style, &mut display).unwrap();
    // let mut fb = Framebuffer::<
    // Rgb565,
    // _,
    // BigEndian,
    // 100,
    // 100,
    // { embedded_graphics::framebuffer::buffer_size::<Rgb565>(100, 100) },
    // >::new();
    // fb.clear(Rgb565::WHITE).unwrap();

    const GAUGE_SIZE: usize =234;

    let dashboard_context = DashboardContext::new();

    let mut speedo: Gauge<GAUGE_SIZE, GAUGE_SIZE,  { embedded_graphics::framebuffer::buffer_size::<Rgb565>(234, 234) }> = Gauge::new_speedo(Point::new(0, 10));
    let mut speedo2: Gauge<GAUGE_SIZE, GAUGE_SIZE,  { embedded_graphics::framebuffer::buffer_size::<Rgb565>(234, 234) }> = Gauge::new_speedo(Point::new(536-234, 10));
    let mut framebuffer = Framebuffer::new();
    loop {
        let message = subscriber.next_message_pure().await;
        match message {
            protocol::Message::Telemetry(telemetry) => match telemetry {
                protocol::TelemetryMessage::MotorSetting(_) => todo!(),
                protocol::TelemetryMessage::MotorRpm(rpm) => todo!(),
                protocol::TelemetryMessage::MotorOdo(_) => todo!(),
                protocol::TelemetryMessage::Rpm(rpm) => {
                    speedo.speed = rpm.to_i32().unwrap();
                    speedo2.speed = rpm.to_i32().unwrap();
                },
                protocol::TelemetryMessage::Odo(_) => todo!(),
                _ => {},
            },
            _ => {}
        }
        let now = rtc.get_time_us();
        framebuffer.clear(Rgb565::BLACK).unwrap();
        speedo.draw(&mut framebuffer, &dashboard_context);
        framebuffer.clear(Rgb565::BLACK).unwrap();
        speedo2.draw(&mut framebuffer,&dashboard_context);
        let drawn = rtc.get_time_us();
        unsafe {
            display.framebuffer_for_viewport(framebuffer.data(), speedo.bounding_box).unwrap();
            display.framebuffer_for_viewport(framebuffer.data(), speedo2.bounding_box).unwrap();
        }
        let flushed = rtc.get_time_us();
        info!("Frame timings flush: {} draw: {} ",flushed-drawn, drawn-now);
    }
}
