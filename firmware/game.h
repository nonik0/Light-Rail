#pragma once

#include <Arduino.h>
#include <Adafruit_IS31FL3731.h>

#include "common.h"
#include "platform.h"
#include "train.h"
#include "track.h"

Adafruit_IS31FL3731 _trackAndPlatformLeds = Adafruit_IS31FL3731(); // TODO: write own optimized code for IS31

// possible game modes (maybe consider inheritance from Game)
enum GameMode
{
  ANIMATION,
  FREEPLAY,
  RACE,
  SURVIVAL,
  PUZZLE
};

class Game
{
public:
  Game() {};

  void setup();
  void tick();
  bool isOver();
  void restart();

private:
  const Track *_trackGraph = reinterpret_cast<const Track *>(TrackData16x9);

  bool _isOver = true;
  //Train _trains[2];
  int _mode;
  int _score;
  int _lives;
};

void Game::setup()
{
  _mode = ANIMATION;

  if (!_trackAndPlatformLeds.begin())
  {
    log("IS31 not found");
    while (1)
      ;
  }
  log("IS31 found!");
}

void Game::tick()
{
  // update game state
  // update display

  //_trackAndPlatformLeds.setLEDPWM
}

bool Game::isOver()
{
  return _isOver;
}

void Game::restart()
{
  // reset game state
  _score = 0;
  _lives = 3;

  //_train = Train(_trackGraph, _trackAndPlatformLeds.setLEDPWM);

  _isOver = false;
}