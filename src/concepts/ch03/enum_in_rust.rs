#[derive(Debug)]
enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto { account_id: String, amount: f32 },
}

#[derive(Debug)]
pub struct DebitData {
    user_id: i32,
    debit_wallet_id: String,
}

#[allow(unreachable_patterns)]
pub fn enum_in_rust_concept_fn() {
    let some_payment = Payment::Cash(100.0);
    let debit_info = DebitData {
        user_id: 1000,
        debit_wallet_id: "1121-2332-343-233-0000".to_owned(),
    };

    let vec_options: Vec<Payment> = vec![
        Payment::Cash(100.0),
        Payment::CreditCard("Steven".to_string(), 1000.00),
        Payment::DebitCard(debit_info),
        Payment::Crypto {
            account_id: "121-232-0000-2323-1111".to_string(),
            amount: 12923.2,
        },
    ];

    println!("{vec_options:#?}");
    for item in vec_options {
        match item {
            Payment::Cash(amt) => println!("{amt}"),
            Payment::CreditCard(user_name, amt) => {
                println!("user extracted --> {}, {}", user_name, amt)
            }
            Payment::DebitCard(debit_info_data) => println!(
                "user extracted --> {}, {}",
                debit_info_data.user_id, debit_info_data.debit_wallet_id
            ),
            Payment::Crypto { account_id, amount } => {
                println!("user extracted --> {}, {}", account_id, amount)
            }

            _ => println!("---"),
        }
    }
}
