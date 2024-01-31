use core::f32::consts::PI;

use alloc::format;
use embedded_graphics::{framebuffer::Framebuffer, geometry::{Angle, Point, Size}, mono_font::{ascii::{FONT_10X20, FONT_8X13}, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor, WebColors}, primitives::{Arc, Line, PrimitiveStyleBuilder, Rectangle, StyledDrawable}, text::{renderer::CharacterStyle, Alignment, Text}, Drawable};
use esp_println::println;
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


pub struct DashboardContext<const W: usize,const H: usize> {
    outer: [Point; 360],
    p_point: [Point; 360],
    l_point: [Point; 360],
    n_point: [Point; 360],

}

impl <const W: usize,const H: usize> DashboardContext<W,H> {
    pub fn new()->Self {
        let mut context: DashboardContext<W, H> = DashboardContext { 
            outer: [Point{ x: 0, y: 0 }; 360],
            p_point: [Point{ x: 0, y: 0 }; 360],
            l_point:  [Point{ x: 0, y: 0 }; 360],
             n_point:  [Point{ x: 0, y: 0 }; 360]
         };
        let r: f32 = (W as i32 / 2).to_f32().unwrap();
        let cx = (W / 2) as i32;
        let cy = (H / 2) as i32;
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
    pub framebuffer: Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>
}

impl <const W: usize,const H: usize,const BUFFER: usize> Gauge<W,H,BUFFER> {

    const CX: i32 = (W / 2) as i32;
    const CY: i32 = (H / 2) as i32;
    // const BACKCOLOR: Rgb565 = Rgb565::from(RawU16::from(0x0026));
    pub fn new_speedo(location: Point)->Self {
        let size = Size::new(W as u32, H as u32);
        let framebuffer = Framebuffer::new();
        Gauge {
            bounding_box: Rectangle::new(location, size),
            framebuffer,
        }
    }

    pub fn draw(&mut self, context: &DashboardContext<W,H>) {
        // self.framebuffer.

        let back_color = Rgb565::from(RawU16::from(0x0026));
        let gauge_color = Rgb565::from(RawU16::from(0x055D));
        let purple = Rgb565::from(RawU16::from(0xEA16));

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

        let text_style = MonoTextStyleBuilder::new()
            .text_color(Rgb565::WHITE)
            .font(&FONT_8X13)
            .build();
        let red_text_style = MonoTextStyleBuilder::new()
            .text_color(purple)
            .font(&FONT_8X13)
            .build();

        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_OUTER_OFFSET, Angle::from_degrees(120.0), Angle::from_degrees(300.0))
            .draw_styled(&outer_style, &mut self.framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_P_OFFSET, Angle::from_degrees(120.0), Angle::from_degrees(300.0))
            .draw_styled(&inner_style, &mut self.framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_L_OFFSET, Angle::from_degrees(0.0), Angle::from_degrees(60.0))
            .draw_styled(&redline_style, &mut self.framebuffer)
            .unwrap();
        Arc::with_center(Point { x: Self::CX, y: Self::CY }, W as u32-I_N_OFFSET, Angle::from_degrees(100.0), Angle::from_degrees(340.0))
            .draw_styled(&outer_style, &mut self.framebuffer)
            .unwrap();
            
        // sprite.drawSmoothArc(cx, cy, r, ir, 30, 330, gaugeColor, backColor);
        // sprite.drawSmoothArc(cx, cy, r - 5, r - 6, 30, 330, TFT_WHITE, backColor);
        // sprite.drawSmoothArc(cx, cy, r - 9, r - 8, 270, 330, purple, backColor);
        // sprite.drawSmoothArc(cx, cy, r - 38, ir - 37, 10, 350, gaugeColor, backColor);

        for i in 0..26 {
            let (tick,current_text_style) = if i<20 {
                (tick_style,text_style)
            } else {
                (red_tick_style,red_text_style)
            };
            if i % 2 == 0 {
                // Text::with_alignment(
                //     &"test",
                //     Point::new(50, 50),
                //     MonoTextStyleBuilder::new()
                //         .background_color(Rgb565::CSS_GRAY)
                //         .text_color(Rgb565::CSS_BISQUE)
                //         .font(&FONT_10X20)
                //         .build(),
                //     Alignment::Center,
                // ).draw(&mut self.framebuffer);

                Line::new(context.outer[i*12], context.p_point[i*12])
                    .draw_styled(&tick, &mut self.framebuffer).unwrap();
                let text = format!("{}",i*10);
                Text::with_alignment(&text, context.l_point[i*12], current_text_style, embedded_graphics::text::Alignment::Center)
                    .draw(&mut self.framebuffer).unwrap();
            } else {
                Line::new(context.outer[i*12], context.p_point[i*12])
                    .draw_styled(&tick, &mut self.framebuffer)
                    .unwrap();

            }            
        } 
        // for (int i = 0; i < 26; i++) {
        //     if (i < 20) {
        //       color1 = gaugeColor;
        //       color2 = TFT_WHITE;
        //     } else {
        //       color1 = purple;
        //       color2 = purple;
        //     }
        
        //     if (i % 2 == 0) {
        //       sprite.drawWedgeLine(x[i * 12], y[i * 12], px[i * 12], py[i * 12], 2, 1, color1);
        //       sprite.setTextColor(color2, backColor);
        //       sprite.drawString(String(i * 10), lx[i * 12], ly[i * 12]);
        //     } else
        //       sprite.drawWedgeLine(x[i * 12], y[i * 12], px[i * 12], py[i * 12], 1, 1, color2);
        //   }
        

        // tach
        // sprite.drawSmoothArc(320 - cx, cy, r, ir, 30, 330, gaugeColor, backColor);
        // sprite.drawSmoothArc(320 - cx, cy, r - 5, r - 6, 30, 330, TFT_WHITE, backColor);
        // sprite.drawSmoothArc(320 - cx, cy, r - 38, ir - 37, 10, 350, gaugeColor, backColor);
      
    }
}

