#pragma once

#include <Arduino.h>

class Track {
  public:
    Track() {};
  private:
    int _ledIndex;
    Track* _next;
    Track* _prev;
};