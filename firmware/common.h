#pragma once

#ifdef DEBUG
#define log(x) Serial.println(x)
#else
#define log(x)
#endif