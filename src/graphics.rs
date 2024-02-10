


use alloc::format;
use embassy_sync::pubsub::subscriber;
use embassy_time::{Delay, Timer};
use embedded_graphics::{draw_target::DrawTarget, framebuffer::Framebuffer, geometry::{Point, Size}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor}, primitives::Rectangle};
use esp_hal_common::{delay, Rtc};
use esp_println::print;
use hal::gpio::{Gpio6, Output, PushPull};

use log::info;
use num_traits::ToPrimitive;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::{dashboard::DashboardContext, gauge::Gauge, MessageSubscriber};



#[embassy_executor::task]
pub async fn graphics_task(
    mut display: RM67162Dma<'static,Gpio6<Output<PushPull>>>, 
    mut subscriber: MessageSubscriber,
    rtc: &'static Rtc<'static>,
) {
    const GAUGE_SIZE: usize = 234;
    const GAUGE_CLEAR_SIZE: usize = 160; //GAUGE_SIZE - I_L_OFFSET.to_usize().unwrap();
    let dashboard_context = DashboardContext::new();

    let mut speedo: Gauge<
        GAUGE_SIZE, GAUGE_SIZE,  { embedded_graphics::framebuffer::buffer_size::<Rgb565>(GAUGE_SIZE, GAUGE_SIZE) },
        GAUGE_CLEAR_SIZE> = Gauge::new_speedo(Point::new(20, 10),["0".into(),"20".into(),"40".into(),"60".into(),"80".into(),"100".into(),"120".into(),"140".into(),"160".into(),"180".into(),"200".into(),"220".into(),"240".into()]
            , "000".into(), "KM/H".into(),
    );
    let mut speedo2: Gauge<
        GAUGE_SIZE, GAUGE_SIZE,  { embedded_graphics::framebuffer::buffer_size::<Rgb565>(GAUGE_SIZE, GAUGE_SIZE) },
        GAUGE_CLEAR_SIZE> = Gauge::new_speedo(Point::new(536-GAUGE_SIZE.to_i32().unwrap(), 10),["0".into(),"1".into(),"2".into(),"3".into(),"4".into(),"5".into(),"6".into(),"7".into(),"8".into(),"9".into(),"10".into(),"11".into(),"12".into()]
        , "".into(), "000000".into(),);
    let mut framebuffer = Framebuffer::new();
    framebuffer.clear(Rgb565::BLACK).unwrap();
    // let mut clear_framebuffer = Framebuffer::new();
    speedo.draw_static(&mut framebuffer, &dashboard_context);
    speedo2.draw_static(&mut framebuffer, &dashboard_context);
    Timer::after_millis(500).await;
    // speedo.draw_clear_mask(&mut clear_framebuffer, &dashboard_context);
    // let mid_buffer: Framebuffer<Rgb565, RawU16, BigEndian, 80, 80, { embedded_graphics::framebuffer::buffer_size::<Rgb565>(80, 80) }> = Framebuffer::new();
    // delay.delay_ms(2000_u32);
    loop {
        if subscriber.available() > 0 {
            let message = subscriber.next_message_pure().await;
            match message {
                protocol::Message::Telemetry(telemetry) => match telemetry {
                    protocol::TelemetryMessage::MotorSetting(_) => {},
                    protocol::TelemetryMessage::MotorRpm(rpm) => {
                        speedo2.value = rpm.to_i32().unwrap();
    
                    },
                    protocol::TelemetryMessage::MotorOdo(_) => {},
                    protocol::TelemetryMessage::Rpm(rpm) => {
                        speedo.value = rpm.to_i32().unwrap();
                    },
                    protocol::TelemetryMessage::Odo(odo) => {
                        speedo2.set_line2(format!("{:06}",odo.to_i32().unwrap()).as_str().into()    )
    
                    },
                    _ => {},
                },
                _ => {}
            }
    
        }
        // let now = rtc.get_time_us();
        Timer::after_millis(1).await;
        speedo.draw_clear_mask(&mut framebuffer, &dashboard_context);
        speedo.draw_dynamic(&mut framebuffer, &dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(framebuffer.data(), speedo.bounding_box).unwrap();
        }

        speedo2.draw_clear_mask(&mut framebuffer, &dashboard_context);
        speedo2.draw_dynamic(&mut framebuffer, &dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(framebuffer.data(), speedo2.bounding_box).unwrap();
        }
        // let drawn = rtc.get_time_us();
        // let flushed = rtc.get_time_us();
        print!(".");
        // info!("Frame timings flush: {} draw: {} ",flushed-drawn, drawn-now);
    }
}
