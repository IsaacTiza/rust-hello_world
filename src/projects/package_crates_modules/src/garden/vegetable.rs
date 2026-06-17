#[derive(Debug)]
pub struct Vegetable {
   pub name: String,
    pub price: i32,
    pub country: String,
}

impl Vegetable {
    pub fn plant_vegetable(&self, count: u32) {
        println!("You have planted {} {} in {}", count, self.name, self.country)
    }
}
