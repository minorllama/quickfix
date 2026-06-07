use std::env;
use std::thread;
use std::time::Duration;

use arboard::Clipboard;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Some(clipped) = args.get(0)  {
        let mut clipboard = Clipboard::new().expect("clipboard unavailable");
        match clipboard.set_text(clipped.to_string()) {
            Ok(_) => println!(" ..clipped[\"{}...\"]", &clipped[0..5]),
            Err(e) => eprintln!(" ..fail[{}]", e),
        }
        eprintln!(" ..keep_alive[500ms]"); // so clipboard managers can grab text
        thread::sleep(Duration::from_millis(500));
    } else {
        eprintln!(" ...");       
    }

}
