// fn main() {
//     let s1 = String::from("Hello! ");
//     let len = cal_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }
// fn cal_length(s: &String) -> usize{
//     s.len()
// }

// fn main() {
//     let string = String::from("Discussing the world War");
//     let first_word = first_word(&string);
//     println!("The first word in the string is: {}", first_word)
// }
// fn first_word(s: &String) -> &str {
//     let byte = s.as_bytes();
//     // println!("{}", byte);
//     for (i, &item) in byte.iter().enumerate(){
//         if item == b' '{
//             return &s[..i]
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..3];
//     assert_eq!(slice,&[2,3])
// }
