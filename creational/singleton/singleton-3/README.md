using std::sync::Once
std::sync::Once for thread-safe, one-time initialization.

1. Once ensures that the initialization code is executed only once, even in multithreaded scenarios.
2. Arc (Atomic Reference Counting) is used to share ownership of the Singleton across threads. 
3. Mutex ensures thread-safe access to the Singleton.

Rust does not have classes or objects in the traditional sense, but you can achieve the singleton pattern using global static variables and lazy initialization.
Use LazyLock for thread-safe lazy initialization.
Wrap the singleton in a Mutex or RwLock if mutability is required.