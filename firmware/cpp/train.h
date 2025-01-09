#pragma once

#include <Arduino.h>
#include "game.h"
#include "track.h"

class Train
{
public:
  using SetLedCallback = void (*)(uint8_t, uint8_t);
  static const uint8_t MaxCars = 5;
  static const uint8_t MinSpeed = 0;
  static const uint8_t MaxSpeed = 100;
  static const uint8_t DefaultSpeed = 10;
  static const uint8_t CarFullBrightness = 200;
  static const uint8_t CarEmptyBrightness = 50;

  struct Car
  {
    uint8_t Location;
    uint8_t Cargo; // TBD
  };

  Train() {}
  void setTrack(const Track *track, SetLedCallback setLed);
  void init(uint8_t location, uint8_t cargo = 0);
  bool advance();
  bool addCar(uint8_t cargo = 0);
  uint8_t front() { return _cars[0].Location; }

private:
  void (*_setLed)(uint8_t, uint8_t);
  const Track *_track;

  bool _engineDirection; // false: exit from anode, true: exit from cathode
  uint8_t _numCars;
  Car _cars[MaxCars];
  uint8_t _speed;
  uint8_t _speedCounter;
};

void Train::setTrack(const Track *track, SetLedCallback setLed) {
  _track = track;
  _setLed = setLed;
}

void Train::init(uint8_t location, uint8_t cargo)
{
  _speed = random(10, 30); // TODO
  _speedCounter = 0;
  _engineDirection = false; // TODO

  _cars[0] = {location, cargo};
  _setLed(location, CarEmptyBrightness);
  _numCars = 1;

  log("Train initialized at " + String(location));
}

bool Train::advance()
{
  // TODO: acceleration: def should factor in number of cars + cargo
  _speedCounter += _speed;

  if (_speedCounter < MaxSpeed)
  {
    return false;
  }

  _speedCounter -= MaxSpeed;

  // move train from the rear, setting each LED accordingly
  _setLed(_cars[_numCars - 1].Location, 0);
  for (uint8_t i = _numCars - 1; i > 0; i--)
  {
    _cars[i].Location = _cars[i - 1].Location;
    _setLed(_cars[i].Location, _cars[i].Cargo > 0 ? CarFullBrightness : CarEmptyBrightness);
  }

  // advance front car to next location, setting LED accordingly
  uint8_t curTrackLoc = _cars[0].Location;
  Track curTrack = _track[curTrackLoc];
  uint8_t nextTrackLoc = _engineDirection ? curTrack.cathodeNextLoc : curTrack.anodeNextLoc;
  uint8_t nextTrackLoc2 = _engineDirection ? curTrack.cathodeNextLoc2 : curTrack.anodeNextLoc2;

  // randomly choose between two possible next tracks for now
  if (nextTrackLoc2 != TRACK_NONE && random(0, 2) == 0)
  {
    nextTrackLoc = nextTrackLoc2;
  }

  _cars[0].Location = nextTrackLoc;
  _setLed(nextTrackLoc, _cars[0].Cargo > 0 ? CarFullBrightness : CarEmptyBrightness);
  _engineDirection = _track[nextTrackLoc].anodeNextLoc == curTrackLoc || _track[nextTrackLoc].anodeNextLoc2 == curTrackLoc; // if (either of) next track's anode connection is current location, exit from next track's cathode connection

  //log("Train advanced to " + String(nextTrackLoc));

  return true;
}

bool Train::addCar(uint8_t cargo)
{
  if (_numCars >= MaxCars)
  {
    return false;
  }

  // add new caboose behind the current caboose
  uint8_t cabooseCarLoc = _cars[_numCars - 1].Location;
  Track cabooseTrack = _track[cabooseCarLoc];
  bool cabooseDirection = _numCars > 1
                              ? cabooseTrack.cathodeNextLoc == _cars[_numCars - 2].Location
                              : _engineDirection; // if caboose is the engine

  // new caboose is in the opposite direction of the current caboose
  uint8_t newCabooseLoc = cabooseDirection ? cabooseTrack.anodeNextLoc : cabooseTrack.cathodeNextLoc;
  _cars[_numCars] = {newCabooseLoc, cargo};
  _setLed(newCabooseLoc, CarEmptyBrightness);
  _numCars++;

  log("Added car at " + String(newCabooseLoc));

  return true;
}