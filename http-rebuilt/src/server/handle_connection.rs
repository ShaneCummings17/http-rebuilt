// Install modules
use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

pub fn handle_connection(mut stream: TcpStream) {
    // Get incoming http request
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Print request
    println!("Request: {:#?}", http_request);

    // Return response
    let status_line = "HTTP/1.1 200 OK";
    let body = "received";
    let length = body.chars().count();

    let response =
        format!("{status_line}\r\n{length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}