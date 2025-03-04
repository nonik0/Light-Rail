# Light Rail

## Overview & Acknowledgements

Light Rail is a minimalist, interactive PCB game that simulates trains traveling along tracks, picking up and delivering cargo from platforms. The track is composed of yellow LEDs arranged in lines, which illuminate sequentially to create the effect of trains moving along the tracks. Next to each of the eight forks and crosses in the track is a button to switch its state. The board also has a three-character seven-segment display for showing the game status and/or score and four more buttons underneath the track for gameplay control. Light Rail can be powered with two LIR2032 batteries or via USB.

I also want to acknowledge and thank [PCBWay](https://www.pcbway.com/) for their generous support of this project! Even before I had the initial idea, PCBWay had already reached out to me asking if I would like to collaborate on my next project. However, I didn't actually see the email until I started looking into how I wanted to manufacture prototypes for Light Rail. Just getting the email and seeing that others were liking my older projects gave me a big motivation boost to continue with this project, be creative, and make something of my own. Overall, my experience with PCBWay was great and I would definitely recommend checking them out for your own projects!

For more details about my design goals, process, and ordering and production experience with PCBWay, please see my full [project writeup](WRITEUP.md)!

[![hardware test](https://github.com/nonik0/Light-Rail/blob/main/images/hardware_test.gif)](https://raw.githubusercontent.com/nonik0/Light-Rail/main/images/hardware_test.mp4)

### Work Left
- first working game prototype
- further game ideas and testing
- investigate CR2032 batteries in series

## Hardware
The hardware is designed in KiCad 8.0 and all design files are included in the [hardware/kicad8.0](hardware/kicad8.0) directory.

The main hardware components are:
- [ATMega32u4](https://www.microchip.com/en-us/product/atmega32u4) 8-bit MCU
- [IS31FL3731](https://www.lumissil.com/applications/industrial/appliance/major-appliances/range-hood/is31fl3731) matrix LED driver (charliplexing)
- [AS1115](https://ams-osram.com/products/drivers/led-drivers/ams-as1115-led-driver-ic) seven-segment LED display driver
- [KCSC02-105](https://www.kingbright.com/attachments/file/psearch/000/00/00/KCSC02-105(Ver.12A).pdf) seven-segment LED display
- [MIC5219](https://ww1.microchip.com/downloads/en/DeviceDoc/MIC5219-500mA-Peak-Output-LDO-Regulator-DS20006021A.pdf) 3.3V LDS Regulator
- Yellow "track" and red "platform" LEDs, 144 total

See the [Bill of Material (BoM)](hardware/documents/Light_Rail_BOM.xlsx) for complete list of components used and my [project writeup](WRITEUP.md) for more details.

## Firmware

The firmware initially started as a C++/PlatformIO project, which can be found in the [firmware/cpp](firmware/cpp) directory. Before I actually received the inital prototype boards I decided to pivot and write everything in Rust, which can be found in the [firmware/rust](firmware/rust) directory. As part of the effort to get back to where I left off with the C++ firmware, I did the following:
- Wrote a new [AS1115 driver](https://github.com/nonik0/as1115) (TODO: finish and polish design before publishing to crates.io)
- Wrote new tone library for avr-hal (TODO: integrate into avr-hal and open PR)
- Updated existing fork of [IS31FL3731](https://github.com/nonik0/is31fl3731) driver to latest embedded-hal with other minor additions

TODO: basic overview of game design and firmware components (in second writeup?)

## Images

#### Prototype
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/board_front_on.jpg" width="30%" />
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/board_front.jpg" width="30%" />
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/board_back.jpg" width="30%" />
</p>

#### Schematic
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/schematic.png" />
</p>

#### PCB Layout
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/pcb_layout.png" width="30%" />
</p>

#### Render
<p align="center" width="100%">
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/render_front.png" width="30%" />
  <img src="https://github.com/nonik0/Light-Rail/blob/main/images/render_back.png" width="30%" />
</p>


