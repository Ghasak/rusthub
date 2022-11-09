



mod concepts;
use concepts::create_text;
use concepts::ch01::{ownership_borrowing,common_collections};


fn main() {
    println!("Hello, world!");
    create_text();

    ownership_borrowing::about_owner_ship_concepts();
    common_collections::common_collections_fn();



}
