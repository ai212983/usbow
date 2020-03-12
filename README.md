# USB Over Wireless with nRF52840

Work in progress USB Over Wireless with nRF52840-DK.


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


## Known problems

- `nrf52840-pac` version 0.9 breaks compilation

[JLinkGDBServer]:https://www.segger.com/products/debug-probes/j-link/tools/j-link-gdb-server/about-j-link-gdb-server/