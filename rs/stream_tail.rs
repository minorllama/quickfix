use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let head:&str = args.first().map(|s| s.as_str()).unwrap_or("10");
    if head == "-h" {
        eprintln!("command 2>&1 | stream_tail &");
        return
    };
    
    
    let n:usize = head.parse().unwrap();
    let mut buffer = vec![String::new(); n];
    let mut head = 0;  
    let mut count = n; 

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(e) => {
                buffer[head] = e; // add at head
                head = (head + 1) % n; // circular indexing
                if count > 0 {
                    count -= 1;
                }
            }
            Err(err) => {
                eprintln!("..err: {}", err);
                break;
            }
        }
    }
    
    
    for line in buffer {
        eprintln!("{}", line);
    }
}
