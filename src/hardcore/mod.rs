trait Printer<'a> {
    fn print(&self, data: &'a str);
}

struct DisplayPrinter;

impl<'a> Printer<'a> for DisplayPrinter {
    fn print(&self, data: &'a str) {
        println!("Display Printer: {}", data);
    }
}

pub fn main_hard_core() {
    let printer = DisplayPrinter;

    let data = String::from("Hardcore example");

    printer.print(&data)
}
