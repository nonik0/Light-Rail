#pragma once

#include <Adafruit_IS31FL3731.h>
#include <Arduino.h>
#include <AS1115.h>
#include <avr/io.h>

#include "common.h"
#include "platform.h"
#include "train.h"
#include "track.h"
#include "tone.h"

#define BUZZ 8 // PB4
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
  static const uint8_t DigitIntensity = 3;
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
  void readSwitches();

  // hardware resources
  const Track *TrackGraph = reinterpret_cast<const Track *>(TrackData);
  Adafruit_IS31FL3731 _boardLeds = Adafruit_IS31FL3731(); // TODO: write own optimized code for IS31
  AS1115 _boardDigits = AS1115(0x00);

  GameMode _mode;
  bool _isOver;

  // game state
  uint8_t _numTrains;
  Train _trains[MaxTrains];
  Platform _platforms[PlatformCount];
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
  DDRD &= ~(1 << SW5 | 1 << SW7 | 1 << SW8);
  DDRE &= ~(1 << SW6 | 1 << SW12);
  DDRF &= ~(1 << SW9 | 1 << SW10 | 1 << SW11);

  PORTB |= (1 << SW1 | 1 << SW2);
  PORTC |= (1 << SW3 | 1 << SW4);
  PORTD |= (1 << SW5 | 1 << SW7 | 1 << SW8);
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

  for (uint8_t i = 0; i < PlatformCount; i++)
  {
    uint8_t platformLoc = Platforms[i];
    uint8_t trackLoc = TrackGraph[platformLoc].anodeNextLoc2; // all properties same
    _platforms[i].setTrack(TrackGraph, platformLoc, trackLoc, setLed);
  }
}

int cargoLoaded = 0;
void Game::tick()
{
  readSwitches();

  for (uint8_t i = 0; i < _numTrains; i++)
  {
    _trains[i].advance();
  }

  for (uint8_t i = 0; i < PlatformCount; i++)
  {
    _platforms[i].tick();
  }

  // quick hack so i can sleep: check if any trains are at an occupied platform and clear it
  for (uint8_t i = 0; i < _numTrains; i++)
  {
    for (uint8_t j = 0; j < PlatformCount; j++)
    {
      if (_trains[i].front() == _platforms[j].track() && _platforms[j].hasCargo())
      {
        _platforms[j].loadCargo();
        setLed(_platforms[j].platform(), 0);

        cargoLoaded++;
        _boardDigits.display(cargoLoaded);
      }
    }
  }
}

bool Game::isOver()
{
  return _isOver;
}

void Game::restart()
{
  // reset game state
  _trains[0].init(69, 1); // TODO: randomize start location
  _trains[0].addCar(0);
  _trains[0].addCar(0);

  _trains[1].init(90, 1); // TODO: randomize start location
  _trains[1].addCar(1);
  _trains[1].addCar(0);
  _trains[1].addCar(0);

  _numTrains = 2;

  _isOver = false;
}

void Game::readSwitches()
{
  // check if any switches are pressed
  if (!(PINB & (1 << SW1)))
  {
    _boardDigits.display(1);
    tone(BUZZ, 1000, 100);
  }
  else if (!(PINB & (1 << SW2)))
  {
    _boardDigits.display(2);
    tone(BUZZ, 2000, 100);
  }
  else if (!(PINC & (1 << SW3)))
  {
    _boardDigits.display(3);
    tone(BUZZ, 3000, 100);
  }
  else if (!(PINC & (1 << SW4)))
  {
    _boardDigits.display(4);
    tone(BUZZ, 4000, 100);
  }
  else if (!(PIND & (1 << SW5)))
  {
    _boardDigits.display(5);
    tone(BUZZ, 5000, 100);
  }
  else if (!(PINE & (1 << SW6)))
  {
    _boardDigits.display(6);
    tone(BUZZ, 6000, 100);
  }
  else if (!(PIND & (1 << SW7)))
  {
    _boardDigits.display(7);
    tone(BUZZ, 7000, 100);
  }
  else if (!(PIND & (1 << SW8)))
  {
    _boardDigits.display(8);
    tone(BUZZ, 8000, 100);
  }
  else if (!(PINF & (1 << SW9)))
  {
    _boardDigits.display(9);
    tone(BUZZ, 9000, 100);
  }
  else if (!(PINF & (1 << SW10)))
  {
    _boardDigits.display(10);
    tone(BUZZ, 10000, 100);
  }
  else if (!(PINF & (1 << SW11)))
  {
    _boardDigits.display(11);
    tone(BUZZ, 11000, 100);
  }
  else if (!(PINE & (1 << SW12)))
  {
    _boardDigits.display(12);
    tone(BUZZ, 12000, 100);
  }
}