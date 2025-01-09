#pragma once

#include <Adafruit_IS31FL3731.h>
#include <Arduino.h>
#include <AS1115.h>

#include "common.h"
#include "platform.h"
#include "train.h"
#include "track.h"

#define BUZZ PB4
#define SW1 PB6
#define SW2 PB7
#define SW3 PC6
#define SW4 PC7
#define SW5 PD4
#define SW6 PE2
#define SW7 PD6
#define SW8 PD7
#define SW9 PF4
#define SW10 PF1
#define SW11 PF0
#define SW12 PE6


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
  AS1115 _boardDigits = AS1115(0x00);

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


  // set buzzer pin to output
  PORTB |= (1 << BUZZ);

  // set all switch pins to input pullup
  DDRB &= ~(1 << SW1 | 1 << SW2);
  DDRC &= ~(1 << SW3 | 1 << SW4);
  DDRD &= ~(1 << SW5 | 1 << SW7);
  DDRE &= ~(1 << SW6 | 1 << SW12);
  DDRF &= ~(1 << SW9 | 1 << SW10 | 1 << SW11);

  PORTB |= (1 << SW1 | 1 << SW2);
  PORTC |= (1 << SW3 | 1 << SW4);
  PORTD |= (1 << SW5 | 1 << SW7);
  PORTE |= (1 << SW6 | 1 << SW12);
  PORTF |= (1 << SW9 | 1 << SW10 | 1 << SW11);

  _boardLeds.begin();
  _boardLeds.clear();

  _boardDigits.init(NumDigits, DigitIntensity);
  _boardDigits.clear();

  for (uint8_t i = 0; i < MaxTrains; i++)
  {
    _trains[i].setTrack(TrackGraph, setLed);
  }

  // TODO: switches
}

int i = 0;
int count = 0;
void Game::tick()
{
  // for (uint8_t i = 0; i < _numTrains; i++)
  // {
  //   _trains[i].advance();
  // }

  _boardLeds.setLEDPWM(i, 0);
  i = (i + 1) % 144;
  _boardLeds.setLEDPWM(i, 200);

  _boardDigits.display(count);
  count = (count + 1) % 1000;
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