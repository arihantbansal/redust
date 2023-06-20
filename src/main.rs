use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 512];
                loop {
                    let bytes_read = _stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        println!("client closed the connection");
                        break;
                    }

                    let res = format_response("PONG");
                    _stream.write(res.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

fn format_response(res: &str) -> String {
    format!("+{}\r\n", res)
}
