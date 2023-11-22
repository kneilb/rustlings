// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// Ordering is important - this has to be defined first!!
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
