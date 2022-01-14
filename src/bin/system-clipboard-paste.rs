use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = ctx.get_contents().unwrap();
    print!("{contents}");
}
