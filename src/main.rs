// use ferris_says::say;
// use std::io; //::{BufWriter, stdout};
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


    // // compound
    // // tuple
    // let tup1: (i32, f64, u8) = (500, 6.4, 1);
    // let tup2 = ('k', 17, true, 22.0);

    // let (x1,y1,z1) = tup1;
    // let x2 = tup2.0;

    // println!("x1 : {x1}");
    // println!("x2 : {x2}");


    // // println!("tup1 : {:?}", tup1);
    // // println!("tup2 : {tup2}");

    // // array
    // let a = [1, 2, 3, 4, 5];
    // let a1: [u64; 10] = [0,0,0,0,0,0,0,0,0,0];
    // let a2 = [3; 5];

    // let a0 = a[0];
    // println!("a0 = {a0}");


    // // array index out of bound 
    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");
    // let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    // FUNCTIONS
    // let some_value= another_function(5, 5);
    // println!("some_value : {some_value}")

    // CONTROL FLOW
    // let num = 7;
    // if num != 7 {
    //     println!("This is if")
    // // } else {
    // //     println!("This is else")
    // }

    // let cond = false;
    // let num2 = if cond { 5 } else { 6 };
    // println!("num2 : {num2}");

    // loop {
    //     println!("this is loop")
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2 ;
    //     }
    // };

    // println!("result : {result}")


    // let mut count = 0;

    // 'counter_loop: loop {
    //     println!("count => {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining => {remaining}");
    //         if remaining < 9 {
    //             break;
    //         }
    //         if (count > 2) {
    //             break 'counter_loop; 
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }

    // let mut counter = 10;

    // while counter >= 0 {
    //     println!("counter: {counter}");
    //     counter -= 1;
    // }
    // println!("Liftoff!!!")

    // let a = [10, 20, 30, 40];
    // for element in a {
    //     println!("Element: {element}")
    // }

    // for number in (0..10).rev() { // rev() is for reverse
    //     println!("Counter: {number}")
    // }

    // OWNERSHIP
    // {
    //     let y: i32;
    //     {
    //         let x = 5;
    //         println!("x = {}", x);
    //         y = x;
    //         println!("x = {}", x);
    //     }
    //     println!("y = {}", y);
    // }
    


    // {
    //     let s2: String;
    //     {
    //         let s1 = String::from("hello");
    //         println!("s1 = {}", s1);
    //         s2= s1;
    //         // println!("s1 = {}", s1);
    //     }
    
    //     println!("s2 = {}", s2);
    // }

    // let s = String::from("hello");
    // println!("up {}", s);
    // takes_ownership(s);
    // // println!("dn {}", s);

    // let x = 5;
    // println!("up {}", x);
    // makes_copy(x); 
    // println!("dn {}", x);

    // REFERENCES
    // let mut s1 = String::from("Hello");
    // let len = calculate_length(&mut s1);
    // println!("s1 : {}, len : {}", s1, len);

    // let r1 = &mut s1;
    // println!("r1 : {}", r1);
    // let r2 = &mut s1;
    // println!("r2 : {}", r2);
    
    // println!("r1 : {}, r2 : {}", r1, r2);

    // SLICES
    // let s1 = String::from("HelloWorld");
    // let word = first_word(&s1);
    // println!("word: {}", word)

    // STRUCT
    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64
    // }

    // let user1 = User {
    //     active: true,
    //     username: String::from("kunarkkhewal"),
    //     email: String::from("kunark@email.com"),
    //     sign_in_count: 1,
    // };

    // // user1.username = String::from("KK");
    // println!("username : {}", user1.username);

    // let mut user2 = User {
    //     active: true,
    //     username: String::from("kunarkkhewal2"),
    //     email: String::from("kunark2@email.com"),
    //     sign_in_count: 2,
    // };

    // println!("Usernaem2: {}", user2.username);
    // user2.username = String::from("KK");
    // println!("Usernaem2: {}", user2.username);


    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         email,
    //         username,
    //         sign_in_count: 3
    //     }
    // }

    // let mut user3 = build_user(String::from("email3"), String::from("username3"));
    // println!("Username3: {}", user3.username);
    // user3.username = String::from("user3");
    // println!("Username3: {}", user3.username);

    // let user4 = User {
    //     email: String::from("email4"),
    //     ..user1
    // };
    // println!("user4 email: {}", user4.email);
    // println!("user4 username: {}", user4.username);
    // println!("user4 active: {}", user4.active);


    // println!("user1 email: {}", user1.email);
    // // println!("user1 username: {}", user1.username);
    // println!("user1 active: {}", user1.active);

    // STRUCT EXAMPLE

    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32
    // }

    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }

    //     fn can_hold(&self, rectangle: &Rectangle) -> bool {
    //         self.width > rectangle.width && self.height > rectangle.height
    //     }

    //     fn square(size: u32) -> Self {
    //         Self {
    //             width: size,
    //             height: size,
    //         }
    //     }
    // }

    // // let scale = 2;
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 15,
    //     height: 25,
    // };
    // let rect3 = Rectangle {
    //     width: 45,
    //     height: 75,
    // };

    // // let rect2 = dbg!(rect1);

    // println!("Can Rect1 hold Rect2 : {}", rect1.can_hold(&rect2));
    // println!("Can Rect1 hold Rect3 : {}", rect1.can_hold(&rect3));
    
    // println!("Rect4 squared : {:?}", Rectangle::square(30));

    // ENUM
    // #[derive(Debug)]
    // enum IpAddrKind {
    //     Ipv4(String),
    //     Ipv6(String)
    // }
    // // let four = IpAddrKind::Ipv4;
    // // let six = IpAddrKind::Ipv6;
    // // println!("four : {:?}, six: {:?}", four, six)

    // let home = IpAddrKind::Ipv4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::Ipv6(String::from("::1"));
    // println!("Home : {:?}, Loopback : {:?}", home, loopback);


    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(u8,u8,u8,u8),
    //     V6(String)
    // }
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    // println!("Home : {:?}, Loopback : {:?}", home, loopback);

    // #[derive(Debug)]
    // enum Message {
    //     Quit,
    //     Move {x: u32, y: u32},
    //     Write(String),
    //     ChangeColor(i32, i32, i32)
    // }

    // impl Message {
    //     fn call(&self) {
    //         println!("Self : {:?}", self)
    //     }
    // }

    // let m = Message::Write(String::from("Kunark Khewal"));
    // m.call();

    // let some_number = Some(5);
    // let some_char = Some('a');

    // let absent_number: Option<i32> = None;

    // println!("some_number : {:?}, some_char: {:?}, absent_number : {:?}", some_number, some_char, absent_number)
    

    #[derive(Debug)]
    enum UsState {
        Alabama, Alaska, California, Florida
    }
    enum Coin {
        Penny, 
        Nickel, 
        Dime, 
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State: {:?}, Quarter", state);
                25
            },
        }
    }

    println!("the value of coin is: {}", value_in_cents(Coin::Penny));
    println!("the value of coin is: {}", value_in_cents(Coin::Quarter(UsState::Florida)));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five);
    println!("Six: {:?}", six);
    println!("none: {:?}", none);


    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        _ => ()
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max: {}", max);
    }


}

// fn another_function(x: i32, y: i32)-> i32 {
//     println!("val of x & y : {x}, {y}");
//     x + y
// }

// fn takes_ownership(some_string: String) { 
//     println!("fn {}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("fn {}", some_integer);
// }

// fn calculate_length (s: &mut String) -> usize {
//     s.push_str(" world");
//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

