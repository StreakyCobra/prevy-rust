use ansi_term::Colour::Red;

pub fn heading(text: &str) {
    println!("{}", Red.paint(text));
}
