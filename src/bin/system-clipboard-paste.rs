use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let contents = clipboard.get_text().unwrap();
    print!("{contents}");
}
