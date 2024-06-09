// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3); // A solution is to add an argument of type u32 to the call or we can redefine the
                // function and change num with a const and remove the argument in the declaration
    
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
