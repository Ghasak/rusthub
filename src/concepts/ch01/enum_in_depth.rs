pub fn enum_and_pattern_mathcing() {
    #[derive(Debug)]
    enum IpAddressKind {
        V4(String),
        V6(String),
    }

    let home = IpAddressKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddressKind::V6(String::from("::1"));


    println!("We have home     -> {:#?}", home);
    println!("We have  loopback-> {:#?}", loopback);
















}
