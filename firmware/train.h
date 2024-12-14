#pragma once

#include <Arduino.h>
#include "track.h"

using namespace std;

class Train
{
public:
  static const uint8_t MaxCars = 5;
  static const uint8_t MinSpeed = 0;
  static const uint8_t MaxSpeed = 100;
  static const uint8_t DefaultSpeed = 5;

  struct Car
  {
    uint8_t TrackLoc;
    uint8_t Contents; // TBD
  };

  Train(const Track* track, void (*setLed)(uint8_t, uint8_t));
  bool advance();
  bool addCar();

private:
  void (*_setLed)(uint8_t, uint8_t);
  const Track* _track;

  Car _cars[MaxCars];
  uint8_t _capacity;
  uint8_t _speed;
  uint8_t _speedCounter;
};

Train::Train(const Track* track, void (*setLed)(uint8_t, uint8_t))
{
  _setLed = setLed;
  _track = track;

  _capacity = 0;
  _speed = DefaultSpeed;
  _speedCounter = 0;

}

bool Train::advance()
{
  _speedCounter += _speed;

  if (_speedCounter < MaxSpeed) {
    return false;
  }

  _speedCounter -= MaxSpeed;

  // TODO

  return true;
}

bool Train::addCar()
{
  if (_capacity >= MaxCars) {
    return false;
  }

  _cars[_capacity] = {0, 0};

  _capacity++;

  return true;
}