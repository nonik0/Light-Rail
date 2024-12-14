
#include <Arduino.h>

#include "common.h"
#include "game.h"

Game game;

void setup()
{
  log("Starting setup...");

  // disable ADC to save power
  ADCSRA = 0;

  game.setup();
  // game.modeSelect();

  log("   complete!");
}

int trainLoc = 0;
int trainSize = 40;

void loop()
{
  game.tick();
  if (game.isOver())
  {
    game.restart(); 
    // game.modeSelect();
  }

  // // clear train caboose from previous location
  // trackLeds.setLEDPWM(trainLoc, 0);

  // // advance train
  // trainLoc = (trainLoc + 1) % TRACK_COUNT;

  // // draw train at new location
  // for (int i = 0; i < trainSize; i++)
  // {
  //   trackLeds.setLEDPWM((trainLoc + i) % TRACK_COUNT, 60);
  // }

  delay(100);

  // digitalWrite(LED_BUILTIN, HIGH);
  // delay(1000);
  // digitalWrite(LED_BUILTIN, LOW);
  // delay(500);
}
