# Enums and Pattern Matching

- Using `Enum` when you have a `pattern` that need variant, a selection of one
  option among many, that you access one at a time, which is a bit different
  from `strcut` which is used to store many optoins and use them all combined.


```rust
    #[derive(Debug)]
    enum IpAddressKind {
        V4,
        V6,
    }

    let four: IpAddressKind = IpAddressKind::V4;
    let six: IpAddressKind = IpAddressKind::V6;

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddressKind,
        address: String,
    }

    let home: IpAddr = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr{
        kind: IpAddressKind::V6,
        address: String::from("::1")
    };


    println!("We have home -> {:#?}", home);

```
- Representing the same concept using just an enum is more concise: rather than
an enum inside a struct, we can put data directly into each enum variant. This
new definition of the IpAddr enum says that both V4 and V6 variants will have
associated String values:

```rust
    #[derive(Debug)]
    enum IpAddressKind {
        V4(String),
        V6(String),
    }

    let home = IpAddressKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddressKind::V6(String::from("::1"));


    println!("We have home     -> {:#?}", home);
    println!("We have  loopback-> {:#?}", loopback);

```
