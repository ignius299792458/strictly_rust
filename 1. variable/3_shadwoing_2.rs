
// Remove a line in the code to make it compile
fn main() {
    let x: i32 = 1;
    println!("unshadowed x: {}", x);
    // x = 7;
    // Shadowing and re-binding
    let mut x = x; 
    x += 3;
    println!("shadowed to x: {}", x);


    let y = 4;
    println!("unshadowed y: {}", y);
    // Shadowing
    let y = "I can also be bound to text!"; 
    println!("shadowed to y: {}", y);


    println!("Success!");
}