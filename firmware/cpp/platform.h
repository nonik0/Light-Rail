#pragma once

#include <Arduino.h>
#include "track.h"

const uint8_t Platforms[] = {
    0x01, 0x05, 0x06, 0x0F, 0x13, 0x15, 0x1A, 0x1C, 0x23,
    0x28, 0x2C, 0x31, 0x3A, 0x3C, 0x40, 0x41, 0x43, 0x4B,
    0x56, 0x58, 0x6C, 0x71, 0x75, 0x79, 0x7C, 0x7E, 0x86};
const uint8_t PlatformCount = sizeof(Platforms) / sizeof(Platforms[0]);

class Platform
{
public:
    using SetLedCallback = void (*)(uint8_t, uint8_t);
    void setTrack(const Track *track, uint8_t platformLoc, uint8_t trackLoc, SetLedCallback setLed);
    uint8_t platform() { return _platform; }
    uint8_t track() { return _track; }
    bool hasCargo() { return _isOccupied; }
    void loadCargo() { _isOccupied = false; }
    void tick();
    Platform() {}

private:
    void (*_setLed)(uint8_t, uint8_t);
    uint8_t _platform; // platform location
    uint8_t _track;    // adjacent track
    bool _isOccupied;
};

void Platform::tick()
{
    if (!_isOccupied && random(0, 800) == 0)
    {
        _isOccupied = true;
        _setLed(_platform, 16);
    }
}

void Platform::setTrack(const Track *track, uint8_t platformLoc, uint8_t trackLoc, SetLedCallback setLed)
{
    _platform = platformLoc;
    _track = trackLoc;
    _setLed = setLed;
    _isOccupied = false;
}