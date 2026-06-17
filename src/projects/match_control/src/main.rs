// enum Coins{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState)
// }

// #[derive(Debug)]
// enum UsState{
//     Alabama,
//     Arizona,
//     Alaska,
// }

// fn main() {

//     let returned = value_in_cents(Coins::Quarter(UsState::Alaska));
//     println!("The returned value from the function {}", returned)

// }
// fn value_in_cents(coin:Coins)->u8{
//     match coin {
//         Coins::Penny => {
//             println!("Lucky Penny!!");
//             70
//         }
//         Coins::Nickel=> 5,
//         Coins::Dime=> 10,
//         Coins::Quarter(state)=>{
//             println!("State Quarter from {state:?}!");
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>)  -> Option<i32>{
//     match x {
//         None => None,
//         Some(i) => Some(1 + i),
//     }
// }

fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let one = plus_one(None);

let config_max = Some(32u8);
// match config_max{
//     Some(max)=> println!("The Maximum is consided to be {max}"),
//     _=>()
// }

if let Some(max) = config_max{
    println!("The maximum configuration is {max}")
}
}
