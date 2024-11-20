use super::pages::*;
use super::{get_route_arg, Response, Status};
use crate::model::dao::*;
use crate::model::pool::PostgrePool;
use std::collections::HashMap;
use std::rc::Rc;

pub fn handle_tech_table(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }
    let tech_dao = tech::TechDaoImpl::new(pool);
    match tech_dao.get_all() {
        Ok(tech) => Response::ok(tech_pages::table_page(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}

pub fn handle_tech_add(
    method: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    if method == "GET" {
        return Response::ok(tech_pages::add_form_get());
    } else if method == "POST" {
        let tech_dao = tech::TechDaoImpl::new(pool);
        let body = body.unwrap();
        let price: i32 = match body["price"].parse() {
            Ok(x) => x,
            Err(e) => return Response::internal_server_error(&e.to_string()),
        };
        let inventory_number: i32 = match body["inventory_number"].parse() {
            Ok(x) => x,
            Err(e) => return Response::internal_server_error(&e.to_string()),
        };

        let mut tech = tech::Tech::new(
            inventory_number,
            &body["name"],
            &body["model"],
            &body["acquisition_date"],
            price,
        );

        let message = match tech_dao.insert(&mut tech) {
            Ok(()) => Message::new("Added tech", MK::Notify),
            Err(_) => Message::new("Failed to add tech", MK::Error),
        };
        let response = Response::ok(tech_pages::add_form_post(message));
        return response;
    } else {
        return Response::method_not_allowed();
    }
}

pub fn handle_tech_edit(
    method: &str,
    route: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguments from url");
        }
    };

    let tech_dao = tech::TechDaoImpl::new(pool);
    let tech = match tech_dao.get_id(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    match method {
        "GET" => Response::ok(tech_pages::edit_form_get(&tech)),
        "POST" => {
            let message = Message::new("Updated tech", MK::Notify);
            let body = body.unwrap();

            let price: i32 = match body["price"].parse() {
                Ok(x) => x,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };

            let inventory_number: i32 = match body["inventory_number"].parse() {
                Ok(x) => x,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };

            let mut tech = tech::Tech::new(
                inventory_number,
                &body["name"],
                &body["model"],
                &body["acquisition_date"],
                price,
            );

            tech.set_id(arg);
            tech_dao.update(&tech);
            let response = Response::ok(tech_pages::edit_form_post(&tech, message));
            return response;
        }
        _ => Response::method_not_allowed(),
    }
}

pub fn handle_tech_delete(method: &str, route: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "POST" {
        return Response::method_not_allowed();
    }
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguments from url");
        }
    };

    let tech_dao = tech::TechDaoImpl::new(pool);
    match tech_dao.delete(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    Response::found("/tech")
}
