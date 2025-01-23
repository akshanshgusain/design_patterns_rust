using the Lazy type - LazyLock + Mutex


Rust does not have classes or objects in the traditional sense, but you can achieve the singleton pattern using global static variables and lazy initialization.
Use LazyLock for thread-safe lazy initialization.
Wrap the singleton in a Mutex or RwLock if mutability is required.