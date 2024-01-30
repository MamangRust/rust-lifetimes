struct Book<'a> {
    title: &'a str,
}

fn print_book_title<'a>(book: &'a Book<'a>) {
    println!("Book title: {}", book.title);
}

pub fn main_hard() {
    let title = String::from("The Rust Programming Language");

    let book = Book { title: &title };

    print_book_title(&book);
}
