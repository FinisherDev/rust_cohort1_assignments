fn main() {
    /*
    -----------------------------------------
    Assignment 1
    A redo on the first assignment as the 
    first version was mistakenly deleted
    in a bid to push to the main branch.
    
    This program is designed to print the 
    creator's:
    * name
    * matriculation number
    * favourite emoji
    * a float division
    
    using statements.
    -----------------------------------------
    */
    println!("Name: Molokwu Chukwufumnanya"); //My name
    println!("Matriculation Number: ENG2002472"); //My Matriculation Number
    println!("\n\u{1f601}"); //My favourite emoji
    println!("A float division between 22 and 7: {}", 22.0/7.0);

    /*
    -----------------------------------------
    Assignment 2
    A redo on the second assignment as the 
    first version was mistakenly deleted
    in a bid to push to the main branch.
    
    This program is designed to print the 
    creator's:
    * name
    * matriculation number
    * favourite emoji
    * a float division
    
    using functions.
    -----------------------------------------
    */    
    name();
    mat_no();
    fav_emoji();
    float_div(22.0, 7.0);
}

fn name() {
    println!("Name: Molokwu Chukwufumnanya"); //Creator's name
}

fn mat_no() {
    println!("Matriculation Number: ENG2002472"); //Creator's Matriculation Number
}

fn fav_emoji() {
    println!("\n\u{1f601}"); //Creator's Favourite Emoji
}

fn float_div(num1:f32, num2:f32) {
    println!("A float division: {}", num1/num2); //A division that forever changed the life of man
}