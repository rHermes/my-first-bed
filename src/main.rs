// #![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

// USB STUFF
// use stm32f7xx_hal::rcc::{HSEClock, HSEClockMode};
use stm32f7xx_hal::{delay::Delay, interrupt, pac, prelude::*};

// use usb_device::prelude::*;

use core::sync::atomic::AtomicBool;

static TGL: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    hprintln!("Hello, world!").unwrap();

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC;
    // let syscfg = dp.SYSCFG;
    // let exti = dp.EXTI;

    // let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    // let gpioc = dp.GPIOC.split();
    let gpioe = dp.GPIOE.split();

    let mut cols = (
        gpiob.pb11.into_push_pull_output(),
        gpiob.pb10.into_push_pull_output(),
        gpioe.pe15.into_push_pull_output(),
        gpioe.pe14.into_push_pull_output(),
    );

    let mut rows = (
        gpioe.pe12.into_open_drain_output(),
        gpioe.pe10.into_open_drain_output(),
        gpioe.pe7.into_open_drain_output(),
        gpioe.pe8.into_open_drain_output(),
    );

    rows.0.set_high().expect("GPIO can never fail");
    rows.1.set_high().expect("GPIO can never fail");
    rows.2.set_high().expect("GPIO can never fail");
    rows.3.set_high().expect("GPIO can never fail");

    // let mut button = gpioc.pc13.into_floating_input();
    // button.make_interrupt_source(&mut syscfg, &mut rcc);
    // button.trigger_on_edge(&mut exti, Edge::RISING);
    // button.enable_interrupt(&mut exti);

    // let mut led_green = gpiob.pb0.into_push_pull_output();
    // let mut led_blue = gpiob.pb7.into_push_pull_output();
    // let mut led_red = gpiob.pb14.into_push_pull_output();

    // let mut wow = gpioa.pa0.into_open_drain_output();

    let clocks = rcc.constrain().cfgr.sysclk(216.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    // let mut last_tgl = TGL.load(Ordering::Relaxed);

    // let mut spacing = 1000_u16;
    let mut spacing = 250_u16;

    loop {
        cols.0.set_high().expect("GPIO CAN NEVER FAIL");

        rows.0.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.0.set_high().expect("GPIO cannot fail");
        rows.1.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.1.set_high().expect("GPIO cannot fail");
        rows.2.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.2.set_high().expect("GPIO cannot fail");
        rows.3.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.3.set_high().expect("GPIO cannot fail");

        cols.0.set_low().expect("GPIO CAN NEVER FAIL");
        cols.1.set_high().expect("GPIO CAN NEVER FAIL");

        rows.0.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.0.set_high().expect("GPIO cannot fail");
        rows.1.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.1.set_high().expect("GPIO cannot fail");
        rows.2.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.2.set_high().expect("GPIO cannot fail");
        rows.3.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.3.set_high().expect("GPIO cannot fail");

        cols.1.set_low().expect("GPIO CAN NEVER FAIL");
        cols.2.set_high().expect("GPIO CAN NEVER FAIL");

        rows.0.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.0.set_high().expect("GPIO cannot fail");
        rows.1.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.1.set_high().expect("GPIO cannot fail");
        rows.2.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.2.set_high().expect("GPIO cannot fail");
        rows.3.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.3.set_high().expect("GPIO cannot fail");

        cols.2.set_low().expect("GPIO CAN NEVER FAIL");
        cols.3.set_high().expect("GPIO CAN NEVER FAIL");

        rows.0.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.0.set_high().expect("GPIO cannot fail");
        rows.1.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.1.set_high().expect("GPIO cannot fail");
        rows.2.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.2.set_high().expect("GPIO cannot fail");
        rows.3.set_low().expect("GPIO cannot fail");
        delay.delay_ms(spacing);
        rows.3.set_high().expect("GPIO cannot fail");

        cols.3.set_low().expect("GPIO CAN NEVER FAIL");

        spacing = match spacing {
            400 => 250,
            250 => 150,
            150 => 75,
            75 => 40,
            40 => 20,
            20 => 10,
            10 => 9,
            9 => 8,
            8 => 5,
            5 => 1,
            _ => spacing,
        };
        // if spacing > 5 {
        //     spacing = spacing / 2;
        // }
    }

    // loop {
    //     // let tgl = TGL.load(Ordering::Relaxed);
    //     // if last_tgl != tgl {
    //     //     if tgl {
    //     //         led_red.set_high().expect("GPIO can never fail");
    //     //         wow.set_low().expect("GPIO can never fail");
    //     //     } else {
    //     //         led_red.set_low().expect("GPIO can never fail");
    //     //         wow.set_high().expect("GPIO can never fail");
    //     //     }
    //     // }
    //     // last_tgl = tgl;
    //     // delay.delay_ms(100_u16);
    //     // led_red.set_high().expect("GPIO can never fail");
    //     // wow.set_low().expect("GPIO can never fail");
    //     // delay.delay_ms(3000_u16);

    //     // led_red.set_low().expect("GPIO can never fail");
    //     // wow.set_high().expect("GPIO can never fail");
    //     // delay.delay_ms(3000_u16);
    // }
}

#[interrupt]
fn EXTI15_10() {}
