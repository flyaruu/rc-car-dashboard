use core::f32::consts::PI;

use embedded_graphics::{draw_target::DrawTarget, framebuffer::{self, Framebuffer}, geometry::{Dimensions, Point}, mono_font::{ascii::{FONT_10X20, FONT_8X13}, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor}, primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle}};
use esp_hal_common::gpio::{Output, PushPull};
use esp_println::println;
use num_traits::ToPrimitive;
use num_traits::Float;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::gauge::Gauge;

pub const OUTER_OFFSET: f32 = 10.0;
pub const P_OFFSET: f32 = 20.0;
pub const L_OFFSET: f32 = 40.0;
pub const N_OFFSET: f32 = 70.0;


pub const I_OUTER_OFFSET: u32 = 10;
pub const I_P_OFFSET: u32 = 20;
pub const I_L_OFFSET: u32 = 40;
pub const I_N_OFFSET: u32 = 70;


struct Dashboard<const GAUGE_WIDTH: usize, const GAUGE_HEIGHT: usize, const GAUGE_FRAMEBUFFER_SIZE: usize, const GAUGE_CLEAR_RADIUS: usize> {
    // pub struct Gauge<const W: usize, const H: usize, const BUFFER: usize, const CLEAR_RADIUS: usize>  {
    left_gauge: Gauge<GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS>,
    right_gauge: Gauge<GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS>,
    framebuffer: Framebuffer<Rgb565,RawU16,BigEndian,GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE>,
    mid_buffer: Framebuffer<Rgb565, RawU16, BigEndian, 80, 80, { embedded_graphics::framebuffer::buffer_size::<Rgb565>(80, 80) }>,

}

impl <const GAUGE_WIDTH: usize, const GAUGE_HEIGHT: usize, const GAUGE_FRAMEBUFFER_SIZE: usize, const GAUGE_CLEAR_RADIUS: usize>Dashboard<GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS> {
    pub fn new()->Self {
        Self {
            left_gauge: Gauge::new_speedo(Point::new(20, 10),["0".into(),"20".into(),"40".into(),"60".into(),"80".into(),"100".into(),"120".into(),"140".into(),"160".into(),"180".into(),"200".into(),"220".into(),"240".into()], "000".into(), "KM/H".into(),),
            right_gauge: Gauge::new_speedo(Point::new(536-GAUGE_WIDTH.to_i32().unwrap(), 10),["0".into(),"1".into(),"2".into(),"3".into(),"4".into(),"5".into(),"6".into(),"7".into(),"8".into(),"9".into(),"10".into(),"11".into(),"12".into()] , "".into(), "000000".into(),),
            framebuffer: Framebuffer::new(),
            mid_buffer: Framebuffer::new(),
        }
    }

    pub fn draw_static(&mut self, dashboard_context: &DashboardContext<GAUGE_WIDTH,GAUGE_HEIGHT>) {
        self.framebuffer.clear(Rgb565::BLACK).unwrap();
        self.left_gauge.draw_static(&mut self.framebuffer, &dashboard_context);
        self.right_gauge.draw_static(&mut self.framebuffer, &dashboard_context);
    
    }

    pub fn redraw(&mut self, display: &mut RM67162Dma<'static,hal::gpio::AnyPin<Output<PushPull>>>, dashboard_context: &DashboardContext<GAUGE_WIDTH,GAUGE_HEIGHT>) {
        self.left_gauge.draw_clear_mask(&mut self.framebuffer, dashboard_context);
        self.left_gauge.draw_dynamic(&mut self.framebuffer, dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(self.framebuffer.data(), self.left_gauge.bounding_box).unwrap();
        }

        self.right_gauge.draw_clear_mask(&mut self.framebuffer, &dashboard_context);
        self.right_gauge.draw_dynamic(&mut self.framebuffer, &dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(self.framebuffer.data(), self.right_gauge.bounding_box).unwrap();
        }

    }

}


pub struct DashboardContext<'a, const GAUGE_WIDTH: usize,const GAUGE_HEIGHT: usize> {
    pub outer: [Point; 360],
    pub p_point: [Point; 360],
    pub l_point: [Point; 360],
    pub n_point: [Point; 360],
    pub centre: Point,
    pub back_color: Rgb565,
    gauge_color: Rgb565,
    purple: Rgb565,
    needle_color: Rgb565,
    pub outer_style: PrimitiveStyle<Rgb565>,
    pub inner_style: PrimitiveStyle<Rgb565>,
    pub redline_style: PrimitiveStyle<Rgb565>,
    pub tick_style: PrimitiveStyle<Rgb565>,
    pub red_tick_style: PrimitiveStyle<Rgb565>,
    pub needle_style: PrimitiveStyle<Rgb565>,
    pub text_style: MonoTextStyle<'a,Rgb565>,
    pub red_text_style: MonoTextStyle<'a, Rgb565>,
    pub centre_text_style: MonoTextStyle<'a, Rgb565>,
    pub clearing_circle_bounds: Rectangle,
//     let gauge_color = Rgb565::from(RawU16::from(0x055D));
//     let purple = Rgb565::from(RawU16::from(0xEA16));
//     let needle_color = Rgb565::from(RawU16::from(0xF811));
}

impl <'a, const GAUGE_WIDTH: usize,const GAUGE_HEIGHT: usize> DashboardContext<'a,GAUGE_WIDTH,GAUGE_HEIGHT> {
    pub fn new()->Self {
        let r: f32 = (GAUGE_WIDTH as i32 / 2).to_f32().unwrap();
        let cx = (GAUGE_WIDTH / 2) as i32;
        let cy = (GAUGE_HEIGHT / 2) as i32;
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

        let mut context: DashboardContext<GAUGE_WIDTH, GAUGE_HEIGHT> = DashboardContext { 
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
            clearing_circle_bounds,
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

    pub fn clearing_boundaries(&self)->Rectangle {
        self.clearing_circle_bounds
    }
}

