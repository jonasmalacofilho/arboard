extern crate clipboard;

use clipboard::ClipboardProvider;
#[cfg(target_os = "linux")]
use clipboard::x11_clipboard::{X11ClipboardContext};

#[cfg(target_os = "linux")]
fn main() {
    let mut ctx: X11ClipboardContext = ClipboardProvider::new().unwrap();

    let the_string = "Hello, world!";

    ctx.set_text(the_string.to_owned()).unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Primary selection is only available under linux!");
}
