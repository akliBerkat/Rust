// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        // Explanation: Use a reference to y to avoid consuming it, so it can still be used later.
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Explanation: y can still be used here because we matched on a reference to y.
}

