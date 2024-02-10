use core::{f32::consts::PI, str::FromStr};

use alloc::{format};
use embedded_graphics::{framebuffer::Framebuffer, geometry::{Angle, Dimensions, Point, Size}, mono_font::{ascii::{FONT_10X20, FONT_8X13}, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor}, primitives::{Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable}, text::Text, Drawable};
use esp_println::{print, println};
use heapless::String;
use num_traits::ToPrimitive;
use num_traits::Float;

use crate::dashboard::{DashboardContext, I_L_OFFSET, I_N_OFFSET, I_OUTER_OFFSET, I_P_OFFSET};


pub struct Gauge<const W: usize, const H: usize, const BUFFER: usize, const CLEAR_RADIUS: usize>  {
    pub bounding_box: Rectangle,
    pub value: i32,
    pub texts: [String<3>; 13],
    line1: String<6>,
    line2: String<6>,
    // pub framebuffer: Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>
}

impl <const W: usize,const H: usize,const BUFFER: usize, const CLEAR_RADIUS: usize>
    Gauge<W,H,BUFFER,CLEAR_RADIUS> {

    const CX: i32 = (W / 2) as i32;
    const CY: i32 = (H / 2) as i32;
    pub fn new_speedo(location: Point, texts: [String<3>;13], line1: String<6>, line2: String<6>)->Self {
        let size = Size::new(W as u32, H as u32);
        // let framebuffer = Framebuffer::new();
        Gauge {
            bounding_box: Rectangle::new(location, size),
            // framebuffer,
            value: 0,
            texts,
            line1,
            line2,

        }
    }

    pub fn set_line1(&mut self, value: String<6>) {
        self.line1 = value;
    }

    pub fn set_line2(&mut self, value: String<6>) {
        self.line2 = value;
    }

    // pub fn draw(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
    //     self.draw_dial(framebuffer, context)
    // }

    // &mut self, 
    pub fn draw_static(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
        // self.framebuffer.


        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_OUTER_OFFSET, Angle::from_degrees(120.0), Angle::from_degrees(300.0))
            .draw_styled(&context.outer_style, framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_P_OFFSET, Angle::from_degrees(120.0), Angle::from_degrees(300.0))
            .draw_styled(&context.inner_style, framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_L_OFFSET, Angle::from_degrees(0.0), Angle::from_degrees(60.0))
            .draw_styled(&context.redline_style, framebuffer)
            .unwrap();
        for i in 0..26 {
            let (tick,current_text_style) = if i<20 {
                (context.tick_style,context.text_style)
            } else {
                (context.red_tick_style,context.red_text_style)
            };
            if i % 2 == 0 {
                Line::new(context.outer[i*12], context.p_point[i*12])
                    .draw_styled(&tick, framebuffer).unwrap();
                let text = format!("{}",i*10);
                Text::with_alignment(&text, context.l_point[i*12], current_text_style, embedded_graphics::text::Alignment::Center)
                    .draw(framebuffer).unwrap();
            } else {
                Line::new(context.outer[i*12], context.p_point[i*12])
                    .draw_styled(&tick, framebuffer)
                    .unwrap();

            }            
        }
    }

    pub fn draw_clear_mask(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
        Circle::with_center(Point { x: Self::CX, y: Self::CY }, CLEAR_RADIUS.to_u32().unwrap())
            .draw_styled(&PrimitiveStyleBuilder::new().fill_color(context.back_color).build(), framebuffer)
            .unwrap();
    }


    pub fn draw_dynamic(&mut self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
        // println!("draw dyn");
        // print!(".");
        // Dynamic
        for i in 0..26 {
            let current_text_style = if i<20 {
                context.text_style
            } else {
                context.red_text_style
            };
            if i % 2 == 0 {
                // TODO time this, could store these:
                let text = &self.texts[i>>1];
                Text::with_alignment(&text, context.l_point[i*12], current_text_style, embedded_graphics::text::Alignment::Center)
                    .draw(framebuffer).unwrap();
            }            
        }
        let gauge_angle: usize = (self.value.to_f32().unwrap() * 1.2).to_usize().unwrap() % 360;
        Line::new(context.l_point[gauge_angle], context.n_point[gauge_angle])
            .draw_styled(&context.needle_style, framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, (W as u32-I_N_OFFSET) / 2, Angle::from_degrees(100.0), Angle::from_degrees(340.0))
            .draw_styled(&context.outer_style, framebuffer)
            .unwrap();

        let speed_text = format!("{}",self.value);
        self.set_line1(String::from_str(&speed_text).unwrap());
        Text::with_alignment(&speed_text, context.centre, context.centre_text_style, embedded_graphics::text::Alignment::Center)
            .draw(framebuffer).unwrap();
        Text::with_alignment(&self.line2, Point::new(context.centre.x, context.centre.y + 18), context.centre_text_style, embedded_graphics::text::Alignment::Center)
            .draw(framebuffer).unwrap();
    }
}

