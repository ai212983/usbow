#![no_main]
#![no_std]

use cortex_m::{iprintln, peripheral::ITM};
use rtfm::app;

use nrf52840_pac as pac;
use nrf52840_hal::{
    prelude::*,
    spim::{Spim, Pins, MODE_0, Frequency as SpimFreq},
    gpio::{Pin, Output, PushPull, Level},
};

#[allow(unused_imports)]
use panic_itm;


// Working with SPI is based on https://git.io/Jv6gj


#[app(device = nrf52840_pac, peripherals = true)]
const APP: () = {
    struct Resources {
        itm: ITM,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {

        // Arduino communicates with the MAX3421E using the SPI bus (through the ICSP header).
        // This is on digital pins 10, 11, 12, and 13 on the Uno and pins 10, 50, 51, and 52 on the Mega.
        // On both boards, pin 10 is used to select the MAX3421E. Pins 7, 8 and 9 are used for GPX, INT and RES pins.
        let pins = cx.device.P1.split();
        let cs = pins.p1_11.into_push_pull_output(Level::Low);
        let spim =Spim::new(
            cx.device.SPIM2,
            Pins {
                sck: pins.p1_14.into_push_pull_output(Level::Low).degrade(),
                mosi: Some(pins.p1_13.into_push_pull_output(Level::Low).degrade()),
                miso: Some(pins.p1_12.into_floating_input().degrade()),
            },
            SpimFreq::K500,
            MODE_0,
            0);

        init::LateResources {
            itm: cx.core.ITM,
        }
    }

    #[idle(resources = [itm])]
    fn idle(cx: idle::Context) -> ! {
        let itm = &mut cx.resources.itm.stim[0];
        iprintln!(itm, " Initializing ");

        let mut x = 0;
        loop {
            if x < 10 {
                iprintln!(itm, " Running {}", x);
                x = x + 1;
            }
        }
    }
};