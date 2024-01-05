// use ferris_says::say;
// use std::io::{BufWriter, stdout};

fn main() {
    // Ferris crate
    // let stdout = stdout();
    // let message = String::from("Hello World!!! this is Kunark");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(&message, width, &mut writer).unwrap();

    // UNDERSTANDING_VARIABLES
    // mutability
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // shadowing
    let x1 = 5;
    let x1 = x1 + 1;
    {
        let x1 = x1 * 2;
        println!("the value of x1 is {x1}");
    }
    println!("the value of x1 is {x1}");

    let spaces = "   ";
    let spaces = spaces.len();

}


