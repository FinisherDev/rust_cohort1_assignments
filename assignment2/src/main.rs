fn main() {
    let scores: [u16; 10] = [50, 63, 76, 42, 86, 94, 53, 41, 32, 21];
    let students: [&str; 10] = ["Jack", "Damian", "Andrea", "Melvin", "Maxwell", "Tracy", "Justin", "Andy", "Kate", "Hannah"];

    let total = total(scores);
    println!("The total class score is {}\n", total);

    avg(scores, total);

    let mode = mode(scores);
    println!("The highest score is {}.\n", mode);

    faliure_seperator(scores, students);
}

fn total (scores:[u16; 10]) -> u16{
    let mut sum: u16 = 0;
    let mut i = 0;
    let length = scores.len();
    
    while i < length {
        sum += scores[i];
        i += 1;
    }

    return sum;
}

fn avg (scores: [u16; 10], total:u16) {
    let length = scores.len();

    let result = total as f32/length as f32;
    println!("\nThe class average is {}\n", result);
}

fn mode(scores: [u16; 10]) -> u16{
    let mut mode = scores[0];

    for i in scores {
        if i > mode{
            mode = i;
        } 
    }

    return mode;
}

fn faliure_seperator(scores: [u16; 10], students: [&str; 10]) {
    let mut fail_count = 0;

    for i in 0..scores.len() {
        if scores[i] < 50 {
            fail_count += 1;
        }
    }

    println!("The number of faliures is {}\n", fail_count);

    for i in 0..students.len() {
        if scores[i] >= 50 {
            println!("{} passed with {} points.\n", students[i], scores[i]);
        } else {
            println!("{} failed with {} points.\n", students[i], scores[i]);
        }
    }
}