/*
- Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
 */

use std::collections::HashMap;

pub fn main() {
    let first_string = "first";
    let first_letter = &first_string[0..1];
    let last_letters = &first_string[1..];
    let suffix = "ay";

    let mut map = HashMap::new();
    map.insert("a", "hay");
    map.insert("e", "hay");
    map.insert("i", "hay");
    map.insert("o", "hay");
    map.insert("u", "hay");

    let mut pig_latin_string = String::new();
    if map.contains_key(first_letter) {
        pig_latin_string = first_string.to_owned() + "-" + map.get(first_letter).unwrap();
    } else {
        pig_latin_string = last_letters.to_string() + "-" + first_letter + suffix;
    }
    println!("{}", pig_latin_string);
}
