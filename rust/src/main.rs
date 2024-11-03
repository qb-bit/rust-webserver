use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if let Ok(request) = std::str::from_utf8(&buffer) {
        let path = extract_path(request);
        
        let response = match read_file(&path) {
            Ok(contents) => format!(
                "HTTP/1.1 200 OK\r\n\
                Content-Length: {}\r\n\
                Content-Type: text/html\r\n\
                \r\n\
                {}",
                contents.len(),
                contents
            ),
            Err(_) => format!(
                "HTTP/1.1 404 NOT FOUND\r\n\
                Content-Length: 13\r\n\
                Content-Type: text/plain\r\n\
                \r\n\
                Not Found"
            ),
        };
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn extract_path(request: &str) -> String {
    let mut path = String::from("index.html"); // Default to index.html
    if let Some(first_line) = request.lines().next() {
        if let Some(second_part) = first_line.split_whitespace().nth(1) {
            if second_part != "/" {
                path = second_part.trim_start_matches('/').to_string();
            }
        }
    }
    path
}

fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // Bind the server to localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Listening on http://127.0.0.1:3000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
