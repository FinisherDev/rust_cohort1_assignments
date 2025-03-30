const OPERATORS: [char; 5] = [
    '+', 
    '-',
    '*',
    '/',
    '~'
];


fn main() {
    print!("
    --------The Simple Rust Calculator------------
    | From the creator of the Rust Scientist's   |
    | Tool, a device yet to be worthy of such    |
    | title, here is the pioneer device, a simple|
    | calculator.                                |
    | -------------------------------------------|
    | For the user, this calculator does basic   |
    | arithemetic, and any other thing I hope I  |
    | have in mind. For more info, type in 'help'|
    ----------------------------------------------
    \n
    ");
    println!("With that said...\n");

    loop {
        println!("Enter the first number (or ask for help)");
        let num_1 = input_validator();

        println!("Enter the operation");
        let operation = operator();

        println!("Enter the second number");
        let num_2 = input_validator();

        let result = operation_handler(operation, num_1, num_2);
        print!("This is the result of the operation: {}", result);

        println!("Would you like to go on? (y/n)");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        match input.trim().parse::<String>() {
            Ok(ans) => {
                if ans == "n" {
                    break
                } else {
                    continue
                }
            }
        }
    }
}

fn input_validator() -> (String, f32){
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    match input.trim().parse::<String>() {
        Ok(input) => {
            if input == "help" {
                print!("
                -------------------------------
                | This device works by taking |
                | a first number, the symbol  |
                | of your choosing from our   |
                | list of symbols and a second|
                | number. Then it carries out |
                | the desired operation,      |
                | except in situations like   |
                | this and others specified.  |
                -------------------------------
                | + Addition Operator         |
                | - Subtraction Operator      |
                | / Division Operator         |
                | * Multiplication Operator   |
                | ~ Square Operator. Only the |
                |   first number is squared.  |
                |   A second number must be   |
                |   entered, but it is not    |
                |   used.                     |
                -------------------------------
                | 'cgpa' Starts cgpa          |
                |        calculator.Useful    |
                |        for students.        |
                |        (Coming Soon)        |
                | 'help' Displays this page   |
                -------------------------------
                ");
                println!("\nEnter the first number(or what ever other operation)");
                input_validator();
                return ("".to_string(), f32::NAN);
            } else if input == "cgpa" {
                println!("Preparing calculator...");
                return ("".to_string(), f32::NAN);
            } else {
                let num:f32 = input.parse().expect("This one no dey");
                return ("".to_string(), num);
            }
        }
    }
}

fn operator() -> char{
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let _ = input.trim();
    let value = input.chars().next().expect("Empty");
    if OPERATORS.contains(&value) {
        return value;
    } else {
        println!("Not an operator");
        let a = operator();
        return a;
    }
}

fn operation_handler(op: char, num1:(String, f32), num2:(String, f32)) -> f32{
    match op {
        '+' => return num1.1 + num2.1,
        '-' => return num1.1 - num2.1,
        '*' => return num1.1 * num2.1,
        '/' => return num1.1 / num2.1,
        '~' => return num1.1 * num1.1,
        _ => todo!(),
    }
}

