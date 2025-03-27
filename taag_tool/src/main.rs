use figleter::FIGfont;

// let small_font = FIGfont::from_file("resources/small.flf").unwrap();
// let figure = small_font.convert("FIGlet");
// assert!(figure.is_some());

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("FIGlet");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}