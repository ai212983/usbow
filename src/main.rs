#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_itm;

use cortex_m::{iprintln, peripheral::ITM};

use rtfm::app;

use nrf52840_pac as pac;

#[app(device = nrf52840_pac, peripherals = true)]
const APP: () = {};