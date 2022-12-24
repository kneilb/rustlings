// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ( $( $x: expr ),* ) => {
        println!("What you do??!?");
    };
}

fn main() {
    my_macro!();
    my_macro!("fish", "fish", "fish");
}
