#pragma once

#ifdef DEBUG
#define LOGFBUFSIZE 128
#define log(x) Serial.println(x)
#else
#define log(x)
#endif