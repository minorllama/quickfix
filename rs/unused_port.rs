use std::net::TcpListener;

fn find_free_port() -> Result<u16, std::io::Error> {
    // bind to local address with port 0 to get assigned port from OS
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    // the listener is dropped and the port is freed
    Ok(port)
}

fn main() -> Result<(), std::io::Error> {
    match find_free_port() {
        Ok(unused_port) => {
          println!("{}", unused_port);
          Ok(())
        }
        Err(e) => {
          eprintln!("error finding a free port: {}", e);
          Err(e)
        }
    }
}
