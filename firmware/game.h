#pragma once

#include <Arduino.h>
#include <Adafruit_IS31FL3731.h>

#include "common.h"
#include "platform.h"
#include "train.h"
#include "track.h"

#define TRACK_COUNT 144

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
private:
  Adafruit_IS31FL3731 _trackLeds = Adafruit_IS31FL3731(); // TODO: write own optimized code for IS31

  int _mode;
  int _score;
  int _lives;

public:
  Game() {};

  void setup();
  void tick();
  bool isOver();
  void restart();
};

void Game::setup()
{
  _mode = ANIMATION;
  _score = 0;

  if (!_trackLeds.begin())
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
}

bool Game::isOver()
{
  return false;
}

void Game::restart()
{
  // reset game state
}