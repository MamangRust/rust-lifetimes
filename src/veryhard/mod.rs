fn longest<'a, 'b>(s1: &'a str, s2: &'b str) -> String
where
    'a: 'b,
{
    if s1.len() > s2.len() {
        s1.to_owned()
    } else {
        s2.to_owned()
    }
}

pub fn main_veryhard() {
    let string1 = String::from("Rust");
    let result;
    {
        let string2 = String::from("Programming");
        result = longest(&string1, &string2);
    }
    println!("Longest string: {}", result);
}
