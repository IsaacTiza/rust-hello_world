
// fn main(){
//     struct User {
//         username:String,
//         email:String,
//         is_active:bool,
//         number_of_sign:u32
//     }

//     let user1 = User {
//         is_active: false,
//         username: String::from("MaplafTirep"),
//         email: String::from("i2493053@gmail.com"),
//         number_of_sign:9
//     };
//     if user1.is_active{
//         println!("The user {}, with and email of {} is active and have signed in {} times", user1.username,user1.email,user1.number_of_sign)
//     }else{
//         println!("The User {} is not active yet!", user1.username)
//     }

//     let user2 = User{
//         username: String::from("RgSpecie"),
//         email:String::from("rgspecie@gmail.com"),
//         ..user1
//     };
//     println!("{} is now added to the list of users, and he has signed in {} times", user2.username, user2.number_of_sign);
//     println!("{} is now added to the list of users, and he has signed in {} time", user1.username, user1.number_of_sign)
// }

// struct Rectangle {
//     width:u64,
//     length:u64
// }

// fn main(){
//     let rect1 = Rectangle{
//         width: 30,
//         length:50
//     };

//     println!("The calculated area of the rectangle is {} meters!", area(&rect1))
// }
// fn area(rectangle: &Rectangle)->u64{
//     rectangle.width * rectangle.length
// }

