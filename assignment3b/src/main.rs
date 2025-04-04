fn main() {
    println!("Enter the number of students\n");
    let student_num: u16;
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    match input.trim().parse() {
        Ok(n) => {
            student_num = n;
        },
        Err(_) => todo!()
    }
    
    for _i in 0..student_num {
        grade_calculator();
    }

    println!("Happy to help. Bye-bye!");
}

fn grade_calculator() {
    let mut courses: Vec<String> = vec!["".to_string(); 0];
    let mut scores: Vec<u16> = vec![0; 0];
    let mut grades: Vec<&str> = vec![""; 0];
    let  course_num: usize;
    let name:String;

    println!("\nEnter the number of courses");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    match input.trim().parse() {
        Ok(n) => {
            course_num = n;
        },
        Err(_) => todo!()
    }

    println!("\nEnter your name");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    match input.trim().parse() {
        Ok(n) => {
            name = n;
        },
        Err(_) => todo!()
    }

    println! ("\nEnter the codes of your courses");
    for _i in 0..course_num {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let course:String = input.trim().parse().expect("G");
        courses.push(course);
        println!("{:?}", courses);
    }

    println! ("\nEnter your scores for your respective courses");
    for _i in 0..course_num {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let grade = input.trim().parse().expect("Reason");
        scores.push(grade);
        println!("{:?}", scores);
    }

    for i in scores {
        if i >= 70 {
            grades.push("A");
        } else if i >= 60 && i <= 69 {
            grades.push("B"); 
        } else if i >= 55 && i <= 59 {
            grades.push("C");
        } else if i >= 45 && i <= 54 {
            grades.push("D");
        } else if i >= 40 && i <= 45 {
            grades.push("E");
        } else {
            grades.push("F");
        }
    }

    println!("\nDear {}", name);
    println!("These are your grades:\n");

    for i in 0..course_num {
        println!("{} - {}\n", courses[i], grades[i]);
    }

}