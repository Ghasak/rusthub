// struct ByteBuf<'a>(&'a [u8]);
//
// impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
//     fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         for byte in self.0 {
//             fmtr.write_fmt(format_args!("{:02x}", byte));
//         }
//         Ok(())
//     }
// }
/// Rust struct, Traits and implementations
/// # Concepts and fundamental function
/// ## Notes
/// - for demonstration purposes only.

pub fn rust_structs_traits_and_implementation_fn() {
    let var: i8 = 123;
    let mut sum: i8 = 0;
    for i in 0..10 {
        println!("{sum:#?}");
        sum +=i;
    }
    // 123 -> 7b00
    // let mut buff = [0_u8; 2];
    // buff[0] = 123;
    //
    // println!("{:x}", ByteBuf(&buff));
}
