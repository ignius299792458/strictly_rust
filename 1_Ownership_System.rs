fn main(){

    // Variable binding creates ownership
    let s1 = String::from("Hellow");

    // ownership moves to the function
    takes_ownership(s1);

    // This would cause a compile error - s1 was moved
    // println!("{}", s1);

     // Primitive types implement Copy trait, so they're copied instead of moved
     let x = 5;
     makes_copy(x);
     println!("{}", x); // This works fine
}

fn takes_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped

fn makes_copy(i: i32) {
    println!("{}", i);
} // i goes out of scope but nothing special happens