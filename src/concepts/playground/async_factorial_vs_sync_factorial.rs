pub fn comparison_factorial_results() {
    use std::time::Instant;

    // Compute factorial of N using sync
    fn factorial_sync(n: u128) -> u128 {
        if n == 0 {
            1
        } else {
            n * factorial_sync(n - 1)
        }
    }

    // Compute factorial of N using async and multi-threading
    fn factorial_async_multithreaded(n: u128) -> u128 {
        if n == 0 {
            1
        } else {
            let num_threads = num_cpus::get();
            let chunk_size = n / num_threads as u128;
            let mut handles = Vec::new();
            for i in 0..num_threads {
                let start = i as u128 * chunk_size + 1;
                let end = if i == num_threads - 1 {
                    n
                } else {
                    (i as u128 + 1) * chunk_size
                };
                let handle = std::thread::spawn(move || {
                    let mut sub_factorial = 1;
                    for j in start..=end {
                        sub_factorial *= j;
                    }
                    sub_factorial
                });
                handles.push(handle);
            }
            let mut sub_factorials = Vec::new();
            for handle in handles {
                sub_factorials.push(handle.join().unwrap());
            }
            let mut result = 1;
            for sub_factorial in sub_factorials {
                result *= sub_factorial;
            }
            result
        }
    }

    // Test the performance of the single-threaded and multithreaded implementations of factorial
    fn compare_factorial_results(n: u128) {
        let now = Instant::now();
        let result_sync = factorial_sync(n);
        let elapsed_sync = now.elapsed().as_micros();
        println!(
            "Factorial of {} using sync: {} (time: {}us)",
            n, result_sync, elapsed_sync
        );

        let now = Instant::now();
        let result_async_mt = factorial_async_multithreaded(n);
        let elapsed_mt = now.elapsed().as_micros();
        println!(
            "Factorial of {} using async and multi-threading: {} (time: {}us)",
            n, result_async_mt, elapsed_mt
        );
    }

    //fn main() {
    let n = 30;
    compare_factorial_results(n);
    //}
}

// use std::thread;
//
// fn factorial(n: u64) -> u64 {
//     if n == 0 {
//         return 1;
//     }
//
//     let num_threads = num_cpus::get();
//     let chunk_size = n / num_threads as u64;
//
//     let mut handles = vec![];
//     for i in 0..num_threads {
//         let start = i as u64 * chunk_size + 1;
//         let end = if i == num_threads - 1 {
//             n
//         } else {
//             (i as u64 + 1) * chunk_size
//         };
//         let handle = thread::spawn(move || {
//             let mut sub_factorial = 1;
//             for j in start..=end {
//                 sub_factorial *= j;
//             }
//             sub_factorial
//         });
//         handles.push(handle);
//     }
//
//     let mut sub_factorials = vec![];
//     for handle in handles {
//         sub_factorials.push(handle.join().unwrap());
//     }
//
//     let mut result = 1;
//     for sub_factorial in sub_factorials {
//         result *= sub_factorial;
//     }
//     result
// }
//
// let n = 20;
// let start = std::time::Instant::now();
// let result = factorial(n);
// let elapsed = start.elapsed();
//
// println!("Factorial of {} is: {}", n, result);
//println!("Elapsed time: {:?}", elapsed);
