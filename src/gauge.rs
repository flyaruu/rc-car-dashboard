use core::f32::consts::PI;

use alloc::format;
use embedded_graphics::{framebuffer::Framebuffer, geometry::{Angle, Dimensions, Point, Size}, mono_font::{ascii::{FONT_10X20, FONT_8X13}, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor, WebColors}, primitives::{Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable}, text::{renderer::CharacterStyle, Alignment, Text}, Drawable};
use esp_println::println;
use log::info;
use num_traits::ToPrimitive;
use num_traits::Float;

const OUTER_OFFSET: f32 = 10.0;
const P_OFFSET: f32 = 20.0;
const L_OFFSET: f32 = 40.0;
const N_OFFSET: f32 = 50.0;

const I_OUTER_OFFSET: u32 = 10;
const I_P_OFFSET: u32 = 20;
const I_L_OFFSET: u32 = 40;
const I_N_OFFSET: u32 = 115;


pub struct DashboardContext<'a, const W: usize,const H: usize> {
    outer: [Point; 360],
    p_point: [Point; 360],
    l_point: [Point; 360],
    n_point: [Point; 360],
    centre: Point,
    back_color: Rgb565,
    gauge_color: Rgb565,
    purple: Rgb565,
    needle_color: Rgb565,
    outer_style: PrimitiveStyle<Rgb565>,
    inner_style: PrimitiveStyle<Rgb565>,
    redline_style: PrimitiveStyle<Rgb565>,
    tick_style: PrimitiveStyle<Rgb565>,
    red_tick_style: PrimitiveStyle<Rgb565>,
    needle_style: PrimitiveStyle<Rgb565>,
    text_style: MonoTextStyle<'a,Rgb565>,
    red_text_style: MonoTextStyle<'a, Rgb565>,
    centre_text_style: MonoTextStyle<'a, Rgb565>,
//     let gauge_color = Rgb565::from(RawU16::from(0x055D));
//     let purple = Rgb565::from(RawU16::from(0xEA16));
//     let needle_color = Rgb565::from(RawU16::from(0xF811));
}

impl <'a, const W: usize,const H: usize> DashboardContext<'a,W,H> {
    pub fn new()->Self {
        let r: f32 = (W as i32 / 2).to_f32().unwrap();
        let cx = (W / 2) as i32;
        let cy = (H / 2) as i32;
        let centre = Point::new(cx, cy);
        let clearing_circle_bounds = Circle::with_center(centre, (2.0*(r - L_OFFSET)).to_u32().unwrap()).bounding_box();
        let back_color = Rgb565::from(RawU16::from(0x0026));
        let gauge_color = Rgb565::from(RawU16::from(0x055D));
        let purple = Rgb565::from(RawU16::from(0xEA16));
        let needle_color = Rgb565::from(RawU16::from(0xF811));
        let outer_style = PrimitiveStyleBuilder::new()
            .stroke_color(gauge_color)
            .stroke_width(3)
            .build();
        let inner_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::WHITE)
            .stroke_width(3)
            .build();
        let redline_style = PrimitiveStyleBuilder::new()
            .stroke_color(purple)
            .stroke_width(3)
            .build();
        let tick_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::WHITE)
            .stroke_width(2)
            .build();
        let red_tick_style = PrimitiveStyleBuilder::new()
            .stroke_color(purple)
            .stroke_width(2)
            .build();
        let needle_style = PrimitiveStyleBuilder::new()
            .stroke_color(needle_color)
            .stroke_width(4)
            .build();
        let text_style = MonoTextStyleBuilder::new()
            .text_color(Rgb565::WHITE)
            .font(&FONT_8X13)
            .build();
        let red_text_style = MonoTextStyleBuilder::new()
            .text_color(purple)
            .font(&FONT_8X13)
            .build();

        let centre_text_style = MonoTextStyleBuilder::new()
            .text_color(Rgb565::WHITE)
            .font(&FONT_10X20)
            .build();

        let mut context: DashboardContext<W, H> = DashboardContext { 
            outer: [Point{ x: 0, y: 0 }; 360],
            p_point: [Point{ x: 0, y: 0 }; 360],
            l_point:  [Point{ x: 0, y: 0 }; 360],
            n_point:  [Point{ x: 0, y: 0 }; 360],
            centre,
            back_color,
            gauge_color,
            purple,
            needle_color,
            outer_style,
            inner_style,
            redline_style,
            tick_style,
            red_tick_style,
            needle_style,
            text_style,
            red_text_style,
            centre_text_style,
        };
        for i in 0..360 {
            let a = ((i + 120) % 360) as i32;
            let angle_rad = a.to_f32().unwrap() * PI / 180.0;
            println!("i: {} a: {} a_rad: {}",i,a,angle_rad);
            context.outer[i] = Point {
                x: ((r - OUTER_OFFSET) * angle_rad.cos()).to_i32().unwrap() + cx,
                y: ((r - OUTER_OFFSET) * angle_rad.sin()).to_i32().unwrap() + cy,
            };
            context.p_point[i] = Point {
                x: ((r - P_OFFSET) * angle_rad.cos()).to_i32().unwrap() + cx,
                y: ((r - P_OFFSET) * angle_rad.sin()).to_i32().unwrap() + cy,
            };
            context.l_point[i] = Point {
                x: ((r - L_OFFSET) * angle_rad.cos()).to_i32().unwrap() + cx,
                y: ((r - L_OFFSET) * angle_rad.sin()).to_i32().unwrap() + cy,
            };
            context.n_point[i] = Point {
                x: ((r - N_OFFSET) * angle_rad.cos()).to_i32().unwrap() + cx,
                y: ((r - N_OFFSET) * angle_rad.sin()).to_i32().unwrap() + cy,
            };
        }
        context
    }
}

pub struct Gauge<const W: usize, const H: usize, const BUFFER: usize>  {
    pub bounding_box: Rectangle,
    pub speed: i32,
    // pub framebuffer: Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>
}

impl <const W: usize,const H: usize,const BUFFER: usize> Gauge<W,H,BUFFER> {

    const CX: i32 = (W / 2) as i32;
    const CY: i32 = (H / 2) as i32;
    pub fn new_speedo(location: Point)->Self {
        let size = Size::new(W as u32, H as u32);
        // let framebuffer = Framebuffer::new();
        Gauge {
            bounding_box: Rectangle::new(location, size),
            // framebuffer,
            speed: 0,
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
        self.draw_dial(framebuffer, context)
    }

    // &mut self, 
    pub fn draw_dial(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>,  context: &DashboardContext<W,H>) {
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

        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_N_OFFSET, Angle::from_degrees(100.0), Angle::from_degrees(340.0))
            .draw_styled(&context.outer_style, framebuffer)
            .unwrap();



        let speedo_angle: usize = (self.speed.to_f32().unwrap() * 1.2).to_usize().unwrap();
        Line::new(context.outer[speedo_angle], context.n_point[speedo_angle])
            .draw_styled(&context.needle_style, framebuffer)
            .unwrap();

        let speed_text = format!("{}",self.speed);
      
        Text::with_alignment(&speed_text, context.centre, context.centre_text_style, embedded_graphics::text::Alignment::Center)
            .draw(framebuffer).unwrap();
        Text::with_alignment("KM/H", Point::new(context.centre.x, context.centre.y + 18), context.centre_text_style, embedded_graphics::text::Alignment::Center)
            .draw(framebuffer).unwrap();
    }
}

