// Fix the error with the use of define_x
let x: string mut = "hey";
fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}