# Fingerprint detection on the STM32 Nucleo 446RE board

A simple program using my [`hzgrow-r502`][hzgrow-r502] library to search for a fingerprint
on a R502 reader connected to USART1. After performing one search, it halts; it outputs its result to the debugger
via semihosting.

## Running the program

This program is written for the STM32F446RE microcontroller on a Nucleo board. Wire accordingly:

* The `RX` pin of the R502 to the `PA9/Serial1_TX` pin of the Nucleo board, marked `D8` on the Arduino-style header
* The `TX` pin of the R502 to the `PA10/Serial1_RX` pin of the Nucleo board, marked `D2` on the Arduino-style header

You will need to have something like OpenOCD installed and running in a console window somewhere:

```powershell
openocd.exe -f ./openocd.cfg
```

Once that's up and running, either use `cargo run` or, if you're using Visual Studio Code, use the
example launch config file with the [Native Debug][webfreak.nativedebug] plugin to launch the debugger functionality
there. If you have a different STM32 microcontroller, please update `memory.x` with its flash and ram size, and
`.cargo/config` with the default architecture to use with `cargo run`.

When you start the program, the blue light on the R502 will come on continuously until you place a finger
on the reader. Then, it will blink either blue or red, depending on whether your fingerprint
was recognised or not. At the same time, in the OpenOCD console window, you will get messages like these:

> Found a match! Index 0 with confidence 56

> No match!

This will also tell you the result of the fingerprint search. Note, if you have a factory-fresh R502,
you will need to enroll some fingerprints, which you can do with one of the examples in [`hzgrow-r502`][hzgrow-r502]
and a USB to UART adapter. Use 3.3V logic and power.

[hzgrow-r502]: https://github.com/FLamparski/hzgrow-r502
[webfreak.nativedebug]: https://marketplace.visualstudio.com/items?itemName=webfreak.debug
