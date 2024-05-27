// Install modules
use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

// Handle connection
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

    // Figure out request type
    let request_split: Vec<&str> = http_request[0].split("/").collect();
    let request_type = (request_split).first().unwrap().trim();

    // Update accordingly
    match request_type {
        "GET" => get(),
        "POST" => post(&http_request),
        "PUT" => println!("PUT"),
        "PATCH" => println!("PATCH"),
        "DELETE" => println!("DELETE"),
        _ => println!("ERROR")
    }

    // Return response
    let status_line = "HTTP/1.1 200 OK";
    let body = format!("Nice {request_type} request partner!");
    let length = body.chars().count();

    let response =
        format!("{status_line}\r\n{length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get() {
    println!("Get");
}

fn post<T>(http_request: &Vec<T>) {
    // Get post request body
    println!("Post");
}