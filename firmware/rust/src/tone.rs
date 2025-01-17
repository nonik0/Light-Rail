use atmega_hal::clock::Clock;
use atmega_hal::port::mode::Output;
use atmega_hal::port::Pin;
use atmega_hal::port::PB4; // hardcoded pin for now to avoid any runtime cost
use core::cell::RefCell;

type Timer3 = atmega_hal::pac::TC3;
type Timer3Prescalar = avr_device::atmega32u4::tc3::tccr3b::CS3_A;

static TIMER3_TONE_STATE: avr_device::interrupt::Mutex<RefCell<Option<Timer3ToneState>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

struct Timer3ToneState {
    timer3: Timer3,
    pin: Pin<Output, PB4>,
    toggle_count: Option<u64>,
}

pub struct Timer3Tone {}

impl Timer3Tone {
    // TODO: is singleton pattern needed if we take the Timer3 peripheral as input?
    pub fn new(timer3: Timer3, pin: Pin<Output, PB4>) -> Self {
        let state = Timer3ToneState {
            timer3,
            pin,
            toggle_count: None,
        };

        avr_device::interrupt::free(|cs| {
            let mut state_opt = TIMER3_TONE_STATE.borrow(cs).borrow_mut();
            *state_opt = Some(state);
        });

        Self {}
    }

    pub fn tone(&mut self, frequency: u16, duration: u16) {
        // calculate prescalar, overflow value, and toggle count for CTC mode
        // OCR3A = CoreClockHz / TargetHz / Prescalar - 1
        let mut ocr: u32 = crate::CoreClock::FREQ / frequency as u32 / 2 - 1;
        let mut prescalar = Timer3Prescalar::DIRECT;
        if ocr > 0xFFFF {
            ocr = crate::CoreClock::FREQ / frequency as u32 / 2 / 64 - 1;
            prescalar = Timer3Prescalar::PRESCALE_64;
        }
        let toggle_count = if duration > 0 {
            Some(2 * frequency as u64 * duration as u64 / 1000)
        } else {
            None
        };

        // update static state and enable interrupt
        avr_device::interrupt::free(|cs| {
            let state_opt_refcell = TIMER3_TONE_STATE.borrow(cs);
            let mut state_opt = state_opt_refcell.borrow_mut();
            let state = state_opt.as_mut().unwrap();

            state.toggle_count = toggle_count;

            // configure timer for CTC mode for the desired frequency
            // WGM3 = 0b0100, CTC mode
            // CS3 = 0b001/prescalar1 or 0b011/prescalar64
            state.timer3.tccr3a.write(|w| w.wgm3().bits(0b00));
            state
                .timer3
                .tccr3b
                .write(|w| w.cs3().variant(prescalar).wgm3().bits(0b01));
            state.timer3.ocr3a.write(|w| w.bits(ocr as u16));
            state.timer3.timsk3.write(|w| w.ocie3a().set_bit());

            //TIMER3_TONE_STATE.borrow(cs).replace(Some(state));
        });
    }
}

#[avr_device::interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    avr_device::interrupt::free(|cs| {
        let mut state_opt = TIMER3_TONE_STATE.borrow(cs).borrow_mut();
        let state = state_opt.as_mut().unwrap(); // unwrap is safe here bc interrupt won't be enabled if state is None

        state.pin.toggle();

        if let Some(mut toggle_count) = state.toggle_count {
            toggle_count -= 1;
            if toggle_count == 0 {
                state.timer3.timsk3.write(|w| w.ocie3a().clear_bit());
                state.toggle_count = None;
            }
            else {
                state.toggle_count = Some(toggle_count);
            }

            //TIMER3_TONE_STATE.borrow(cs).replace(Some(*state));
        }
    })
}
