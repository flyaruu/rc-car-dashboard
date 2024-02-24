use embedded_graphics::{framebuffer::Framebuffer, geometry::{Point, Size}, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565}, primitives::{CornerRadii, Line, Rectangle, RoundedRectangle, StyledDrawable, Triangle}};

use crate::dashboard::DashboardContext;

pub enum LightIndicator {
    On,
    Off,
    High,
}
pub struct StatusScreen<const W: usize, const H: usize, const BUFFER: usize, const GW: usize, const GH: usize> {
    left_blinker: LightIndicator,
    right_blinker: LightIndicator,
    headlight_indicator: LightIndicator,
}

impl <const W: usize, const H: usize, const BUFFER: usize, const GW: usize, const GH: usize>StatusScreen<W,H,BUFFER,GW,GH> {

    pub fn new()->Self {
        Self { left_blinker: LightIndicator::Off, right_blinker: LightIndicator::Off, headlight_indicator: LightIndicator::Off }        
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<GW,GH>) {
        self.draw_headlight_indicator(framebuffer, context, Point { x: 20, y: 50 });
        self.draw_indicators(framebuffer, context);
    }

    pub fn set_left_blinker(&mut self, value: LightIndicator) {
        self.left_blinker = value;
    }
    
    pub fn set_right_blinker(&mut self, value: LightIndicator) {
        self.right_blinker = value;
    }

    pub fn set_headlight_indicator(&mut self, value: LightIndicator) {
        self.headlight_indicator = value;
    }

    fn draw_indicators(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<GW,GH>) {
        let left_style = match self.left_blinker {
            LightIndicator::On => context.blinker_on_style,
            LightIndicator::Off => context.light_off_style,
            LightIndicator::High => context.light_off_style,
        };
        let right_style = match self.right_blinker {
            LightIndicator::On => context.blinker_on_style,
            LightIndicator::Off => context.blinker_off_style,
            LightIndicator::High => context.light_off_style,
        };
        Triangle::new(Point { x: 1, y: 14 }, Point { x: 20, y: 0 }, Point { x: 20, y: 28 })
            .draw_styled(&left_style, framebuffer)
            .unwrap();
        Rectangle::new(Point { x: 20, y: 8 }, Size { width: 16, height: 14 })
            .draw_styled(&left_style, framebuffer)
            .unwrap();

        Triangle::new(Point { x: 80-1, y: 14 }, Point { x: 80-20, y: 0 }, Point { x: 80-20, y: 28 })
            .draw_styled(&right_style, framebuffer)
            .unwrap();
        Rectangle::new(Point { x: 80-36, y: 8 }, Size { width: 16, height: 14 })
            .draw_styled(&right_style, framebuffer)
            .unwrap();
    }

    fn draw_headlight_indicator(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<GW,GH>, top_left: Point) {
        const HEIGHT: u32 = 20;
        let color = match self.headlight_indicator {
            LightIndicator::On => context.headlight_on_style,
            LightIndicator::High => context.headlight_high_style,
            LightIndicator::Off => context.light_off_style,
        };
        // -150, -80
        RoundedRectangle::new(Rectangle { top_left: Point { x: 2, y: 2 } + top_left, size: {Size { width: 16, height: HEIGHT }} }, 
            CornerRadii { top_left: Size { width: 7, height: 7 }, top_right: Size { width: 0, height: 0 }, bottom_right: Size { width: 0, height: 0 }, bottom_left:Size { width: 7, height: 7 } } )
            .draw_styled(&color, framebuffer)
            .unwrap();
        Line::new(Point { x: 21, y: 2 }+ top_left, Point { x: 27, y: 4 }+ top_left)
            .draw_styled(&color, framebuffer)
            .unwrap();
        Line::new(Point { x: 21, y: 7 }+ top_left, Point { x: 27, y: 9 }+ top_left)
            .draw_styled(&color, framebuffer)
            .unwrap();
        Line::new(Point { x: 21, y: 12 }+ top_left, Point { x: 27, y: 14 }+ top_left)
            .draw_styled(&color, framebuffer)
            .unwrap();
        Line::new(Point { x: 21, y: 17 }+ top_left, Point { x: 27, y: 19 }+ top_left)
            .draw_styled(&color, framebuffer)
            .unwrap();
    }
}

