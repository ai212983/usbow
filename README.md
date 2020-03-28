# USB Over Wireless with nRF52840

Work in progress USB Over Wireless with nRF52840-DK pca10056 with MAX3421E USB Host.


## Debug

[JLinkGDBServer] by Segger is used to debug, launch `jlinkgdb` to start it:

```
./jlinkgdb
```

Then run the program
```
cargo run
```
cargo will use the run definition in `.cargo/config` to launch `gdb` with the `jlink.gdb` script.

Type `continue` (or `c` for short) in gdb input to actually start the program.

### Logging
There are multiple logging solutions can be found on Internet. 
I've been able to set up logging via [RTT] with `jlink_rtt` and [RTT Viewer], but using SWO seems to be 
more widely [used](https://github.com/blueluna/nrf52840-dk-experiments). 

Usually SWO [requires a debug probe](https://mcuoneclipse.com/2016/10/17/tutorial-using-single-wire-output-swo-with-arm-cortex-m-and-eclipse/), 
but J-Link GDB Server supports [SWO via USB]. To see its output, run `telnet` on output port:
```
telnet localhost 2332
```


## Known problems

- `nrf52840-pac` version 0.9 breaks compilation



[JLinkGDBServer]:https://www.segger.com/products/debug-probes/j-link/tools/j-link-gdb-server/about-j-link-gdb-server/
[NRF Logger module]:https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_getting_started%2FUG%2Fgs%2Fdevelop_sw.html&cp=1_0_2
[RTT]:https://www.segger.com/products/debug-probes/j-link/technology/about-real-time-transfer/
[RTT Viewer]:https://www.segger.com/products/debug-probes/j-link/tools/rtt-viewer/
[SWO via USB]:https://wiki.segger.com/How_to_use_SWO_with_GDB
