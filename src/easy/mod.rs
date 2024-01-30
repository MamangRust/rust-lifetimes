fn concatenate_strings<'a>(s1: &'a str, s2: &'a str) -> String {
    format!("{}{}", s1, s2)
}

pub fn main_easy() {
    let greeting = "Hello, ";
    let name = "Rust";

    let result = concatenate_strings(greeting, name);

    println!("{}", result);
}
