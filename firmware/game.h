#pragma once

#include <Arduino.h>

#include "platform.h"
#include "train.h"
#include "track.h"

// possible game modes (maybe consider inheritance from Game)
enum GameMode
{
  FREEPLAY,
  RACE,
  SURVIVAL,
  PUZZLE
};

class Game
{
public:
  Game() {};

private:
  int _mode;
  int _score;
  int _lives;
};