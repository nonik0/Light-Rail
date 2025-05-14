use atmega_hal::{
    clock::Clock,
    port::{mode::Output, Dynamic, Pin},
};
use core::cell::RefCell;

type Timer = atmega_hal::pac::TC3;
type Prescalar = avr_device::atmega32u4::tc3::tccr3b::CS3_A;

static TONE_STATE: avr_device::interrupt::Mutex<RefCell<Option<ToneState>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

struct ToneState {
    timer: Timer,
    output_pin: Pin<Output, Dynamic>,
    toggle_count: Option<u64>,
}

// TODO: try using trait to abstract the timer peripheral
// TODO: use generic type for output pin
pub struct TimerTone {}

impl TimerTone {
    // TODO: is singleton pattern needed if we take the Timer3 peripheral as input?
    pub fn new(timer: Timer, output_pin: Pin<Output, Dynamic>) -> Self {
        let state = ToneState {
            timer,
            output_pin,
            toggle_count: None,
        };

        // set timer for CTC mode, WGM3 = 0b0100
        state.timer.tccr3a.write(|w| w.wgm3().bits(0b00));
        state.timer.tccr3b.write(|w| w.wgm3().bits(0b01));

        avr_device::interrupt::free(|cs| {
            let mut state_opt = TONE_STATE.borrow(cs).borrow_mut();
            *state_opt = Some(state);
        });

        // TODO: should caller/owner be responsible for enabling interrupts?
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
        // OCR3A = CoreClockHz / TargetHz / Prescalar - 1
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

        // update static state
        avr_device::interrupt::free(|cs| {
            let state_opt_refcell = TONE_STATE.borrow(cs);
            let mut state_opt = state_opt_refcell.borrow_mut();
            let state = state_opt.as_mut().unwrap();

            state.output_pin.set_low();
            state.toggle_count = toggle_count;

            // uppdate timer for desired frequency
            // CS3 = 0b001/prescalar1 or 0b011/prescalar64
            state.timer.tccr3b.modify(|_, w| w.cs3().variant(prescalar));
            state.timer.ocr3a.write(|w| w.bits(ocr as u16));
            state.timer.timsk3.write(|w| w.ocie3a().set_bit());
        });
    }

    pub fn no_tone(&mut self) {
        avr_device::interrupt::free(|cs| {
            let state_opt_refcell = TONE_STATE.borrow(cs);
            let mut state_opt = state_opt_refcell.borrow_mut();
            let state = state_opt.as_mut().unwrap();

            state.output_pin.set_low();
            state.timer.timsk3.write(|w| w.ocie3a().clear_bit());
            state.toggle_count = None;
        });
    }
}

#[avr_device::interrupt(atmega32u4)]
#[allow(non_snake_case)]
fn TIMER3_COMPA() {
    avr_device::interrupt::free(|cs| {
        let mut state_opt = TONE_STATE.borrow(cs).borrow_mut();
        let state = state_opt.as_mut().unwrap(); // unwrap is safe here bc interrupt won't be enabled if state is None

        state.output_pin.toggle();

        if let Some(mut toggles_left) = state.toggle_count {
            toggles_left -= 1;
            if toggles_left == 0 {
                state.timer.timsk3.write(|w| w.ocie3a().clear_bit());
                state.toggle_count = None;
            } else {
                state.toggle_count = Some(toggles_left);
            }
        }
    })
}
