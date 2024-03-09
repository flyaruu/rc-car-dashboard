


use alloc::format;

use embassy_executor::{task, Spawner};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;
use embedded_graphics::pixelcolor::Rgb565;
use esp_hal_common::Rtc;

use hal::gpio::{Gpio6, Output, PushPull};


use log::info;
use num_traits::ToPrimitive;

use t_display_s3_amoled::rm67162::dma::RM67162Dma;

use crate::{dashboard::{Dashboard, DashboardContext}, status_screen::LightIndicator, MessageSubscriber};

pub static VALUE_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();



#[task]
async fn value_updater() {
    loop {
        VALUE_SIGNAL.signal(());
        Timer::after_millis(20).await;
    }
}

#[task]
pub async fn graphics_task(
    mut display: RM67162Dma<'static,Gpio6<Output<PushPull>>>, 
    mut subscriber: MessageSubscriber,
    spawner: Spawner,
    _rtc: &'static Rtc<'static>,
) {
    const GAUGE_SIZE: usize = 220;
    const GAUGE_CLEAR_SIZE: usize = 160; //GAUGE_SIZE - I_L_OFFSET.to_usize().unwrap();
    const STATUS_SCREEN_WIDTH: usize = 80;
    const STATUS_SCREEN_HEIGHT: usize = 160;
    
    spawner.spawn(value_updater()).unwrap();
    info!("Value updater started");
    let dashboard_context = DashboardContext::new();

    let mut dashboard: Dashboard<GAUGE_SIZE, GAUGE_SIZE, { embedded_graphics::framebuffer::buffer_size::<Rgb565>(GAUGE_SIZE, GAUGE_SIZE) }, GAUGE_CLEAR_SIZE, STATUS_SCREEN_WIDTH, STATUS_SCREEN_HEIGHT, { embedded_graphics::framebuffer::buffer_size::<Rgb565>(STATUS_SCREEN_WIDTH, STATUS_SCREEN_HEIGHT) },Gpio6<Output<PushPull>>> = Dashboard::new();    
    dashboard.draw_static(&dashboard_context);
    info!("Starting graphics loop");
    loop {
        dashboard.redraw(&mut display, &dashboard_context);
        if VALUE_SIGNAL.signaled() {
            dashboard.update_indicated();
            VALUE_SIGNAL.reset();
        }
        if subscriber.available() > 0 {
            let message = subscriber.next_message_pure().await;
            match message {
                protocol::Message::Telemetry(telemetry) => match telemetry {
                    protocol::TelemetryMessage::MotorSetting(_) => {},
                    protocol::TelemetryMessage::MotorRpm(rpm) => {
                        dashboard.set_right_value(rpm.to_i32().unwrap());
    
                    },
                    protocol::TelemetryMessage::MotorOdo(odo) => {
                        dashboard.set_right_line2(format!("{:06}",odo.to_i32().unwrap()).as_str().into());
                    },
                    protocol::TelemetryMessage::Rpm(rpm) => {
                        dashboard.set_left_value(rpm.to_i32().unwrap());
                    },
                    protocol::TelemetryMessage::Odo(_) => {
                        // info!("Received odo: {}",odo);
                        // info!("Odo: {}",odo);
                    },
                    protocol::TelemetryMessage::Blink(blink_state) => {
                        match blink_state {
                            protocol::BlinkState::LeftOn => {dashboard.set_left_blinker(LightIndicator::On); dashboard.set_right_blinker(LightIndicator::Off);},
                            protocol::BlinkState::RightOn => {dashboard.set_left_blinker(LightIndicator::Off); dashboard.set_right_blinker(LightIndicator::On);},
                            protocol::BlinkState::AllOff => {dashboard.set_left_blinker(LightIndicator::Off); dashboard.set_right_blinker(LightIndicator::Off);},
                            protocol::BlinkState::Alarm => {dashboard.set_left_blinker(LightIndicator::On); dashboard.set_right_blinker(LightIndicator::On);},
                        }
                    },
                    _ => {},
                },
                protocol::Message::Control(c) => match c {
                    protocol::ControlMessage::HeadlightCommand(cmd) => {
                        match cmd {
                            protocol::Headlights::Low => dashboard.set_headlight_indicator(LightIndicator::On),
                            protocol::Headlights::High => dashboard.set_headlight_indicator(LightIndicator::High),
                            protocol::Headlights::Off => dashboard.set_headlight_indicator(LightIndicator::Off),
                        }                        
                    },
                    protocol::ControlMessage::BlinkerCommand(_) => {},
                    _ => {}

                }
            }
    
        } else {
            // Make sure there is at least _something_ yielding
            Timer::after_millis(1).await;
        }
    }
}


