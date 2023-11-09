# Rust Experiment

This repository is for experimental purposes that involve using the latest APIs
in Rust and other functionalities.

## ASYNC CONFIGURATION


### JavaScript Async Simple

```js

// Async code example
console.log("Start");

setTimeout(() => {
  console.log("We are in the timeout ... ");
}, 5000);

for (let i = 0; i < 10; i++) {
  console.log(`The value of i -> ${i}`);
}
```


### Equavant in Rust

```rs
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");

    // Spawn a new asynchronous task for the timeout
    let timeout_task = task::spawn(async {
        sleep(Duration::from_secs(5)).await;
        println!("We are in the timeout ... ");
    });

    // Perform the synchronous loop
    for i in 0..10 {
        println!("The value of i -> {}", i);
    }

    // Await the timeout task to complete
    timeout_task.await.unwrap();
}

```
