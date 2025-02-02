# Light Rail

## Overview
A game with trains! TODO: more

Inspirations:
- trains
- Lego trains (also see my [M5Cardputer firmware](https://github.com/nonik0/CardputerLegoTrainControl) for controlling Lego trains)
- my nephew who also loves trains
- [Mini Metro](https://dinopoloclub.com/games/mini-metro/), a game with trains
- PCB Convention Badges, especially [Supercon 2022 badge](https://github.com/Hack-a-Day/2022-Supercon6-Badge-Tool)
- [TIGER Electronics](https://en.wikipedia.org/wiki/Tiger_Electronics) handheld LCD games
- Adafruit LED backpacks, especially [this one](https://www.adafruit.com/product/2946)

### Hardware
The hardware is designed in KiCad 8.0 and all design files are included in the [hardware/kicad8.0](hardware/kicad8.0) directory.

The main hardware components are:
- [ATMega32u4](https://www.microchip.com/en-us/product/atmega32u4) 8-bit MCU
- [IS31FL3731](https://www.lumissil.com/applications/industrial/appliance/major-appliances/range-hood/is31fl3731) matrix LED driver (charliplexing)
- [AS1115](https://ams-osram.com/products/drivers/led-drivers/ams-as1115-led-driver-ic) seven-segment LED display driver
- [KCSC02-105](https://www.kingbright.com/attachments/file/psearch/000/00/00/KCSC02-105(Ver.12A).pdf) seven-segment LED display
- [MIC5219](https://ww1.microchip.com/downloads/en/DeviceDoc/MIC5219-500mA-Peak-Output-LDO-Regulator-DS20006021A.pdf) 3.3V LDS Regulator
- Yellow "track" and red "platform" LEDs, 144 total

See the Bill of Material (BoM) for complete list of components used.

TODO: more hardware overview (using 4 layer pcb, design choices, etc.)

TODO: design comments/flaws/learnings for first prototype

### Firmware

The firmware initially started as a C++/PlatformIO project, which can be found in the [firmware/cpp](firmware/cpp) directory. Before I actually received the inital prototype boards I decided to pivot and write everything in Rust, which can be found in the [firmware/rust](firmware/rust) directory. As part of the effort to get back to where I left off with the C++ firmware, I did the following:
- Wrote a new [AS1115 driver](https://github.com/nonik0/as1115) (TODO: finish and polish design before publishing to crates.io)
- Wrote new tone library for avr-hal (TODO: integrate into avr-hal and open PR)
- Updated existing fork of [IS31FL3731](https://github.com/nonik0/is31fl3731) driver to latest embedded-hal with other minor additions

TODO: basic overview of game design and firmware components

## Design Images

<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/hardware/images/schematic.png" />
</p>
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/hardware/images/pcb_layout.png" width="48%" />
</p>
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/hardware/images/render_front.png" width="48%" />
  <img src="https://github.com/nonik0/Light-Rail/blob/main/hardware/images/render_back.png" width="48%" />
</p>

## Work Left
- pictures/videos of prototype boards in action
- finish basic game components in Rust
- first working game prototype
- further game ideas and testing

## Changelog
* 0.1.0:
     * [ ] TODO
