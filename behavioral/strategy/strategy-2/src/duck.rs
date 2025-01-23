use crate::FlyBehavior;
use crate::QuackBehavior;

pub trait Duck {
    fn display(&self);

    /*
    Generic Approach (T: FlyBehavior):
    Uses static dispatch (compile-time). The compiler generates a separate version of the method
    for each type T that you use with the method.
    Results in faster execution since there’s no runtime overhead for dispatching calls.
    Requires knowing the concrete type of fly at compile time.
    */
    fn perform_fly<T: FlyBehavior>(&self, fly: &T) {
        fly.fly();
    }

    /*
    Trait Object (Box<dyn FlyBehavior>):
    Uses dynamic dispatch (runtime). The actual method implementation is determined at runtime
    using a vtable (virtual method table).
    Adds a slight runtime overhead due to dynamic dispatch.
    Allows more flexibility because the concrete type of fly does not need to be known at compile time.
    */
    fn perform_kick(&self, kick: Box<dyn FlyBehavior>){
        kick.fly();
    }

    fn perform_quack<T: QuackBehavior>(&self, quack: &T) {
        quack.quack();
    }
    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct MallardDuck;
impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}

pub struct ModelDuck;
impl Duck for ModelDuck {
    fn display(&self) {
        println!("I'm a model duck");
    }
}
