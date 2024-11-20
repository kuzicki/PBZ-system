pub mod connection;
pub(super) mod controller;
pub(super) mod employee_controller;
pub(super) mod unit_controller;
pub(super) mod tech_controller;
pub(super) mod pages;
use maud::{html, Markup};
use pages::base_page;
use std::str::FromStr;

pub fn get_route_arg<T>(route: &str, count: usize) -> Option<T>
where
    T: FromStr,
{
    let route = route.split('?').next()?;
    let part = route.split('/').nth(count)?;
    part.parse::<T>().ok()
}

struct Location {
    uri: String,
}

enum Status {
    Ok,
    NotFound,
    MethodNotAllowed,
    Found(Location),
    InternalServerError,
}

struct Response {
    status: Status,
    page: Markup,
}

impl Response {
    fn method_not_allowed() -> Response {
        return Response {
            status: Status::MethodNotAllowed,
            page: base_page::method_not_allowed(),
        };
    }

    fn internal_server_error(reason: &str) -> Response {
        return Response {
            status: Status::InternalServerError,
            page: base_page::base_error_template("Internal server error", reason),
        };
    }

    fn ok(page: Markup) -> Response {
        return Response {
            status: Status::Ok,
            page,
        };
    }

    fn found(route: &str) -> Response {
        let location = Location {
            uri: route.to_string(),
        };
        return Response {
            status: Status::Found(location),
            page: html! {},
        };
    }

    fn not_found() -> Response {
        return Response {
            status: Status::NotFound,
            page: base_page::base_error_template("Not found", "Not found"),
        };
    }

    fn get_status_str(&self) -> String {
        let status = match &self.status {
            Status::Ok => "HTTP/1.1 200 OK".to_string(),
            Status::NotFound => "HTTP/1.1 404 Not found".to_string(),
            Status::MethodNotAllowed => "HTTP/1.1 405 Method Not Allowed".to_string(),
            Status::InternalServerError => "500 Internal Server Error".to_string(),
            Status::Found(location) => {
               format!("HTTP/1.1 302 Found\nLocation: {}", location.uri) 
            }
        };
        status
    }
}
