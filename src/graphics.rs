
use embedded_graphics::{draw_target::DrawTarget, framebuffer::Framebuffer, geometry::{Angle, Point}, mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor, WebColors}, primitives::{Arc, Circle, PrimitiveStyleBuilder, StyledDrawable}, text::{Alignment, Text}, Drawable};
use esp_println::print;
use hal::gpio::{Gpio6, Output, PushPull};

use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::MessageSubscriber;


#[embassy_executor::task]
pub async fn graphics_task(
    mut display: RM67162Dma<'static,Gpio6<Output<PushPull>>>, 
    mut shape_subscriber: MessageSubscriber,
) {

    let mut fb = Framebuffer::<
    Rgb565,
    _,
    BigEndian,
    100,
    100,
    { embedded_graphics::framebuffer::buffer_size::<Rgb565>(100, 100) },
    >::new();
    fb.clear(Rgb565::WHITE).unwrap();



// let mut dial_fb = Framebuffer::<
//     Rgb565,
//     _,
//     BigEndian,
//     100,
//     100,
//     { embedded_graphics::framebuffer::buffer_size::<Rgb565>(100, 100) },>::new();



    loop {

        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::RED)
            .stroke_color(Rgb565::GREEN)
            .stroke_width(6)
            .build();


        let message = shape_subscriber.next_message_pure().await;
        
        round_dial(&mut fb);

        // display.fill_contiguous(area, colors)
        // unsafe {
            // display.fill_with_framebuffer(fb.data()).unwrap();
        // }
        print!(".");

    }
}

// { embedded_graphics::framebuffer::buffer_size::<Rgb565>(100, 100 ) }
// fn round_dial(fb: &mut Framebuffer<Rgb565,RawU16,BigEndian,536,240,257280>) {

fn round_dial(fb: &mut Framebuffer<Rgb565,RawU16,BigEndian,100,100,20000>) {
    fb.clear(Rgb565::WHITE).unwrap();
    let circle_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_GRAY)
        .stroke_color(Rgb565::GREEN)
        .stroke_width(6)
        .build();
    Circle::new(Point { x: 0, y: 0 }, 100)
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

