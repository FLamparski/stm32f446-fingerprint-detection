#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
#[macro_use]
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use crate::hal::{
    prelude::*,
    stm32,
    serial::{Serial, config::Config},
};
use hzgrow_r502::{R502, Command, Reply, GenImgResult, GenImgStatus, SearchStatus};

const DEFAULT_BAUD_RATE: u32 = 57600;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Set up a serial port.
        // On the Nucleo-446RE board, PA9 (TX) is called D8 on the Arduino header, and PA10 (RX) is called D2.
        // Because making sense is so last century.
        let gpioa = dp.GPIOA.split();
        let tx_pin = gpioa.pa9.into_alternate_af7();
        let rx_pin = gpioa.pa10.into_alternate_af7();
        let serial_config = Config::default().baudrate(DEFAULT_BAUD_RATE.bps());
        let serial = Serial::usart1(
            dp.USART1,
            (tx_pin, rx_pin),
            serial_config,
            clocks
        ).unwrap();

        // Get the R502
        let (tx, rx) = serial.split();
        let mut r502 = R502::new(tx, rx, 0xffffffff);

        // Do some R502 ops
        let search_result: Reply = Ok(())
            .and_then(|_| r502.send_command(Command::VfyPwd { password: 0x00000000 }))
            .and_then(|_| loop {
                match r502.send_command(Command::GenImg) {
                    Ok(Reply::GenImg(GenImgResult { confirmation_code: GenImgStatus::Success, .. })) => return Ok(()),
                    Ok(Reply::GenImg(GenImgResult { confirmation_code: GenImgStatus::FingerNotDetected, .. })) => {},
                    _ => return Err(hzgrow_r502::Error::RecvWrongReplyType),
                }
            })
            .and_then(|_| r502.send_command(Command::Img2Tz { buffer: 1 }))
            .and_then(|_| r502.send_command(Command::Search { buffer: 1, start_index: 0, end_index: 0xffff }))
            .unwrap();
        
        match search_result {
            Reply::Search(result) => {
                match result.confirmation_code {
                    SearchStatus::Success => hprintln!("Found a match! Index {} with confidence {}", result.match_id, result.match_score),
                    SearchStatus::NoMatch => hprintln!("No match!"),
                    _ => hprintln!("something is odd, try again"),
                }.unwrap();
            },
            _ => hprintln!("something is odd, try again").unwrap(),
        };
    }
    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
