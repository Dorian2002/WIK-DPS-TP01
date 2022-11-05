use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let address = match env::var("PING_LISTEN_PORT") {
        Ok(val) => format!("0.0.0.0:{val}"),
        Err(e) => format!("0.0.0.0:8080"),
    };
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    if http_request[0] == "GET /ping HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let mut contents:Vec<String> = Vec::new();
        for i in http_request[1..].into_iter() {
            let (data1, data2) = i.split_once(": ").unwrap();
            contents.push(format!("\"{}\":\"{}\"",data1, data2));
        }
        contents.push(format!("\"{}\":\"{}\",","Content-Type", "application/json"));
        let response = format!("{status_line}\r\n\r\n{{\n\t{}\n}}",contents.join(",\n\t"));
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let response = format!(
            "{status_line}\r\n"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}