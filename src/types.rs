use embedded_graphics::{framebuffer::Framebuffer, pixelcolor::{raw::{BigEndian, RawU16}, Rgb565}};

pub const SCREEN_WIDTH: usize = 536;
pub const SCREEN_HEIGHT: usize = 150;
pub const BUFFER_SIZE: usize = embedded_graphics::framebuffer::buffer_size::<Rgb565>(SCREEN_WIDTH, SCREEN_HEIGHT);
pub type DashboardFrameBuffer = Framebuffer::<Rgb565,RawU16,BigEndian,SCREEN_WIDTH,SCREEN_HEIGHT,BUFFER_SIZE>;

// #define backColor 0x0026
// #define gaugeColor 0x055D
// #define dataColor 0x0311
// #define purple 0xEA16
// #define needleColor 0xF811