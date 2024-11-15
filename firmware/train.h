#pragma once

#include <Arduino.h>

#include "track.h"

class Train {
  public:
    Train() {}
  private:
    int _capacity;
    Track* _frontLoc;
};