use super::{controller, employee_controller, tech_controller, unit_controller, Response};
use crate::model::pool::PostgrePool;
use std::collections::HashMap;
use std::rc::Rc;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn form_response(status_line: &str, contents: &str) -> String {
    format!("{}\r\nAccess-Control-Allow-Origin: https://my-cool-site.com\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents)
}

fn parse_form_data(data: &str) -> HashMap<String, String> {
    data.split('&')
        .filter_map(|pair| {
            let mut split = pair.splitn(2, '=');
            Some((
                split.next()?.to_string(),
                split
                    .next()
                    .unwrap_or("")
                    .replace('+', " ")
                    .trim()
                    .to_string(),
            ))
        })
        .collect()
}

fn get_headers(
    buf_reader: &mut BufReader<&mut TcpStream>,
    content_length: &mut u32,
) -> Option<String> {
    let mut request = String::new();
    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => {
                eprintln!("Connection closed by client.");
                return None;
            }
            Ok(_) => {
                if line == "\r\n" {
                    break;
                }
                if line.to_lowercase().starts_with("content-length:") {
                    *content_length = line["content-length:".len()..].trim().parse().unwrap_or(0);
                }
                request.push_str(&line);
            }
            Err(e) => {
                eprintln!("Error reading request: {}", e);
                return None;
            }
        }
    }
    let request = request.replace('+', " ");
    println!("{}", request);
    Some(request)
}

fn get_body(
    buf_reader: &mut BufReader<&mut TcpStream>,
    method: &str,
    content_length: u32,
) -> Option<HashMap<String, String>> {
    let mut body = String::new();
    if method != "POST" || content_length == 0 {
        return None;
    }
    buf_reader
        .take(content_length as u64)
        .read_to_string(&mut body)
        .unwrap();
    println!("Received body:\n{}", body);
    let form_data = parse_form_data(&body);
    println!("Parsed form data: {:?}", form_data);
    return Some(form_data);
}

fn get_route(request: &str) -> (&str, &str) {
    let mut lines = request.lines();
    let request_line = lines.next().expect("Expected request line");
    let mut split_request = request_line.split_whitespace();
    let method = split_request.next().expect("Expected method");
    let route = split_request.next().expect("Expected route");
    (method, route)
}

fn opt_route(route: &str, pattern: &str) -> bool {
    route.starts_with(pattern)
}

fn handle_connection(mut stream: TcpStream, pool: Rc<PostgrePool>) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut content_length = 0;

    let request = match get_headers(&mut buf_reader, &mut content_length) {
        Some(val) => val,
        None => return,
    };

    // println!("Received headers:\n{}", request);
    let (method, route) = get_route(&request);

    let body = get_body(&mut buf_reader, method, content_length);

    let response = match route {
        "/" => controller::handle_welcome(method),
        "/tech" => tech_controller::handle_tech_table(method, pool),
        "/add-tech" => tech_controller::handle_tech_add(method, body, pool),
        "/unit" => unit_controller::handle_unit_table(method, pool),
        "/add-unit" => unit_controller::handle_unit_add(method, body, pool),
        "/employee" => employee_controller::handle_employee_table(method, pool),
        "/add-employee" => employee_controller::handle_employee_add(method, body, pool),
        "/transfer" => controller::handle_transfer_table(method, pool),
        "/add-transfer" => controller::handle_transfer_add(method, body, pool),
        "/unit-tech" => unit_controller::handle_unit_tech(method, pool),
        "/employee-tech" => employee_controller::handle_employee_tech(method, pool),
        _ if opt_route(route, "/edit-tech/") => {
            tech_controller::handle_tech_edit(method, route, body, pool)
        }
        _ if opt_route(route, "/delete-tech/") => {
            tech_controller::handle_tech_delete(method, route, pool)
        }
        _ if opt_route(route, "/edit-unit/") => {
            unit_controller::handle_unit_edit(method, route, body, pool)
        }
        _ if opt_route(route, "/delete-unit/") => {
            unit_controller::handle_unit_delete(method, route, pool)
        }
        _ if opt_route(route, "/edit-employee/") => {
            employee_controller::handle_employee_edit(method, route, body, pool)
        }
        _ if opt_route(route, "/delete-employee/") => {
            employee_controller::handle_employee_delete(method, route, pool)
        }
        _ if opt_route(route, "/edit-transfer/") => {
            controller::handle_transfer_edit(method, route, body, pool)
        }
        _ if opt_route(route, "/view-unit-tech/") => {
            unit_controller::handle_view_unit_tech(method, route, pool)
        }
        _ if opt_route(route, "/view-employee-tech/") => {
            employee_controller::handle_view_employee_tech(method, route, pool)
        }
        _ => {
            println!("Not found");
            Response::not_found()
        }
    };

    let response = form_response(&response.get_status_str(), &response.page.into_string());
    // println!("{}", response);
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn open_connection(pool: Rc<PostgrePool>) {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, pool.clone());
    }
}
