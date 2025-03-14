// This function takes two string slices and returns a reference to the longer one
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    } // string2 goes out of scope here

    // This would cause a compile error because string2 doesn't live long enough
    // error[E0425]: cannot find value `string2` in this scope
    // let result = longest(string1.as_str(), string2.as_str());
}
