
use embassy_time::Timer;
use embedded_graphics::{draw_target::DrawTarget, framebuffer::Framebuffer, geometry::{Angle, Point}, mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor, WebColors}, primitives::{Arc, Circle, PrimitiveStyleBuilder, StyledDrawable}, text::{Alignment, Text}, Drawable};
use esp_hal_common::Rtc;
use hal::gpio::{Gpio6, Output, PushPull};

use log::info;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::{gauge::{DashboardContext, Gauge}, types::DashboardFrameBuffer, MessageSubscriber};



#[embassy_executor::task]
pub async fn graphics_task(
    mut display: RM67162Dma<'static,Gpio6<Output<PushPull>>>, 
    mut shape_subscriber: MessageSubscriber,
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
    let dashboard_context = DashboardContext::new();

    let mut speedo: Gauge<200, 200, 80000> = Gauge::new_speedo(Point::new(0, 10));
    // let mut speedo2: Gauge<200, 200, 80000> = Gauge::new_speedo(Point::new(267, 10));
    loop {
        let now = rtc.get_time_us();

        speedo.draw(&dashboard_context);
        // speedo2.draw(&dashboard_context);
        let drawn = rtc.get_time_us();
        unsafe {
            display.framebuffer_for_viewport(speedo.framebuffer.data(), speedo.bounding_box).unwrap();
            // display.framebuffer_for_viewport(speedo2.framebuffer.data(), speedo2.bounding_box).unwrap();
        }


        let flushed = rtc.get_time_us();
        info!("Frame timings flush: {} draw: {} ",flushed-drawn, drawn-now);


        // for i in 0..10 {
        //     let now = rtc.get_time_us();            
        //     println!("Clearing: {}",(rtc.get_time_us() - now));
        //     let space = Rectangle::new(Point::new(0, 0), Size::new(100, 100));
        //     let centre = Point::new(50, 50);
        //     round_dial(&mut fb, 10*i, &space, centre);
        //     println!("Dial: {}",(rtc.get_time_us() - now));
        //     unsafe {
        //         display.fill_with_framebuffer(fb.data()).unwrap();
        //     }
        //     led.toggle().unwrap();       
        // }
    }


    // loop {
    //     info!("Starting draw loop");

    //     // Timer::after_millis(1000).await;

        
    //     // let message = shape_subscriber.next_message().await;
    //     let now = rtc.get_time_us();
    //     fb.clear(Rgb565::WHITE).unwrap();
    //     let cleared = rtc.get_time_us();

    //     round_dial(&mut fb);
    //     let drawn = rtc.get_time_us();

    //     unsafe {
    //         display.fill_with_framebuffer(fb.data()).unwrap();
    //     }
    //     let flushed = rtc.get_time_us();
    //     info!("Frame timings flush: {} draw: {} clear: {}",flushed-drawn, drawn-cleared, cleared-now);
    // }
}

// { embedded_graphics::framebuffer::buffer_size::<Rgb565>(100, 100 ) }
// fn round_dial(fb: &mut Framebuffer<Rgb565,RawU16,BigEndian,536,240,257280>) {



fn round_dial(fb: &mut DashboardFrameBuffer) {
    let circle_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_GRAY)
        .stroke_color(Rgb565::GREEN)
        .stroke_width(6)
        .build();
    Circle::new(Point { x: 0, y: 0 }, 100)
        .draw_styled(&circle_style, fb)
        .unwrap();
    Circle::new(Point { x: 10, y: 10 }, 100)
        .draw_styled(&circle_style, fb)
        .unwrap();

        let arc_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(6)
        .build();

    Arc::new(Point::new(50, 50) , 200, Angle::from_degrees(45.0), Angle::from_degrees(270.0))
        .draw_styled(&arc_style, fb)
        .unwrap();
    Text::with_alignment(
        &"test",
        Point::new(50, 50),
        MonoTextStyleBuilder::new()
            .background_color(Rgb565::CSS_GRAY)
            .text_color(Rgb565::CSS_BISQUE)
            .font(&FONT_10X20)
            .build(),
        Alignment::Center,
    ).draw(fb)
    .unwrap();
    
    }

