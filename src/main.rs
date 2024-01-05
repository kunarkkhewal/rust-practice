// use ferris_says::say;
use std::io; //::{BufWriter, stdout};
// use std::fmt::Display;

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
    // let x1 = 5;
    // let x1 = x1 + 1;
    // {
    //     let x1 = x1 * 2;
    //     println!("the value of x1 is {x1}");
    // }
    // println!("the value of x1 is {x1}");

    // let spaces = "   ";
    // let spaces = spaces.len();

    // data types
    //Scaler
    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient: f32 = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // println!("sum: {sum}");
    // println!("difference: {difference}");
    // println!("product: {product}");
    // println!("quotient: {quotient}");
    // println!("truncated: {truncated}");
    // println!("remainder: {remainder}");

    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // println!("t: {t}");
    // println!("f: {f}");

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // println!("c: {c}");
    // println!("z: {z}");
    // println!("heart_eyed_cat: {heart_eyed_cat}");


    // compound
    // tuple
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = ('k', 17, true, 22.0);

    let (x1,y1,z1) = tup1;
    let x2 = tup2.0;

    println!("x1 : {x1}");
    println!("x2 : {x2}");


    // println!("tup1 : {:?}", tup1);
    // println!("tup2 : {tup2}");

    // array
    let a = [1, 2, 3, 4, 5];
    let a1: [u64; 10] = [0,0,0,0,0,0,0,0,0,0];
    let a2 = [3; 5];

    let a0 = a[0];
    println!("a0 = {a0}");


    // array index out of bound 
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}


