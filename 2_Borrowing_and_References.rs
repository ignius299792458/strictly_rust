fn main() {
    let s1 = String::from("hello");
    
    // Immutable borrow
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable borrow (note: you can only have one mutable borrow at a time)
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2); // Prints "hello, world"
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't have ownership so nothing happens

fn change(s: &mut String) {
    s.push_str(", world");
}