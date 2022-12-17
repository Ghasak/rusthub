# Several way to do profiling

Profilers

There are many different profilers available, each with their strengths and
weaknesses. The following is an incomplete list of profilers that have been
used successfully on Rust programs.

perf is a general-purpose profiler that uses hardware performance counters.
Hotspot and Firefox Profiler are good for viewing data recorded by perf. It
works on Linux. Instruments is a general-purpose profiler that comes with Xcode
on macOS. AMD μProf is a general-purpose profiler. It works on Windows and
Linux. flamegraph is a Cargo command that uses perf/DTrace to profile your code
and then displays the results in a flame graph. It works on Linux and all
platforms that support DTrace (macOS, FreeBSD, NetBSD, and possibly Windows).
Cachegrind & Callgrind give global, per-function, and per-source-line
instruction counts and simulated cache and branch prediction data. They work on
Linux and some other Unixes. DHAT is good for finding which parts of the code
are causing a lot of allocations, and for giving insight into peak memory
usage. It can also be used to identify hot calls to memcpy. It works on Linux
and some other Unixes. dhat-rs is an experimental alternative that is a little
less powerful and requires minor changes to your Rust program, but works on all
platforms. heaptrack and bytehound are heap profiling tools. They work on
Linux. counts supports ad hoc profiling, which combines the use of eprintln!
statement with frequency-based post-processing, which is good for getting
domain-specific insights into parts of your code. It works on all platforms.
Coz performs causal profiling to measure optimization potential, and has Rust
support via coz-rs. It works on Linux.

## Profiling your project

```rust
cargo install flamegraph
```
```rust
flamegraph -o my_flamegraph.svg -- ./target/debug/rust_learning_hub
```

## Using the `Instrument` form apple
- [Ref.01](http://www.cmyr.net/blog/rust-profiling.html)
- [Ref.02](https://gist.github.com/loderunner/36724cc9ee8db66db305)

## REFERENCES
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/profiling.html)
- [How to use the instruments](https://knowledge.broadcom.com/external/article/180011/how-to-use-macintosh-xcodes-instruments.html)
- [Using the flamegraph](https://github.com/flamegraph-rs/flamegraph)
