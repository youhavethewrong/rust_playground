fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    println!("{}", no_dangle());

    let sentence = String::from("have a taco");
    println!(
        "Does the first word in '{}' end at index {}?",
        sentence,
        first_word_index(&sentence)
    );

    println!(
        "Did you know that '{}' is the first word in '{}'?",
        first_word(&sentence[..]),
        sentence
    );
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("Oh, hello.");
    s
}

fn first_word_index(s: &String) -> usize {
    // This does not work for multi-byte encodings.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // b'x' is byte literal syntax
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
