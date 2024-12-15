#pragma once

#include <Adafruit_IS31FL3731.h>
#include <Arduino.h>
#include <AS1115.h>

#include "common.h"
#include "platform.h"
#include "train.h"
#include "track.h"

// TODO: make sure all game resources are statically allocated

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
  static const uint8_t MaxTrains = 5;
  static const uint8_t NumDigits = 3;
  static const uint8_t DigitIntensity = 5;
  // static const uint8_t NumPlatforms = X;
  static void setLed(uint8_t ledNum, uint8_t brightness);
  // static void setDigit(uint8_t index, uint8_t value); // print number
  // static void setDigit(uint8_t index, uint8_t rawValue); // control individual segments
  //  TODO: helper functions for digits?

  // singleton pattern
  static Game &getInstance();
  Game(const Game &) = delete;
  Game(Game &&) = delete;
  Game &operator=(const Game &) = delete;
  Game &operator=(Game &&) = delete;

  void setup();   // called once
  void restart(); // called each time game is over
  void tick();    // called successively, frequency affects game speed
  bool isOver();  // used to determine when to restart

private:
  // singleton pattern
  Game() {}
  ~Game() {}

  // hardware resources
  const Track *TrackGraph = reinterpret_cast<const Track *>(TrackData);
  Adafruit_IS31FL3731 _boardLeds = Adafruit_IS31FL3731(); // TODO: write own optimized code for IS31
  AS1115 _boardDigits = AS1115(0x13);

  GameMode _mode;
  bool _isOver;

  // game state
  uint8_t _numTrains;
  Train _trains[MaxTrains];
  // Platform _platforms[NumPlatforms];
};

Game &Game::getInstance()
{
  static Game game;
  return game;
}

void Game::setLed(uint8_t ledNum, uint8_t brightness)
{
  getInstance()._boardLeds.setLEDPWM(ledNum, brightness);
}

void Game::setup()
{
  log("Game setup...");

  _mode = ANIMATION;
  _isOver = true;

  _boardLeds.begin();
  _boardLeds.clear();

  // _boardDigits.init(NumDigits, DigitIntensity);
  // _boardDigits.clear();

  for (uint8_t i = 0; i < MaxTrains; i++)
  {
    _trains[i].setTrack(TrackGraph, setLed);
  }

  // TODO: switches
}

void Game::tick()
{
  for (uint8_t i = 0; i < _numTrains; i++)
  {
    _trains[i].advance();
  }
}

bool Game::isOver()
{
  return _isOver;
}

void Game::restart()
{
  // reset game state
  _trains[0].init(0, 1); // TODO: randomize start location
  _trains[0].addCar(0);
  _trains[0].addCar(0);

  _trains[1].init(143, 1); // TODO: randomize start location
  _trains[1].addCar(1);
  _trains[1].addCar(0);
  _trains[1].addCar(0);

  _numTrains = 2;

  _isOver = false;
}