// Rust's pattern matching is powerful

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: f64, y: f64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("PageLoading..."),
        WebEvent::PageUnload => println!("PageUnloading..."),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

fn main() {
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);
    inspect(WebEvent::KeyPress('a'));
    inspect(WebEvent::Paste(String::from("I m rust !")));
    inspect(WebEvent::Click { x: 34.23, y: 45.43 });
}
