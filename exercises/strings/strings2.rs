// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

// after some research i found that String deferencies str so that in the Standard Type(struct )
// declaration they included this and that's what makes the conversion possible.
// the main difference is that String is Mutable and allows ownership cintrary to str where we are
// only allowed to read in this case &word is a slice of word the original String declared.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
