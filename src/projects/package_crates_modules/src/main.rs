mod garden;
// use crate::garden;

fn main() {
    let pineapple = garden::vegetable::Vegetable{
        name:String::from("Pineapple"),
        price:12,
        country: String::from("USA")
    };
    println!("The vegetable is: {:?}", pineapple);
    pineapple.plant_vegetable(10);

    garden::current_environment();
}
