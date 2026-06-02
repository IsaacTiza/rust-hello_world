//Temprature converter from Celsius to Fahrenheit and vice versa

use std::io;
fn main(){
    println!("TEMPERATURE CONVERTER");
    println!("Enter the temperature you want to convert");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Please input a temperature");
    
    let temperature:f32 = match temperature.trim().parse(){
        Ok(num)=>num,
        Err(_)=> {
            println!("Please input a valid Number");
            return;},
    };
    println!("Enter the unit you want to convert to C/F?");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Enter a unit to convert to");

    let unit = unit.trim().to_uppercase();
    if unit == "C"{
        let result:f32 = fahrenheit_to_celsius(temperature);
        println!("{temperature}°F = {result:.2}°C")
    }else if unit == "F"{
         let result:f32 = celsius_to_fahrenheit(temperature);
        println!("{temperature}°C = {result:.2}°F")
    }else{
        println!("That is not a valid unit to convert to");
        return
    }
    println!("Conversion Done!!")

}

fn celsius_to_fahrenheit(celsius: f32) -> f32{
    celsius * 1.8 + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32{
    (fahrenheit - 32.0) / 1.8
}