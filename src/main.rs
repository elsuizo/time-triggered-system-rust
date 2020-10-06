#![deny(unsafe_code)]
#![no_std]
#![no_main]

// use embedded_hal::digital::v2::OutputPin;
// use portable::datetime::DateTime;
// use portable::{alarm, datetime};
// use portable::button::{Button, Event};
use panic_halt as _;

mod logger;
use heapless::{String, consts::*};

use embedded_hal::timer::CountDown;
use stm32f1xx_hal::{
    prelude::*,
    pac::{interrupt, Interrupt, Peripherals, TIM2},
    rtc::Rtc,
    serial::{Serial, Config},
    delay::Delay,
    i2c::{BlockingI2c, DutyCycle, Mode},
    time::Hertz,
    timer::{CountDownTimer, Event, Timer},
};
use cortex_m_rt::entry;
use core::default::Default;

const MAX_NUM_TASKS: usize = 7;

struct Task {
    /// the callback function that represents the task logic
    callback: fn(),
    /// nose
    delay: u16,
    /// nose
    period: u16
}

struct Scheduller {
    /// the tick rate of the system
    tick_rate: Hertz,
    /// the tasks of the system
    tasks: Option<&'static mut [Task; MAX_NUM_TASKS]>,
    /// the timer associate to the system
    timer: Timer<TIM2>
}

impl Task {
    pub fn new(callback: fn(), delay: u16, period: u16) -> Self {
        Self{callback, delay, period}
    }
}

// TODO(elsuizo:2020-10-05): hacerlo generico para cualquier timer
impl Scheduller {
    pub fn new(tick_rate: Hertz, timer: Timer<TIM2>) -> Self {
        Self {tick_rate, tasks: None, timer}
    }

    pub fn start(self) {
        self.timer.start_count_down(self.tick_rate);
    }

    pub fn update(&self) {
        for (index, task) in self.tasks.enumerate() {
        }

    }
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut pwr = dp.PWR;
    let mut rcc = dp.RCC.constrain();

    // Prepare the alternate function I/O registers
    let mut afio  = dp.AFIO.constrain(&mut rcc.apb2);
    // get the gpioa port
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(8.mhz())
        .pclk1(8.mhz())
        .freeze(&mut flash.acr);
    // let button0_pin = gpiob.pb10.into_pull_up_input(&mut gpiob.crh);
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
    }
}
