#![no_main]
#![no_std]

use cortex_m::{iprintln, peripheral::ITM};
use rtfm::app;

use nrf52840_pac as pac;
use nrf52840_hal::{
    spim::Spim,
    gpio::{Pin, Output, PushPull},
};

#[allow(unused_imports)]
use panic_itm;

// see https://git.io/Jv6gj on how to work with SPIM

#[app(device = nrf52840_pac, peripherals = true)]
const APP: () = {

    struct Resources {
        itm: ITM
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        init::LateResources {
            itm: cx.core.ITM
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