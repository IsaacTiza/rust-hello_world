fn main() {
    println!("CHRISTMAS SONG");
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts: [&str; 12] = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese-a-Laying",
        "seven Swans-a-Swimming",
        "eight Maids-a-Milking",
        "nine Ladies Dancing",
        "ten Lords-a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for i in 0..days.len() {
        println!("On the {} day of Christmas, my true love gave to me", days[i]);
        for j in (0..=i).rev(){
            println!("{}", gifts[j]);
        }
        println!();
    }
}
