pub fn main(args: &[String]) {
    for animal in args {
        match animal.to_lowercase().as_str() {
            "dog" => println!("Bark"),
            "duck" => println!("Quack"),
            _ => println!("All quiet out here")
        }
    }
}
