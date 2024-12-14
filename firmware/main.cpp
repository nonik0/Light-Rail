
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
    // game.modeSelect();
    game.restart(); 
  }

  delay(100);
}
