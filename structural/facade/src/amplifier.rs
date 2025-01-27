use crate::tuner::Tuner;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Amplifier {
    description: String,
    tuner: Option<Rc<RefCell<Tuner>>>,
    /*
    Why use Rc with RefCell together?
    The problem arises when you need shared ownership (Rc) and mutation (RefCell).
    Since Rc only allows shared ownership and doesn’t allow mutation, combining it with RefCell solves
    this limitation.

    Use case: When multiple owners need to mutate the same value

    */
    level: i32,
}

impl Amplifier {
    pub fn new(description: &str) -> Self {
        Amplifier {
            description: String::from(description),
            tuner: None,
            level: 0,
        }
    }

    pub fn on(&self) {
        println!("{} on", self.description);
    }

    pub fn off(&self) {
        println!("{} off", self.description);
    }

    pub fn set_volume(&mut self, level: i32) {
        self.level = level;
        println!("{} setting volume to {}", self.description, level);
    }

    pub fn set_tuner(&mut self, tuner: Rc<RefCell<Tuner>>) {
        self.tuner = Some(tuner);
    }
}
