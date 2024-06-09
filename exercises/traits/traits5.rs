// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    // Explanation: Use impl SomeTrait + OtherTrait to specify that item must implement both traits.
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func_with_some_struct() {
        assert!(some_func(SomeStruct {}));
    }

    #[test]
    fn test_some_func_with_other_struct() {
        assert!(some_func(OtherStruct {}));
    }
}

