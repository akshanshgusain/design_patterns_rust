use crate::{Amplifier, Tuner};
use std::cell::RefCell;
use std::rc::Rc;

pub struct HomeTheaterFacade {
    amp: Amplifier,
    tuner: Rc<RefCell<Tuner>>,
    /*
    Why use Rc with RefCell together?
    The problem arises when you need shared ownership (Rc) and mutation (RefCell).
    Since Rc only allows shared ownership and doesn’t allow mutation, combining it with RefCell solves
    this limitation.

    Use case: When multiple owners need to mutate the same value
    */
}

impl HomeTheaterFacade {
    pub fn new(amp: Amplifier, tuner: Tuner) -> Self {
        HomeTheaterFacade {
            amp,
            tuner: Rc::new(RefCell::new(tuner)),
        }
    }

    pub fn listen_radio(&mut self, frequency: f64) {
        println!("Tuning in the airwaves...");
        self.tuner.borrow().on();
        self.tuner.borrow_mut().set_frequency(frequency);
        self.amp.on();
        self.amp.set_volume(5);
        self.amp.set_tuner(Rc::clone(&self.tuner));
    }

    pub fn stop_radio(&self) {
        println!("Shutting down the radio...");
        self.tuner.borrow().off();
        self.amp.off();
    }
}
