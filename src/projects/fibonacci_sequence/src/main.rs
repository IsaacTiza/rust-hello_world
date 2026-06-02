
use std::io;

fn main() {
    println!("===GENERATE THE FIBONACCI NUMBER===");
println!("Enter a term (0–93): ");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read a term!");
    let term: u64 = match term.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("No Valid Term Entered!");
            return
        }
    };
    if term > 93{
        println!("Term should be less than 93!");
        return
    }
    let fibonacci_number = generate_fibonacci_number(term);
    println!("The {}th Fibonacci number is: {}", term, fibonacci_number);
}
// VERY INEFFICIENT FUNCTION, IT TAKES A LONG TIME TO COMPUTE THE FIBONACCI NUMBER FOR LARGE TERMS
//  fn generate_fibonacci_number(term: u64) -> u64{
//     match term{
//         0 => 0,
//         1 => 1,
//         _ => generate_fibonacci_number(term - 1) + generate_fibonacci_number(term - 2),
//     }
// }



// EFFICIENT WAY TO GENRATE THE FIBONACCI NUMBER USING BINET'S FORMULA, IT CAN COMPUTE THE FIBONACCI NUMBER FOR LARGE TERMS INSTANTLY
fn generate_fibonacci_number(term: u64)->u64{
    match term{
        0=>0,
        1=>1,
        _=>binet(term)
    }
}
fn binet(n:u64)->u64{
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let psi = (1.0 - 5.0_f64.sqrt()) / 2.0;
    ((phi.powf(n as f64) - psi.powf(n as f64)) / 5.0_f64.sqrt()).round() as u64
}