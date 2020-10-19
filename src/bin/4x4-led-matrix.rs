#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f7xx_hal::{delay::Delay, gpio, pac, prelude::*};

type Cols = (
    gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>,
    gpio::gpiob::PB10<gpio::Output<gpio::PushPull>>,
    gpio::gpioe::PE15<gpio::Output<gpio::PushPull>>,
    gpio::gpioe::PE14<gpio::Output<gpio::PushPull>>,
);

type Rows = (
    gpio::gpioe::PE12<gpio::Output<gpio::OpenDrain>>,
    gpio::gpioe::PE10<gpio::Output<gpio::OpenDrain>>,
    gpio::gpioe::PE7<gpio::Output<gpio::OpenDrain>>,
    gpio::gpioe::PE8<gpio::Output<gpio::OpenDrain>>,
);

// It's laid out row by row
type Frame = [[bool; 4]; 4];

fn draw_frame(
    cols: &mut Cols,
    rows: &mut Rows,
    frame: &Frame,
    delay: &mut Delay,
    duration: u32,
    min_space: u32,
) {
    // We want the whole frame to to last duration microseconds.
    // each turn on needs a pause. So we need to divide the duration
    // with the numbers of lights we are going to turn on to get
    // the idea. If we turn on zero lights, then we just wait.
    let num_on: u32 = frame.into_iter().flatten().filter(|x| **x).count() as u32;

    // How long a lap takes
    let lap: u32 = min_space * num_on;

    if lap == 0 {
        delay.delay_us(duration);
        return;
    }

    let loops = duration / lap;

    for _ in 0..loops {
        for (i, row) in frame.into_iter().enumerate() {
            // Turn on the gate
            match i {
                0 => rows.0.set_low().expect("GPIO can not fail!"),
                1 => rows.1.set_low().expect("GPIO can not fail!"),
                2 => rows.2.set_low().expect("GPIO can not fail!"),
                3 => rows.3.set_low().expect("GPIO can not fail!"),
                _ => panic!("NEVER SHOULD HAPPEN!"),
            };

            for (j, c) in row.into_iter().enumerate() {
                if !c {
                    continue;
                }

                match j {
                    0 => cols.0.set_high().expect("GPIO can not fail!"),
                    1 => cols.1.set_high().expect("GPIO can not fail!"),
                    2 => cols.2.set_high().expect("GPIO can not fail!"),
                    3 => cols.3.set_high().expect("GPIO can not fail!"),
                    _ => panic!("NEVER SHOULD HAPPEN!"),
                }
                delay.delay_us(min_space);
                match j {
                    0 => cols.0.set_low().expect("GPIO can not fail!"),
                    1 => cols.1.set_low().expect("GPIO can not fail!"),
                    2 => cols.2.set_low().expect("GPIO can not fail!"),
                    3 => cols.3.set_low().expect("GPIO can not fail!"),
                    _ => panic!("NEVER SHOULD HAPPEN!"),
                }
            }

            // Turn of the gate again
            match i {
                0 => rows.0.set_high().expect("GPIO can not fail!"),
                1 => rows.1.set_high().expect("GPIO can not fail!"),
                2 => rows.2.set_high().expect("GPIO can not fail!"),
                3 => rows.3.set_high().expect("GPIO can not fail!"),
                _ => panic!("NEVER SHOULD HAPPEN!"),
            };
        }
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC;

    let gpiob = dp.GPIOB.split();
    let gpioe = dp.GPIOE.split();

    let mut cols: Cols = (
        gpiob.pb11.into_push_pull_output(),
        gpiob.pb10.into_push_pull_output(),
        gpioe.pe15.into_push_pull_output(),
        gpioe.pe14.into_push_pull_output(),
    );

    let mut rows: Rows = (
        gpioe.pe12.into_open_drain_output(),
        gpioe.pe10.into_open_drain_output(),
        gpioe.pe7.into_open_drain_output(),
        gpioe.pe8.into_open_drain_output(),
    );

    // Zero the pins
    cols.0.set_low().expect("GPIO can never fail");
    cols.1.set_low().expect("GPIO can never fail");
    cols.2.set_low().expect("GPIO can never fail");
    cols.3.set_low().expect("GPIO can never fail");

    rows.0.set_high().expect("GPIO can never fail");
    rows.1.set_high().expect("GPIO can never fail");
    rows.2.set_high().expect("GPIO can never fail");
    rows.3.set_high().expect("GPIO can never fail");

    let clocks = rcc.constrain().cfgr.sysclk(216.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let fps: u32 = 10;
    let min_space = 50_u32;

    // How long we have to wait in microseconds
    let frame_space = (1000 * 1000) / fps;

    let frames = pat_spiral_trail();

    loop {
        for frame in &frames {
            draw_frame(
                &mut cols,
                &mut rows,
                frame,
                &mut delay,
                frame_space,
                min_space,
            )
        }
    }
}

fn pat_spiral_trail() -> [Frame; 32] {
    let f = false;
    let t = true;
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        [
            [f, f, f, f],
            [f, f, t, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, f],
            [f, t, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, f],
            [f, t, t, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, f],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [f, f, f, t],
            [f, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [f, f, t, t],
            [f, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [f, t, t, t],
            [f, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [t, t, t, t],
            [f, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [f, t, t, t],
            [f, f, f, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
            [f, f, f, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
            [t, f, f, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
            [t, t, f, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, f],
        ],
        [
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
            [t, t, t, t],
        ],
        // Reverse
        [
            [t, t, t, t],
            [t, t, f, t],
            [t, t, t, t],
            [t, t, t, t],
        ],
        [
            [t, t, t, t],
            [t, f, f, t],
            [t, t, t, t],
            [t, t, t, t],
        ],
        [
            [t, t, t, t],
            [t, f, f, t],
            [t, f, t, t],
            [t, t, t, t],
        ],
        [
            [t, t, t, t],
            [t, f, f, t],
            [t, f, f, t],
            [t, t, t, t],
        ],
        [
            [t, t, t, t],
            [t, f, f, t],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [t, t, t, t],
            [t, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [t, t, t, f],
            [t, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [t, t, f, f],
            [t, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [t, f, f, f],
            [t, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [f, f, f, f],
            [t, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [t, f, f, f],
            [t, t, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [t, t, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, t, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
    ]
}

fn pat_spiral() -> [Frame; 18] {
    let f = false;
    let t = true;
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        [
            [f, f, f, f],
            [f, f, t, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, t, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, t, f, f],
            [f, t, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, t, t, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, t, t],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, t],
            [f, f, f, t],
            [f, f, f, f],
        ],
        [
            [f, f, f, t],
            [f, f, f, t],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, t, t],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, t, t, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [t, t, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [t, f, f, f],
            [t, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [t, f, f, f],
            [t, f, f, f],
            [f, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [t, f, f, f],
            [t, f, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [t, t, f, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, t, t, f],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, t, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, t],
        ],
        [
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
        ],
    ]
}

fn pat_xor() -> [Frame; 2] {
    let f_xor_one: Frame = [
        [false, true, false, true],
        [true, false, true, false],
        [false, true, false, true],
        [true, false, true, false],
    ];

    let f_xor_two: Frame = [
        [true, false, true, false],
        [false, true, false, true],
        [true, false, true, false],
        [false, true, false, true],
    ];

    [f_xor_one, f_xor_two]
}

fn pat_spinner() -> [Frame; 20] {
    let f_frame_01: Frame = [
        [false, false, true, false],
        [false, false, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ];
    let f_frame_02: Frame = [
        [false, false, false, true],
        [false, false, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ];
    let f_frame_03: Frame = [
        [false, false, false, false],
        [false, false, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ];
    let f_frame_04: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, true, true],
        [false, false, false, false],
    ];
    let f_frame_05: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, true, false],
        [false, false, false, true],
    ];
    let f_frame_06: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
    ];
    let f_frame_07: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [false, true, false, false],
        [false, true, false, false],
    ];
    let f_frame_08: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [false, true, false, false],
        [true, false, false, false],
    ];
    let f_frame_09: Frame = [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, false, false],
        [false, false, false, false],
    ];
    let f_frame_10: Frame = [
        [false, false, false, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ];
    let f_frame_11: Frame = [
        [true, false, false, false],
        [false, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ];
    let f_frame_12: Frame = [
        [false, true, false, false],
        [false, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ];

    [
        f_frame_01, f_frame_01, f_frame_02, f_frame_03, f_frame_03, f_frame_04, f_frame_04,
        f_frame_05, f_frame_06, f_frame_06, f_frame_07, f_frame_07, f_frame_08, f_frame_09,
        f_frame_09, f_frame_10, f_frame_10, f_frame_11, f_frame_12, f_frame_12,
    ]
}
