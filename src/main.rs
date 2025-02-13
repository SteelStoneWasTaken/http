use std::io::{Write, Read};
use std::net::TcpListener;
use std::thread;

mod page;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024];
            let s = stream.read(&mut buffer).unwrap();
            let info = String::from_utf8_lossy(&buffer[..s]);

            let path = info
                .lines().next().unwrap()
                .split_whitespace().nth(1).unwrap();
            
            let mut content = String::new();
            let content_type;

            if   path.ends_with(".css") { content_type = "text/css" } else
            if   path.ends_with(".js")  { content_type = "application/javascript" }
            else {
                content_type = "text/html";
                content = page::get(path);
            }

            stream.write_all(&response(
                200,
                content_type,
                content.as_str()
            )).unwrap();
        });
    }
}

fn response(
    status_code: u16,
    content_type: &str,
    content: &str,
) -> Vec<u8> {

    let mut status = "UNKNOWN";
    match status_code {
        200 => status = "OK",
        404 => status = "NOT FOUND",
        _ => ()
    }

    format!("\
        HTTP/1.1 {status_code} {status}\r\n\
        Content-Type: {content_type}\r\n\
        Content-Length: {}\r\n\
        Connection: keep-alive\r\n\
        \r\n{content}",

        content.len()
    ).as_bytes().to_vec()
}