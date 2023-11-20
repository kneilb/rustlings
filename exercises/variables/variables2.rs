// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    // overflow detected (at least in debug mode)
    // let x: u8 = 1000;
    let x: u32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
