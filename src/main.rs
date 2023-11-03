use std::io;
use std::io::Read;
use arboard::Clipboard;
use regex::Regex;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read all input into a single string
    if let Err(e) = handle.read_to_string(&mut input) {
        eprintln!("Error reading from stdin: {}", e);
    }
    let re = Regex::new(r"\\r?\\n|\\r").unwrap();
    input = re.replace_all(&input, "").to_string();

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(input.trim()).unwrap();
    
    // let the_string = "Hello, world!";
	// clipboard.set_text(the_string).unwrap();

}
