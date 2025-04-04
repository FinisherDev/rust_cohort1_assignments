const ITEMS: [&str; 12] = [
    "a patridge in a pear tree.", 
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "FIVE GOLDEN RINGS!!!",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking", 
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming"
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
    "Nineth",
    "Tenth",
    "Eleventh",
    "Tewlfth"
];

fn main() {
    print!("
    *********************************
      The Twelve Days of Christmas. 
    *********************************
    \n
    ");
    for i in 1..13 {
        let items = &mut ITEMS[0..i];
        items.reverse();
        println!("On the {} day of Christmas", DAYS[i-1]);
        println!("My true love gave to me");
        for item in items {
            println!("{}", item);
        }
        println!("\n");
    }
}
