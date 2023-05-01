# Rust Enums

## Introduced

Based on lecture notes by `Doug Milford`. Enum is similar to other different
language, it's just a hard-coded list of options.

- Lets start by creating an `enum`, personally consider cash or credit card you
  use the lowercase.

```rust
#[derive(Debug)]
enum Payment {
    Cash,
    CreditCard,
}

pub fn enum_in_rust_concept_fn() {
    let some_payment = Payment::Cash;

    match some_payment {
        Payment::Cash => {
            println!("Paying in cash! ...")
        }
        Payment::CreditCard => {
            println!("Paying in credit card! ...")
        }
    }
}
```

- Watch what happens when we add a new itemm for a debit card now. We will get
  a compiler error on the `match` statement, stating that all options are not
  covered it. It performs an exhaustive check to ensure all scenarios have been
  handled by your code. This is more powerful than it first seems whenever.
  Whenever you add new items to `Enum` there's there's a danger that code using
  that `Enum` to be updated in order to handle the new item properly. Other
  languages often don't care will not alert to potential issues. The strictness
  of `Rrust` is a feature not a bug. The complier will help us to track all
  places that might need to be updated.

```rust

#[derive(Debug)]
enum Payment {
    Cash,
    CreditCard,
    DebitCard,
}

pub fn enum_in_rust_concept_fn() {
    let some_payment = Payment::Cash;

    match some_payment {
        Payment::Cash => {
            println!("Paying in cash! ...")
        }
        Payment::CreditCard => {
            println!("Paying in credit card! ...")
        }
    }
}
```

- Let's also update our `match` statement to handle the `Payment::DebitCard`
  with:

```rust
match some_payment {
    Payment::Cash => {
        println!("Paying in cash! ...")
    }
    Payment::CreditCard => {
        println!("Paying in credit card! ...")
    }
    Payment::DebitCard => {
        println!("Paying in debit card! ...")
    }
}
```

- You can also add for the `match` statement the `_` escape clause from the
  exhaustive check for all other options of the given `Enum`. It is recommended
  to use it only when it makes sense though

```rust

match some_payment {
    Payment::Cash => {
        println!("Paying in cash! ...")
    }
    Payment::CreditCard => {
        println!("Paying in credit card! ...")
    }
    Payment::DebitCard => {
        println!("Paying in debit card! ...")
    }
    _ => {
         println!("Not implemented selection ...")
        }
}
```

- Now, We will talk about one of the feature of `Enum` which is being able to
  associate data within `Enum` item. Lets associate `amount` to the `Cash` data
  ussing the `f32`. You have to provide the `data` associated with this
  selection when you call it.

```rust
#[derive(Debug)]
enum Payment {
    Cash(f32),
    CreditCard,
    DebitCard,
}

pub fn enum_in_rust_concept_fn() {
    let some_payment = Payment::Cash(100.0);

    match some_payment {
        Payment::Cash(val) => {
            println!("Paying in cash! ${val:#?} US dollars ...")
        }
        Payment::CreditCard => {
            println!("Paying in credit card! ...")
        }

        Payment::DebitCard => {
            println!("Paying in credit card! ...")
        }
    }
}
```

- Here is a nice example of using `Enum` with `Vec`.

```rust
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
```

- We are not limited to simple types in `Enum`, we can use more complicated
  data structure, such as;

```rust
enum Payment{
        Cash(f32),
        CreditCard(String, f32),
        DebitCard,
    }

```
- Or, using a `struct` such as this

```rust
enum Payment{
        Cash(f32),
        CreditCard(String, f32),
        DebitCard(DebitData),
    }
struct DebitData {
        pub card_number: String,
        pub amount : f32,
    }

```

- Another option is to create a strongly typed `Enum` similar to how you create
  a struct as in

```rust

enum Payment{
        Cash(f32),
        CreditCard(String, f32),
        DebitCard(DebitData),
        Crypto{account_id: String, amount: f32},
    }
```

- Its a better coding practise to avoid `tuple` unless its necessary, and use
  explicit names. Here I will finish the example include all possible ways to
  construct an `Enum`.

-
```rust
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
```

## Enum Summary
We have several ways to define a new `Enum` in `Rust`.

| idx | Enum Form                                  | Syntax             | Note                                                                    |
|-----|--------------------------------------------|--------------------|-------------------------------------------------------------------------|
| 1   | Cash(f32),                                 | variant(val)       | passing tuple for value                                                 |
| 2   | CreditCard(String, f32)                    | variant(val1,val2) | holding more values for a tuple                                         |
| 3   | DebitCard(DebitData)                       | variant(struct)    | hold a struct that has many field                                       |
| 4   | Crypto{val_nam: type, val_name2: type2 ..} | variant{struct}    | This is the only one needs to exlculsivly mentioned in the match clause |

- `Payment::Crypto { account_id, amount }` This is must match the same entries
  that you used when you construct your `Enum` that depends on the `struct`
  that it pull the data from.




