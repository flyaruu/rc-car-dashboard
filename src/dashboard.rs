use core::{f32::consts::PI, marker::PhantomData};

use embassy_executor::Spawner;
use embedded_graphics::{draw_target::DrawTarget, framebuffer::Framebuffer, geometry::{Dimensions, Point, Size}, mono_font::{ascii::{FONT_10X20, FONT_8X13}, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565, RgbColor}, primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle}};
use embedded_hal::digital::OutputPin;
use esp_println::println;
use heapless::String;
use num_traits::ToPrimitive;
use num_traits::Float;
use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::{gauge::Gauge, status_screen::{LightIndicator, StatusScreen}};

pub const OUTER_OFFSET: f32 = 10.0;
pub const P_OFFSET: f32 = 20.0;
pub const L_OFFSET: f32 = 40.0;
pub const N_OFFSET: f32 = 70.0;


pub const I_OUTER_OFFSET: u32 = 10;
pub const I_P_OFFSET: u32 = 20;
pub const I_L_OFFSET: u32 = 40;
pub const I_N_OFFSET: u32 = 70;


pub struct Dashboard<'a, const GAUGE_WIDTH: usize, 
    const GAUGE_HEIGHT: usize, 
    const GAUGE_FRAMEBUFFER_SIZE: usize,
    const GAUGE_CLEAR_RADIUS: usize,
    const STATUS_SCREEN_WIDTH: usize,
    const STATUS_SCREEN_HEIGHT: usize,
    const STATS_SCREEN_FRAMEBUFFER_SIZE: usize,
    CsPin: OutputPin,
    > {
    // pub struct Gauge<const W: usize, const H: usize, const BUFFER: usize, const CLEAR_RADIUS: usize>  {
    left_gauge: Gauge<'a, GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS,240>,
    right_gauge: Gauge<'a, GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS,1200>,
    framebuffer: Framebuffer<Rgb565,RawU16,BigEndian,GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE>,
    status_screen: StatusScreen<STATUS_SCREEN_WIDTH,STATUS_SCREEN_HEIGHT, STATS_SCREEN_FRAMEBUFFER_SIZE,GAUGE_WIDTH,GAUGE_HEIGHT>,
    mid_buffer: Framebuffer<Rgb565, RawU16, BigEndian, STATUS_SCREEN_WIDTH, STATUS_SCREEN_HEIGHT, STATS_SCREEN_FRAMEBUFFER_SIZE>,
    _phantom: PhantomData<CsPin>,

}

