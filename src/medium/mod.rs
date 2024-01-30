struct Container<'a> {
    data: &'a str,
}

impl<'a> Container<'a> {
    fn new(data: &'a str) -> Container<'a> {
        Container { data }
    }

    fn get_data(&self) -> &'a str {
        self.data
    }
}

pub fn main_medium() {
    let input_data = String::from("Medium Example");
    let container = Container::new(&input_data);

    println!("Data is container: {}", container.get_data())
}
