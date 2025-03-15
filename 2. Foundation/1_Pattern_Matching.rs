fn main() {
    let number = 1;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),
    }

    // multiple-value matching
    // match ranges using ..= (inclusive) or .. (exclusive):
    match number {
        1 | 2 => println!("One or two"),
        3..=5 => println!("Three to five"),
        _ => println!("Something else"),
    }


    // percentage_to_grade
    println!("85 is grade: {}", percentage_to_grade(85));
    println!("92 is grade: {}", percentage_to_grade(92));
    println!("75 is grade: {}", percentage_to_grade(75));
}

fn percentage_to_grade(score: u8)-> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'X', // Invalid score
    }
}