const ITEMS: [&str; 12] = [
    "a patridge in a pear tree.", 
    "two turtle doves and",
    "three french hens,",
    "four calling birds,",
    "FIVE GOLDEN RINGS!!!",
    "six geese a-laying,",
    "seven swans a-swimming,",
    "eight maids a-milking,", 
    "nine ladies dancing,",
    "ten lords a-leaping,",
    "eleven pipers piping,",
    "twelve drummers drumming,"
];

const DAYS: [&str; 12] = [
    "First", 
    "Second", 
    "Third",
    "Fourth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eighth",
    "Ninth",
    "Tenth",
    "Eleventh",
    "Twelfth"
];

fn main() {
    println!("
    *********************************
      The Twelve Days of Christmas. 
    *********************************
    ");
    for i in 1..13 {
        let mut items: Vec<&str> = ITEMS[0..i].to_vec();
        items.reverse();
        println!("On the {} day of Christmas", DAYS[i-1]);
        println!("My true love gave to me");
        for item in items {
            println!("{}", item);
        }
        println!("\n");
    }
}
