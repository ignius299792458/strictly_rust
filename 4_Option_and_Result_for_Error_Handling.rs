fn main() {
    // Option<T> is used instead of null
    let username = get_username(1);
    // method 1: to handle Option<T> return
    // match username {
    //     Some(name) => println!("Found user: {name}"),
    //     None => println!("No user found"),
    // }

    // method 2: to handle Option<T> return
    // If let is a cleaner way to handle a single pattern
    if let Some(name) = username {
        println!("User name: {}", name);
    }

    // Result<T, E> is used for error handling
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // The ? operator can be used to propagate errors (in functions that return Result)
    let result = calculate().unwrap_or_else(|err| {
        println!("Calculation error: {}", err);
        0.0
    });
    println!("calculate result: {}", result);
}

fn get_username(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("alice"))
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn calculate() -> Result<f64, String> {
    let a = divide(10.0, 2.0)?; // If this returns Err, the function returns early
    let b = divide(a, 2.0)?;
    Ok(b)
}
