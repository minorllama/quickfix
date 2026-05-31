// #![feature(explicit_tail_calls)] // only works on nightly (and not stable) rustc

use std::io::{self, BufRead};
use std::env;

/* trying out `become` and tco in rust
// uses `become` for tail call optimization
#[expect(incomplete_features)]
fn log_buffer(buf:&[String], size:usize, idx:usize, depth:usize){
    if depth < size {
        eprintln!("{}", buf[idx]);
        become log_buffer(buf, size, (idx+1)%size, depth+1);
    }
}
*/

// loop version; keep the same signature as tco version
fn log_buffer(buf:&[String], size:usize, mut idx:usize, mut depth:usize){
    loop {
        if depth >= size {
            break;
        } else {
            idx = idx % size;
            eprintln!("{}", buf[idx]);
            idx += 1;
            depth += 1;
        } 
    }

}

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
    let mut count = 0; 

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(e) => {
                buffer[head] = e; // add at head
                head = (head + 1) % n; // circular indexing
                if count < n {
                    count += 1;
                }
            }
            Err(err) => {
                eprintln!("..err: {}", err);
                break;
            }
        }
    }
    
    
    // assuming there were at least n element
    // since head was incremented, it's pointing to start of the n to last element.
    // if less than n elements were in the stream, the idexing is mod count
    if count == n {
        log_buffer(&buffer, n, head % n, 0)
    } else {
        log_buffer(&buffer, count, head % count, 0)
    }
    /* 
    $ seq 2 | ./stream_tail 3
      1
      2
    $ seq 100 | ./stream_tail 3
      98
      99
      100
    */
}
