#![no_main]
#![no_std]

use cortex_m::{iprintln, peripheral::ITM};
use rtfm::app;
use nb::block;

use nrf52840_pac as pac;
#[allow(unused_imports)]
use nrf52840_hal::{
    prelude::*,
    spim::{Spim, Pins, MODE_0, MODE_2, Frequency as SpimFreq},
    gpio::{self as gpio, Pin, Output, PushPull, Level},
};

use nrf52840_dk_bsp::{
    hal::{
        prelude::*,
        timer::{self, Timer},
    },
};

#[allow(unused_imports)]
use panic_itm;

const R_REVISION: u8 = 0x90;
const R_HRSL: u8 = 0xf8;
const R_0: u8 = 0xf8;

/* PINCTL Bits  */
const bmFDUPSPI: u8 = 0x10;
const bmINTLEVEL: u8 = 0x08;
const bmPOSINT: u8 = 0x04;
const bmGPXB: u8 = 0x02;
const bmGPXA: u8 = 0x01;

const rUSBCTL: u8 =    0x78;    //15<<3
/* USBCTL Bits  */
const bmCHIPRES: u8 =  0x20;    //b5

// Working with SPI is based on https://git.io/Jv6gj

#[app(device = nrf52840_pac, peripherals = true)]
const APP: () = {
    struct Resources {
        itm: ITM,
        spim: Spim<pac::SPIM2>,
        cs: Pin<Output<PushPull>>,
        leds: Leds,
        timer: Timer<pac::TIMER0>
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {

        //  MAX3421E use the SPI bus (through the ICSP header)
        let pins0 = gpio::p0::Parts::new(cx.device.P0);
        let pins1 = gpio::p1::Parts::new(cx.device.P1);

// MAX3421E initialization sample code:
// https://github.com/electricimp/reference/blob/master/hardware/max3421e/max3421e.device.nut#L752

        // sample timer: https://github.com/nrf-rs/nrf52-hal/blob/master/boards/nRF52840-DK/examples/blinky.rs
        init::LateResources {
            itm: cx.core.ITM,
            spim: Spim::new(
                cx.device.SPIM2,
                Pins {
                    sck: pins1.p1_15.into_push_pull_output(Level::Low).degrade(),
                    mosi: Some(pins1.p1_13.into_push_pull_output(Level::Low).degrade()),
                    miso: Some(pins1.p1_14.into_floating_input().degrade()),
                },
                SpimFreq::K125,
                MODE_0,
                0),
            cs: pins1.p1_12.into_push_pull_output(Level::High).degrade(),
            // RES - D7 - 1_08
            // GPX - D8 - 1_10
            // INT - D9 - 1_11
            leds: Leds {
                led_1: Led::new(pins0.p0_13.degrade()),
                led_2: Led::new(pins0.p0_14.degrade()),
                led_3: Led::new(pins0.p0_15.degrade()),
                led_4: Led::new(pins0.p0_16.degrade()),
            },
            timer: Timer::new(cx.device.TIMER0)
        }
    }

    #[idle(resources = [itm,spim,cs,leds,timer])]
    fn idle(cx: idle::Context) -> ! {
        let itm = &mut cx.resources.itm.stim[0];

        iprintln!(itm, "Initializing...");

        let reset_data = [(rUSBCTL + 2), bmCHIPRES];
        let reset_slice = &reset_data;
        let mut reset_vec = *reset_slice;

        cx.resources.leds.led_4.disable();

        let reset_res = cx.resources.spim.transfer(cx.resources.cs, &mut reset_vec);
        iprintln!(itm, "Resetting: {}, {:?}", reset_vec[0], reset_res);

        let reference_data = &[R_REVISION; 1];
        let mut test_vec1 = *reference_data;

        let res = cx.resources.spim.transfer(cx.resources.cs, &mut test_vec1);
        iprintln!(itm, "Revision: {}, {:?}", test_vec1[0], res);

        let hrsl_data = &[R_0; 1];
        let mut test_vec2 = *hrsl_data;
        let res2 = cx.resources.spim.transfer(cx.resources.cs, &mut test_vec2);
        iprintln!(itm, "Revision: {}, {:?}", test_vec2[0], res2);

        let mut x = 0;
        loop {
            if x < 3 {
                iprintln!(itm, " Running {}", x);
                x = x + 1;
            }
            delay(cx.resources.timer, 100_000); // 100ms
            cx.resources.leds.led_3.enable();
            delay(cx.resources.timer, 100_000);
            cx.resources.leds.led_3.disable();
        }
    }
};

/// The LEDs on the nRF52840-DK board
pub struct Leds {
    /// nRF52840-DK: LED1, nRF52: P0.30
    pub led_1: Led,

    /// nRF52840-DK: LED2, nRF52: P0.31
    pub led_2: Led,

    /// nRF52840-DK: LED3, nRF52: P0.22
    pub led_3: Led,

    /// nRF52840-DK: LED4, nRF52: P0.14
    pub led_4: Led,
}

/// An LED on the nRF52840-DK board
pub struct Led(Pin<Output<PushPull>>);

impl Led {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Led(pin.into_push_pull_output(Level::High))
    }

    /// Enable the LED
    pub fn enable(&mut self) {
        self.0.set_low().ok();
    }

    /// Disable the LED
    pub fn disable(&mut self) {
        self.0.set_high().ok();
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32) where T: timer::Instance {
    timer.start(cycles);
    let _ = block!(timer.wait());
}