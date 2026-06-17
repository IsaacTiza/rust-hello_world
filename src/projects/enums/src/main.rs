enum Message {
    Quit,
    Write(String),
    Move{x:i32,y:i32},
    ChangeColor(i32,i32,i32)
}
impl Message{
    fn call(&self){
        let text = self;
        println!("Printing self to the terminal {}", text)
    }
}

fn main(){
    let m = Message::Write(String::from("Hello World!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
 enum Option<T>{
    None,
    Some(T)
 }