impl <'a, const GAUGE_WIDTH: usize, 
    const GAUGE_HEIGHT: usize,
    const GAUGE_FRAMEBUFFER_SIZE: usize,
    const GAUGE_CLEAR_RADIUS: usize,
    const STATUS_SCREEN_WIDTH: usize,
    const STATUS_SCREEN_HEIGHT: usize,
    const STATUS_SCREEN_FRAMEBUFFER_SIZE: usize,
    CsPin: OutputPin
    > Dashboard<'a, GAUGE_WIDTH,GAUGE_HEIGHT,GAUGE_FRAMEBUFFER_SIZE,GAUGE_CLEAR_RADIUS,STATUS_SCREEN_WIDTH,STATUS_SCREEN_HEIGHT,STATUS_SCREEN_FRAMEBUFFER_SIZE, CsPin> {
    pub fn new()->Self {
        Self {
            left_gauge: Gauge::new_speedo(Point::new(0, 10),["0","20","40","60","80","100","120","140","160","180","200","220","240"], "000".into(), "KM/H".into(),),
            right_gauge: Gauge::new_speedo(Point::new(536-GAUGE_WIDTH.to_i32().unwrap(), 10),["0","1","2","3","4","5","6","7","8","9","10","11","12"] , "".into(), "000000".into(),),
            framebuffer: Framebuffer::new(),
            status_screen: StatusScreen::new(),
            mid_buffer: Framebuffer::new(),
            _phantom: PhantomData,
        }
    }

    pub fn set_left_line1(&mut self, text: String<6>) {
        self.left_gauge.set_line1(text);
    }

    pub fn set_left_line2(&mut self, text: String<6>) {
        self.left_gauge.set_line2(text);
    }

    pub fn set_right_line1(&mut self, text: String<6>) {
        self.right_gauge.set_line1(text);
    }

    pub fn set_right_line2(&mut self, text: String<6>) {
        self.right_gauge.set_line2(text);
    }

    pub fn set_left_value(&mut self, value: i32) {
        self.left_gauge.set_value(value);
    }

    pub fn set_right_value(&mut self, value: i32) {
        self.right_gauge.set_value(value);
    }

    pub fn draw_static(&mut self, dashboard_context: &DashboardContext<GAUGE_WIDTH,GAUGE_HEIGHT>) {
        self.framebuffer.clear(Rgb565::BLACK).unwrap();
        self.left_gauge.draw_static(&mut self.framebuffer, dashboard_context);
        self.right_gauge.draw_static(&mut self.framebuffer, dashboard_context);
    
    }

    pub fn update_indicated(&mut self) {
        self.left_gauge.update_indicated();
        self.right_gauge.update_indicated();
    }

    pub fn redraw(&mut self, display: &mut RM67162Dma<'static,CsPin>, dashboard_context: &DashboardContext<GAUGE_WIDTH,GAUGE_HEIGHT>) {
        self.left_gauge.draw_clear_mask(&mut self.framebuffer, dashboard_context);
        self.left_gauge.draw_dynamic(&mut self.framebuffer, dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(self.framebuffer.data(), self.left_gauge.bounding_box).unwrap();
        }

        self.right_gauge.draw_clear_mask(&mut self.framebuffer, dashboard_context);
        self.right_gauge.draw_dynamic(&mut self.framebuffer, dashboard_context);
        unsafe {
            display.framebuffer_for_viewport(self.framebuffer.data(), self.right_gauge.bounding_box).unwrap();
        }
        self.status_screen.draw(&mut self.mid_buffer, dashboard_context);

        unsafe {
            display.framebuffer_for_viewport(self.mid_buffer.data(), Rectangle { top_left: Point { x: 228, y: 30 }, size: Size { width: STATUS_SCREEN_WIDTH as u32, height: STATUS_SCREEN_HEIGHT as u32 } }).unwrap();
        }
    }

    pub fn set_left_blinker(&mut self, value: LightIndicator) {
        self.status_screen.set_left_blinker(value);
    }
    
    pub fn set_right_blinker(&mut self, value: LightIndicator) {
        self.status_screen.set_right_blinker(value);
    }

    pub fn set_headlight_indicator(&mut self, value: LightIndicator) {
        self.status_screen.set_headlight_indicator(value);
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
    pub headlight_on_style: PrimitiveStyle<Rgb565>,
    pub indicator_on_style: PrimitiveStyle<Rgb565>,
    pub blinker_on_style: PrimitiveStyle<Rgb565>,
    pub blinker_off_style: PrimitiveStyle<Rgb565>,
    pub headlight_high_style: PrimitiveStyle<Rgb565>,
    pub light_off_style: PrimitiveStyle<Rgb565>,
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
        let headlight_on_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::GREEN)
            .stroke_width(1)
            .stroke_color(Rgb565::GREEN)
            .build();
        let headlight_high_style = PrimitiveStyleBuilder::new()
            .fill_color(gauge_color)
            .stroke_width(1)
            .stroke_color(gauge_color)
            .build();

        let indicator_on_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::GREEN)
            .stroke_width(2)
            .build();
        let blinker_on_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::GREEN)
            .build();
        let blinker_off_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(0x4, 0x8, 0x4))
            .build();
        // let color = Rgb565::new(0x33, 0x33, 0x33);


        let light_off_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(0x4, 0x8, 0x4))
            .stroke_width(1)
            .fill_color(Rgb565::new(0x4, 0x8, 0x4))
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
            headlight_on_style,
            headlight_high_style,
            indicator_on_style,
            blinker_on_style,
            blinker_off_style,
            light_off_style,            
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
}

