// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I T DONE

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    // Explanation: Specify the type of elements in the Vec as &str
    shopping_list.push("milk");
}

