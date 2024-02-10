use embedded_graphics::{framebuffer::Framebuffer, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565}};

use crate::dashboard::DashboardContext;

enum LightIndicator {
    On,
    Off,
}
pub struct StatusScreen<const W: usize, const H: usize, const BUFFER: usize> {
    left_blinker: LightIndicator,
    right_blinker: LightIndicator,
    headlight_indicator: LightIndicator,
}

impl <const W: usize, const H: usize, const BUFFER: usize>StatusScreen<W,H,BUFFER> {
    pub fn draw(&self, framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<W,H>) {
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

    fn draw_indicators(framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<W,H>) {
        // sprite.fillSmoothCircle(cx, cy, r + 2, backColor);
        // sprite.fillSmoothCircle(320 - cx, cy, r + 2, backColor);
        // sprite.fillTriangle(126, 14, 136, 7, 136, 21, dirColor[leftPointer]);  //dirction pointers
        // sprite.fillRect(136, 11, 8, 7, dirColor[leftPointer]);
        // sprite.fillTriangle(126 + 68, 14, 136 + 48, 7, 136 + 48, 21, dirColor[rightPointer]);
        // sprite.fillRect(176, 11, 8, 7, dirColor[rightPointer]);
    }

    fn draw_headlight_indicator(framebuffer: &mut Framebuffer<Rgb565,RawU16,BigEndian,W,H,BUFFER>, context: &DashboardContext<W,H>) {
          //.....................................drawing LIGHTs
        // sprite.fillSmoothRoundRect(152, 82, 14, 10, 7, lightColor[lights], lightColor[0]);
        // sprite.fillRect(161, 82, 5, 10, lightColor[0]);
        // sprite.drawLine(163, 82, 167, 84 - lights, lightColor[lights]);
        // sprite.drawLine(163, 85, 167, 87 - lights, lightColor[lights]);
        // sprite.drawLine(163, 88, 167, 90 - lights, lightColor[lights]);
        // sprite.drawLine(163, 91, 167, 93 - lights, lightColor[lights]);
    }
}

