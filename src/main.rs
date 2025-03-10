use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const PING_LISTEN_PORT: &str = "8080";

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PING_LISTEN_PORT)).unwrap();
    for stream in listener.incoming().filter_map(Result::ok) {
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);

    let response = if request.starts_with("GET /ping") {
        format_response(200, &extract_headers(&request))
    } else {
        format_response(404, "")
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn extract_headers(request: &str) -> String {
    request.lines().skip(1).take_while(|l| !l.is_empty())
        .filter_map(|line| line.split_once(": "))
        .map(|(k, v)| format!("  \"{}\": \"{}\"", k, v))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn format_response(status_code: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_code, body.len(), body
    )
}
