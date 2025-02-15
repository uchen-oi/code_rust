struct StringManagerOwned {
    strings: Vec<String>,
}
impl StringManagerOwned {
    fn new() -> Self {
        StringManagerOwned { strings: Vec::new() }
    }
    fn add(&mut self, string: String) {
        self.strings.push(string);
    }
    fn get_longest(&self) -> Option<&String> {
        self.strings.iter().max_by_key(|s| s.len())
    }
}
fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from("Ownership");
    let s3 = String::from("Borrowing");
    let s4 = String::from("Lifetimes");
    let mut manager = StringManagerOwned::new();
    manager.add(s1);
    manager.add(s2);
    manager.add(s3);
    manager.add(s4);
    if let Some(longest) = manager.get_longest() {
        println!("The longest string is: {}", longest);
    } else {
        println!("No strings to compare.");
    }
}
