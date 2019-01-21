use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::fs::File;

mod http_request;
mod http_response;
use http_request::{HttpRequest, Method};
use http_response::{Status, HttpResponse};

fn build_request(mut stream: &TcpStream) -> Option<HttpRequest> {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            Some(HttpRequest::new(req_str.into_owned()))
            },
        Err(error) => {
            println!("Failed to read stram: {:?}", error);
            None
        },
    }
}

fn build_response(request: HttpRequest) -> HttpResponse {
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            Some(HttpRequest::new(req_str.into_owned()))
            },
        Err(error) => {
            println!("Failed to read stram: {:?}", error);
            None
        },
    }
}

fn handle_request(request: HttpRequest) -> Option<HttpResponse> {
    let HttpRequest { method, uri, text, protocol, version, headers } = request;
    let response = HttpResponse::new(Status { code: 200, text: String::from("OK")});
    match &method {
        Method::GET => {
            let mut html_file = File::open("./html/hello.html").expect("Failed to open HTML file");

            let mut file_content = String::new();
            html_file.read_to_string(&mut file_content).expect("Failed to read file content");

            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", file_content);
            Some(response)
        },
        Method::POST => println!("Handle this POST request"),
        Method::PUT => println!("Handle this PUT request"),
        Method::DELETE => println!("Handle this DELETE request"),
    }

    // match stream.write(response.as_bytes()) {
    //     Ok(_) => println!("Response sent"),
    //     Err(error) => println!("Failed sending response: {}", error),
    // }
}

fn handle_connection(stream: TcpStream) {
    let http_request = build_request(&stream).expect("Failed to instantiate HttpRequest");
    let http_response = build_response(http_request).expect("Failed to build HttpResponse");
    build_response(http_request);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind on port 8080");
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => {
                println!("Unable to connect: {}", error);
            }
        }
    }
}
