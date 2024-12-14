#pragma once

#include <Arduino.h>

#define TRACK_COUNT 144
#define TRACK_NONE 0xFF

struct Track
{
  uint8_t aConn;  // neighbor on anode side
  uint8_t cConn;  // neighbor on cathode side
  uint8_t aConn2; // neighbor2 on anode side
  uint8_t cConn2; // neighbor2 on cathode side
};

// TODO: platform data, potentially just use special values in track data
// e.g. platform: 0xEEEEEEXX where XX is the track connection/neighbor
struct Platform
{
  uint8_t ledIndex;
  uint8_t trackConn;
};

// // track data for Light-Rail PCB
// uint32_t TrackData[TRACK_COUNT] = {
//   0x3538FFFF, // track 38 / 0x26
//   0x36260000, // track 53 / 0x35
//   0x35370000, // track 54 / 0x36
//   0x3F36002E, // track 55 / 0x37
// };

// track on 16x9 grid
const uint32_t TrackData16x9[TRACK_COUNT] = {
    0x0110FFFF, // track 0/0x00
    0x0200FFFF, // track 1/0x01
    0x0301FFFF, // track 2/0x02
    0x0402FFFF, // track 3/0x03
    0x0503FFFF, // track 4/0x04
    0x0604FFFF, // track 5/0x05
    0x0705FFFF, // track 6/0x06
    0x0806FFFF, // track 7/0x07
    0x0907FFFF, // track 8/0x08+
    0x0A08FFFF, // track 9/0x09
    0x0B09FFFF, // track 10/0x0A
    0x0C0AFFFF, // track 11/0x0B
    0x0D0BFFFF, // track 12/0x0C
    0x0E0CFFFF, // track 13/0x0D
    0x0F0DFFFF, // track 14/0x0E
    0x1F0EFFFF, // track 15/0x0F
    0x0020FFFF, // track 16/0x10
    0xFFFFFFFF, // track 17/0x11
    0xFFFFFFFF, // track 18/0x12
    0xFFFFFFFF, // track 19/0x13
    0xFFFFFFFF, // track 20/0x14
    0xFFFFFFFF, // track 21/0x15
    0xFFFFFFFF, // track 22/0x16
    0xFFFFFFFF, // track 23/0x17
    0xFFFFFFFF, // track 24/0x18
    0xFFFFFFFF, // track 25/0x19
    0xFFFFFFFF, // track 26/0x1A
    0xFFFFFFFF, // track 27/0x1B
    0xFFFFFFFF, // track 28/0x1C
    0xFFFFFFFF, // track 29/0x1D
    0xFFFFFFFF, // track 30/0x1E
    0x2F0FFFFF, // track 31/0x1F
    0x1030FFFF, // track 32/0x20
    0xFFFFFFFF, // track 33/0x21
    0xFFFFFFFF, // track 34/0x22
    0xFFFFFFFF, // track 35/0x23
    0xFFFFFFFF, // track 36/0x24
    0xFFFFFFFF, // track 37/0x25
    0xFFFFFFFF, // track 38/0x26
    0xFFFFFFFF, // track 39/0x27
    0xFFFFFFFF, // track 40/0x28
    0xFFFFFFFF, // track 41/0x29
    0xFFFFFFFF, // track 42/0x2A
    0xFFFFFFFF, // track 43/0x2B
    0xFFFFFFFF, // track 44/0x2C
    0xFFFFFFFF, // track 45/0x2D
    0xFFFFFFFF, // track 46/0x2E
    0x3F1FFFFF, // track 47/0x2F
    0x2040FFFF, // track 58/0x30
    0xFFFFFFFF, // track 59/0x31
    0xFFFFFFFF, // track 60/0x32
    0xFFFFFFFF, // track 61/0x33
    0xFFFFFFFF, // track 62/0x34
    0xFFFFFFFF, // track 63/0x35
    0x4F2FFFFF, // track 63/0x3F
    0x3050FFFF, // track 64/0x40
    0xFFFFFFFF, // track 65/0x41
    0xFFFFFFFF, // track 66/0x42
    0xFFFFFFFF, // track 67/0x43
    0xFFFFFFFF, // track 68/0x44
    0xFFFFFFFF, // track 69/0x45
    0xFFFFFFFF, // track 70/0x46
    0xFFFFFFFF, // track 71/0x47
    0xFFFFFFFF, // track 72/0x48
    0xFFFFFFFF, // track 73/0x49
    0xFFFFFFFF, // track 74/0x4A
    0xFFFFFFFF, // track 75/0x4B
    0xFFFFFFFF, // track 76/0x4C
    0xFFFFFFFF, // track 77/0x4D
    0xFFFFFFFF, // track 78/0x4E
    0x5F3FFFFF, // track 79/0x4F
    0x4060FFFF, // track 80/0x50
    0xFFFFFFFF, // track 81/0x51
    0xFFFFFFFF, // track 82/0x52
    0xFFFFFFFF, // track 83/0x53
    0xFFFFFFFF, // track 84/0x54
    0xFFFFFFFF, // track 85/0x55
    0xFFFFFFFF, // track 86/0x56
    0xFFFFFFFF, // track 87/0x57
    0xFFFFFFFF, // track 88/0x58
    0xFFFFFFFF, // track 89/0x59
    0xFFFFFFFF, // track 90/0x5A
    0xFFFFFFFF, // track 91/0x5B
    0xFFFFFFFF, // track 92/0x5C
    0xFFFFFFFF, // track 93/0x5D
    0xFFFFFFFF, // track 94/0x5E
    0x6F4FFFFF, // track 95/0x5F
    0x5070FFFF, // track 96/0x60
    0xFFFFFFFF, // track 97/0x61
    0xFFFFFFFF, // track 98/0x62
    0xFFFFFFFF, // track 99/0x63
    0xFFFFFFFF, // track 100/0x64
    0xFFFFFFFF, // track 101/0x65
    0xFFFFFFFF, // track 102/0x66
    0xFFFFFFFF, // track 103/0x67
    0xFFFFFFFF, // track 104/0x68
    0xFFFFFFFF, // track 105/0x69
    0xFFFFFFFF, // track 106/0x6A
    0xFFFFFFFF, // track 107/0x6B
    0xFFFFFFFF, // track 108/0x6C
    0xFFFFFFFF, // track 109/0x6D
    0xFFFFFFFF, // track 110/0x6E
    0x7F5FFFFF, // track 111/0x6F
    0x6080FFFF, // track 112/0x70
    0xFFFFFFFF, // track 113/0x71
    0xFFFFFFFF, // track 114/0x72
    0xFFFFFFFF, // track 115/0x73
    0xFFFFFFFF, // track 116/0x74
    0xFFFFFFFF, // track 117/0x75
    0xFFFFFFFF, // track 118/0x76
    0xFFFFFFFF, // track 119/0x77
    0xFFFFFFFF, // track 120/0x78
    0xFFFFFFFF, // track 121/0x79
    0xFFFFFFFF, // track 122/0x7A
    0xFFFFFFFF, // track 123/0x7B
    0xFFFFFFFF, // track 124/0x7C
    0xFFFFFFFF, // track 125/0x7D
    0xFFFFFFFF, // track 126/0x7E
    0x8F6FFFFF, // track 127/0x7F
    0x7090FFFF, // track 128/0x80
    0x7081FFFF, // track 128/0x80
    0x8082FFFF, // track 129/0x81
    0x8183FFFF, // track 130/0x82
    0x8284FFFF, // track 131/0x83
    0x8385FFFF, // track 132/0x84
    0x8486FFFF, // track 133/0x85
    0x8587FFFF, // track 134/0x86
    0x8688FFFF, // track 135/0x87
    0x8789FFFF, // track 136/0x88
    0x888AFFFF, // track 137/0x89
    0x898BFFFF, // track 138/0x8A
    0x8A8CFFFF, // track 139/0x8B
    0x8B8DFFFF, // track 140/0x8C
    0x8C8EFFFF, // track 141/0x8D
    0x8D8FFFFF, // track 142/0x8E
    0x8E7FFFFF, // track 143/0x8F
};
