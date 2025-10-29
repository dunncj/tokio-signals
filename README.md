# tokio-signals
 Simple implementation of a Signal using Tokio's watch channel


## Usage

```rust 
use tokio_signals::signal::Signal;



fn main() {
    let count = Signal::new(0);

    // Set count to 10 
    count.set(10);

    let current_count = count.get();
    println!("Current count: {}", current_count);
    // Prints "Current count: 10"

    // Subscribe to changes
    count.subscribe(|new_value| {
        println!("Count changed reactively: {}", new_value);
    });

    // Update count
    count.set(20);
    // Prints "Count changed reactively: 20"
}




```