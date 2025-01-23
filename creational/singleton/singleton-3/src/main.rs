use std::sync::{Once, Arc, Mutex};

struct Singleton {
    value: String,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            value: "Default Value".to_string(),
        }
    }

    fn set_value(&mut self, new_value: &str) {
        self.value = new_value.to_string();
    }

    fn get_value(&self) -> &str {
        &self.value
    }
}

struct SingletonManager {
    instance: Arc<Mutex<Singleton>>,
}

impl SingletonManager {
    fn get_instance() -> Arc<Mutex<Singleton>> {
        static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                SINGLETON = Some(Arc::new(Mutex::new(Singleton::new())));
            });
            SINGLETON.clone().unwrap()
        }
    }
}

fn main() {
    let instance1 = SingletonManager::get_instance();
    {
        let mut singleton = instance1.lock().unwrap();
        singleton.set_value("Singleton Initialized");
    }

    let instance2 = SingletonManager::get_instance();
    {
        let singleton = instance2.lock().unwrap();
        println!("Singleton Value: {}", singleton.get_value());
    }
}
