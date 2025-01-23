use std::sync::{LazyLock, Mutex};

struct Singleton {
    value: i32,
}

// Global static instance of the singleton
static SINGLETON: LazyLock<Mutex<Singleton>> =
    LazyLock::new(|| Mutex::new(Singleton { value: 42 }));

impl Singleton {
    fn get_value() -> i32 {
        let instance = SINGLETON.lock().unwrap();
        instance.value
    }

    fn set_value(value: i32) {
        let mut instance = SINGLETON.lock().unwrap();
        instance.value = value;
    }
}

fn main() {
    // Access the singleton instance
    println!("Initial value: {}", Singleton::get_value());

    // Modify the singleton value
    Singleton::set_value(100);
    println!("Updated value: {}", Singleton::get_value());
}
