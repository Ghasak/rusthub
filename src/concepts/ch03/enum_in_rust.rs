#[derive(Debug)]
enum Payment {
    Cash(f32),
    CreditCard(f32),
    DebitCard,
}

#[allow(unreachable_patterns)]
pub fn enum_in_rust_concept_fn() {
    let some_payment = Payment::Cash(100.0);

    let vec_options: Vec<Payment> = vec![
        Payment::Cash(100.0),
        Payment::CreditCard(21.23),
        Payment::DebitCard,
        Payment::CreditCard(21.23),
    ];
    for item in vec_options.iter() {
        match item {
            Payment::Cash(amt) => {
                println!("[+] Paying in cash! ${amt:#?} US dollars ...")
            }
            Payment::CreditCard(amt) => {
                println!("[+] Paying in credit card! ${amt:#?} ...")
            }

            Payment::DebitCard => {
                println!("[+] Paying in  debit card! ...")
            }
            _ => {}
        }
    }
}
