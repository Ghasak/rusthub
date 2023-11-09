// Simple function we call here
pub fn create_text() {
    println!("This is a function called from mod.rs of concepts ...")
}

pub mod ch01;
pub mod ch02;
pub mod ch03;
pub mod easy_rust;
pub mod playground;

use std::collections::{self, HashMap};

/// Our Emoji Storage dictionary
///
/// # Function to bring emoji by name,
/// Args:
/// - `&str`: string literal for getting the name of the icon to be brought form the dictionary.
/// output:
/// - `String` icons, ownership released from the function, so can be used anyhere.
/// ```shell
///     [🚀] current value is =>it is  1 !
///     [🎉] current value is =>it is  2 !
/// ```
/// List of options:
/// - rocket
/// - sparkles
/// - fire
/// - party_popper
/// - wrapped_gift
/// - light_bulb
/// - info, warn, error, trace, pass
/// - robot, pass, exclamation_red, question_white, question_red,
/// - exclamation_white,double_exclamation, exclamation_question
pub fn my_emoji(icon_name: &str) -> String {
    let mut emoji_storage: HashMap<String, String> = HashMap::new();
    emoji_storage.insert(String::from("rocket"), String::from("🚀"));
    emoji_storage.insert(String::from("sparkles"), String::from("✨"));
    emoji_storage.insert(String::from("fire"), String::from("🔥"));
    emoji_storage.insert(String::from("party_popper"), String::from("🎉"));
    emoji_storage.insert(String::from("wrapped_gift"), String::from("🎁"));
    emoji_storage.insert(String::from("light_bulb"), String::from("💡"));
    emoji_storage.insert(String::from("info"), String::from("ℹ️ "));
    emoji_storage.insert(String::from("warn"), String::from("⚠️ "));
    emoji_storage.insert(String::from("error"), String::from("❌"));
    emoji_storage.insert(String::from("trace"), String::from("🔬"));
    emoji_storage.insert(String::from("robot"), String::from("🤖"));
    emoji_storage.insert(String::from("pass"), String::from("✅"));
    emoji_storage.insert(String::from("exclamation_red"), String::from("❗"));
    emoji_storage.insert(String::from("question_white"), String::from("❔"));
    emoji_storage.insert(String::from("exclamation_white"), String::from("❕"));
    emoji_storage.insert(String::from("question_red"), String::from("❓"));
    emoji_storage.insert(String::from("double_exclamation"), String::from("‼️ "));
    emoji_storage.insert(String::from("exclamation_question"), String::from("⁉️ "));

    let return_icon = emoji_storage.get(icon_name);

    match return_icon {
        Some(val) => val.clone(),
        None => "We couldn't find the icon".to_string(),
    }
}
