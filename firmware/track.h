#pragma once

#include <Arduino.h>


struct Track {
  uint8_t aConn; // neighbor on anode side
  uint8_t cCOnn; // neighbor on cathode side
  uint8_t aConn2; // neighbor2 on anode side
  uint8_t cConn2; // neighbor2 on cathode side
};

uint32_t TrackData[144] = {
  0x3538FFFF, // track 38 / 0x26
  0x36260000, // track 53 / 0x35
  0x35370000, // track 54 / 0x36
  0x3F36002E, // track 55 / 0x37
};



// Train Data:
// 1 byte per car?