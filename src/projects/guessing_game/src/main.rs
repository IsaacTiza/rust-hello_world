
// GUESSING GAME
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// fn main() {
//     println!("Guessing Game");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     loop {
//         println!("Input your guess");
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read guess");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You Guessed {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too Small!"),
//             Ordering::Greater => println!("Too Big!"),
//             Ordering::Equal => {
//                 println!("Congratulations!! You Win!! ");
//                 break;
//             }
//         }
//     }
// }



// fn main(){
//      let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main(){
//     let x = 5;
//     let x = x +5;
//     {
//         let x = x+5;
//         println!("The value of x in this scope {x}")
//     }
//     println!("The calue of x in the outer scope {x}")
// }

// fn main(){
//     another_function(5,'h');
//     let x = five();
//     println!("The value of x is : {x}")
// ;}
// fn another_function(x: i32, unit_label:char){
//     println!("The Value of x is :{x}{unit_label}");
// }
// fn five() -> i32{
//     5
// }

fn main(){
    let age = 18;
    if age < 18{
        println!("You are not an Adult!")
    } else {
        println!("You are an Adult!")
    }
}