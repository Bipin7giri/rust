// fn main() {
//     let mut x = 4;
//     println!("x is:{}",x);
//     x = 5;
//     println!("x is:{}",x);
// }

// fn main() {
//     let mut x: i32 = 4;
//     println!("x is:{}", x);
//     {
//         let x = x-2;
//         println!("x is: {}", x);
//     }
//     x = x+1;
//     println!("x is:{}", x);
// }
// fn main() {
//     let x = 4;
//     println!("x is:{}", x);
//     let x = "hello";
//     println!("x is:{}", x);
// }

// constant
// fn main() {
//     const SECONDS_IN_MINUETS:u32 = 60;
//     println!("{}", SECONDS_IN_MINUETS);
// }

// datatype
// primitive data type
//  scalar || compound

// scalar datatype
// fn main() {
//     let x: i32 = 2;
//     let y = 10.4;
//     // boolean
//     let isActive: bool = true;
//     // char
//     let letter: char = '8';
// }

// compound type
// tuple
// fn main() {
//     let mut tup: (i32, bool, char) = (1, true, 's');
//     tup.0 = 20;
//     println!("{}",tup.0);
// }
// array
// fn main(){
//     let mut arr = [1,23,5,6,78,9];
//     arr[0]= 22;
//      println!("{}",arr[0]);
// }

// getting input from the console
// use std::io;
// fn main() {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//     println!("{}", input);
// }

// type casting
// use as like as ts
use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expect to read line");
    let int_input:i64 = input.trim().parse().unwrap();
    println!("{}",int_input)
}