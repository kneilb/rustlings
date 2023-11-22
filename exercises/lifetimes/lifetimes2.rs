// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // Commented this out and moved the let onto the start of the assignment below
    // let result;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // Moved next line up into this scope.
        println!("The longest string is '{}'", result);
    }
}
