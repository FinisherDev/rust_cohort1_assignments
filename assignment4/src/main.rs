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
        fn end_function() -> String{
            println!("\nWould you like to go on? (y/n)");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            match input.trim().parse::<String>() {
                Ok(ans) => {
                   return ans;
                }
            }
        }

        println!("Enter the first number (or ask for help)");
        let num_1 = input_validator();


        println!("Enter the operation");
        let operation = operator();

        println!("Enter the second number");
        let num_2 = input_validator();

        let result = operation_handler(operation, num_1, num_2);
        print!("This is the result of the operation: {}", result);

        let answer = end_function();
        if answer == "n" {
            break;
        } else {
            continue;
        }
    }
}

fn input_validator() -> (&'static str, f32){
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
                | 'cgpa' Starts GPA calculator|
                |        Useful for students. |
                | 'help' Displays this page   |
                -------------------------------
                ");
                println!("\nEnter the first number(or what ever other operation)");
                input_validator();
                return ("", f32::NAN);
            } else if input == "cgpa" {
                println!("Preparing calculator...");
                gpa_calculator();
                return  ("cgpa", f32::NAN);
            } else {
                let num:f32 = input.parse().expect("This one no dey");
                return ("", num);
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

fn operation_handler(op: char, num1:(&str, f32), num2:(&str, f32)) -> f32{
    match op {
        '+' => return num1.1 + num2.1,
        '-' => return num1.1 - num2.1,
        '*' => return num1.1 * num2.1,
        '/' => return num1.1 / num2.1,
        '~' => return num1.1 * num1.1,
        _ => todo!(),
    }
}

fn gpa_calculator() {
    let mut total_credits: u16 = 0;
    let mut course_point_avg: u16 = 0;
    let mut credits: Vec<u16> = vec![0; 0];
    let mut grades: Vec<char> = vec!['.'; 0];
    let mut grade_points: Vec<u16> = vec![0; 0];
    let  course_num: u16;
    let  gpa:f32;

    println!("\nEnter the number of courses");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    match input.trim().parse() {
        Ok(n) => {
            course_num = n;
        },
        Err(_) => todo!()
    }

    println! ("\nEnter the credit loads of your courses");
    for _i in 0..course_num {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let credit:u16 = input.trim().parse().expect("G");
        credits.push(credit);
        println!("{:?}", credits);
    }

    println! ("\nEnter your grades using capital letters");
    for _i in 0..course_num {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let x = input.trim();
        let grade = x.chars().next().expect("Empty");
        grades.push(grade);
        println!("{:?}", grades);
    }

    for i in grades {
        match i {
            'A'=> grade_points.push(5),
            'B' => grade_points.push(4),
            'C' => grade_points.push(3),
            'D' => grade_points.push(2),
            'E' => grade_points.push(1),
            'F' => grade_points.push(0),
            _ => todo!(),
        }
    }

    for i in 0..grade_points.len() {
        total_credits += credits[i];
        course_point_avg += grade_points[i] * credits[i];
    }

    gpa = course_point_avg as f32 / total_credits as f32;
    println!("\n Your Grade Point Average  {}", gpa);
}