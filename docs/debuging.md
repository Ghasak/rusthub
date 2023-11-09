# Debugging

For current `64-bit arm`, `M1 MacBook Pro` the implementation of `gdp` is not
available yet (up to `11/11/2022`).

## Debugging with lldb

An alternative, you can use the `lldb` which comes with the debugger of the
`vs-code` or via `nvim`. Usually I use:

1. In the terminal use the shell command
```shell
lldb ./target/debug/rusthub
```
This will inform the `lldb` that we want to run this binary.

2. Inside the lldb `REPL` you can use


```rust
process launch
or
run
or simply
r
```

- Read more about using the `lldb` [here](https://lldb.llvm.org/use/tutorial.html)







































