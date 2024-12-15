
#include <Arduino.h>

#include "common.h"
#include "game.h"

// singleton instance of Game manages all game state and board hardware
Game &game = Game::getInstance();

void setup()
{
  Serial.begin(9600);
  delay(5000);

  log("Starting setup...");

  // disable ADC to save power
  ADCSRA = 0;

  game.setup();
  // game.modeSelect();

  log("Setup complete!");
}

void loop()
{
  if (game.isOver())
  {
    // game.modeSelect();
    game.restart(); 
  }

  game.tick();

  delay(50); // TODO: configurable to control overall game speed
}
