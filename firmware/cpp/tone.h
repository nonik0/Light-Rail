
#pragma once

// compacted impl of Arduino tone.cpp for atmega32u4 (using timer3)
#include <Arduino.h>

static uint8_t tone_pin2;
volatile long toggle_count2;
volatile uint8_t *pin_port2;
volatile uint8_t pin_mask2;


#define TCCR3A _SFR_MEM8(0x90)
#define WGM30 0
#define WGM31 1
#define COM3C0 2
#define COM3C1 3
#define COM3B0 4
#define COM3B1 5
#define COM3A0 6
#define COM3A1 7

#define TCCR3B _SFR_MEM8(0x91)
#define CS30 0 XXXXXX
#define CS31 1
#define CS32 2 XXXXX
#define WGM32 3
#define WGM33 4
#define ICES3 6
#define ICNC3 7

//TCCR3A = 0b00000000;
//TCCR3B = 0b0000001;

void tone2(uint8_t _pin, unsigned int frequency, unsigned long duration)
{
    TCCR3A = 0;
    TCCR3B = 0;
    bitWrite(TCCR3B, WGM32, 1);
    bitWrite(TCCR3B, CS30, 1);
    pin_port2 = portOutputRegister(digitalPinToPort(_pin));
    pin_mask2 = digitalPinToBitMask(_pin);

    pinMode(_pin, OUTPUT);

    // two choices for the 16 bit timers: ck/1 or ck/64
    uint32_t ocr = F_CPU / frequency / 2 - 1;
    uint8_t prescalarbits = 0b001;
    if (ocr > 0xffff)
    {
        ocr = F_CPU / frequency / 2 / 64 - 1;
        prescalarbits = 0b011;
    }

    toggle_count2 = duration > 0 ? 2 * frequency * duration / 1000 : -1;
    TCCR3B = (TCCR3B & 0b11111000) | prescalarbits;
    OCR3A = ocr;
    bitWrite(TIMSK3, OCIE3A, 1);
}

void disableTimer2()
{
    bitWrite(TIMSK3, OCIE3A, 0);
}

ISR(TIMER3_COMPA_vect)
{
    if (toggle_count2 != 0)
    {
        *pin_port2 ^= pin_mask2;

        if (toggle_count2 > 0)
        {
            toggle_count2--;
        }
    }
    else
    {
        disableTimer2();
        *pin_port2 &= ~(pin_mask2); // keep pin low after stop
    }
}

void noTone2(uint8_t _pin)
{
    if (tone_pin2 == _pin)
    {
        tone_pin2 = 255;
    }

    disableTimer2();
    digitalWrite(_pin, 0);
}
