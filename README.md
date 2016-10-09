# Native Rust on the FRDM-KL27Z Board

Following [the Copper book](https://japaric.github.io/copper/), but converting from STM32F3 (Cortex-M3) to KL27Z (Cortex-M0+)

## Debugging

### Start GDB Server

`JLinkGDBServer -if SWD -device MKL27Z64xxx4`

TODO: Document getting JLinkGDBServer installed and setup:

* Install JLink driver for SDA connection to development board
* ? Maybe some stuff from my gdbrc?

### Attach to GDB Server

`arm-none-eabi-gdb target/cortex-m0plus/debug/frdm-app`

