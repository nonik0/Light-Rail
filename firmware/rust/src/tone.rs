use atmega_hal::{
    clock::Clock,
    port::{mode::Output, Dynamic, Pin},
};
use core::cell::RefCell;

type Timer = atmega_hal::pac::TC1;
type Prescalar = avr_device::atmega32u4::tc1::tccr1b::CS1_A;

static TONE_STATE: avr_device::interrupt::Mutex<RefCell<Option<ToneState>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

struct ToneState {
    timer: Timer,
    output_pin: Pin<Output, Dynamic>,
    toggle_count: Option<u64>,
}

pub struct Timer1Tone {}

impl Timer1Tone {
    // TODO: is singleton pattern needed if we take the Timer1 peripheral as input?
    pub fn new(timer: Timer, output_pin: Pin<Output, Dynamic>) -> Self {
        let state = ToneState {
            timer,
            output_pin,
            toggle_count: None,
        };

        avr_device::interrupt::free(|cs| {
            let mut state_opt = TONE_STATE.borrow(cs).borrow_mut();
            *state_opt = Some(state);
        });

        unsafe {
            avr_device::interrupt::enable();
        }

        Self {}
    }

    pub fn tone(&mut self, frequency: u16, duration: u16) {
        if frequency == 0 {
            self.no_tone();
            return;
        }

        // calculate prescalar, overflow value, and toggle count for CTC mode
        // OCR1A = CoreClockHz / TargetHz / Prescalar - 1
        let mut ocr: u32 = crate::CoreClock::FREQ / frequency as u32 / 2 - 1;
        let mut prescalar = Prescalar::DIRECT;
        if ocr > 0xFFFF {
            ocr = crate::CoreClock::FREQ / frequency as u32 / 2 / 64 - 1;
            prescalar = Prescalar::PRESCALE_64;
        }
        let toggle_count = if duration > 0 {
            Some(2 * frequency as u64 * duration as u64 / 1000)
        } else {
            None
        };

        // update static state and enable interrupt
        avr_device::interrupt::free(|cs| {
            let state_opt_refcell = TONE_STATE.borrow(cs);
            let mut state_opt = state_opt_refcell.borrow_mut();
            let state = state_opt.as_mut().unwrap();

            state.output_pin.set_low();
            state.toggle_count = toggle_count;

            // configure timer for CTC mode for the desired frequency
            // WGM1 = 0b0100, CTC mode
            // CS1 = 0b001/prescalar1 or 0b011/prescalar64
            state.timer.tccr1a.write(|w| w.wgm1().bits(0b00));
            state
                .timer
                .tccr1b
                .write(|w| w.cs1().variant(prescalar).wgm1().bits(0b01));
            state.timer.ocr1a.write(|w| w.bits(ocr as u16));
            state.timer.timsk1.write(|w| w.ocie1a().set_bit());
        });
    }

    pub fn no_tone(&mut self) {
        avr_device::interrupt::free(|cs| {
            let state_opt_refcell = TONE_STATE.borrow(cs);
            let mut state_opt = state_opt_refcell.borrow_mut();
            let state = state_opt.as_mut().unwrap();

            state.output_pin.set_low();
            state.timer.timsk1.write(|w| w.ocie1a().clear_bit());
            state.toggle_count = None;
        });
    }
}

#[avr_device::interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    avr_device::interrupt::free(|cs| {
        let mut state_opt = TONE_STATE.borrow(cs).borrow_mut();
        let state = state_opt.as_mut().unwrap(); // unwrap is safe here bc interrupt won't be enabled if state is None

        state.output_pin.toggle();

        if let Some(mut toggles_left) = state.toggle_count {
            toggles_left -= 1;
            if toggles_left == 0 {
                state.timer.timsk1.write(|w| w.ocie1a().clear_bit());
                state.toggle_count = None;
            } else {
                state.toggle_count = Some(toggles_left);
            }
        }
    })
}
