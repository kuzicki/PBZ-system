use super::pages::*;
use super::{get_route_arg, Response, Status};
use crate::model::dao::*;
use crate::model::pool::PostgrePool;
use std::collections::HashMap;
use std::rc::Rc;

pub fn handle_unit_table(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }

    let tech_dao = unit::UnitDAOImpl::new(pool);
    match tech_dao.get_all() {
        Ok(tech) => Response::ok(unit_pages::table_page(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}

pub fn handle_unit_add(
    method: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    if method == "GET" {
        return Response::ok(unit_pages::add_form_get());
    } else if method == "POST" {
        let body = body.unwrap();
        let unit_number: i32 = match body["unit_number"].parse() {
            Ok(x) => x,
            Err(e) => return Response::internal_server_error(&e.to_string()),
        };
        let mut unit = unit::Unit::new(unit_number, &body["full_name"], &body["short_name"]);

        let unit_dao = unit::UnitDAOImpl::new(pool);
        let message = match unit_dao.insert(&mut unit) {
            Err(_) => Message::new(
                "Tried adding unit(the unit number should be unique)",
                MK::Error,
            ),
            Ok(()) => Message::new("Updated unit", MK::Notify),
        };

        let response = Response::ok(unit_pages::add_form_post(message));
        return response;
    } else {
        return Response::method_not_allowed();
    }
}

pub fn handle_unit_edit(
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

    let unit_dao = unit::UnitDAOImpl::new(pool);
    let unit = match unit_dao.get_id(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    match method {
        "GET" => Response::ok(unit_pages::edit_form_get(&unit)),
        "POST" => {
            let body = body.unwrap();
            let unit_number: i32 = match body["unit_number"].parse() {
                Ok(x) => x,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };
            let mut unit = unit::Unit::new(unit_number, &body["full_name"], &body["short_name"]);

            unit.set_id(arg);

            let message = match unit_dao.update(&mut unit) {
                Err(e) => Message::new(
                    &format!("Tried updating unit: {}", e.to_string()),
                    MK::Error,
                ),
                Ok(()) => Message::new("Updated unit", MK::Notify),
            };

            let response = Response::ok(unit_pages::edit_form_post(&unit, message));
            return response;
        }
        _ => Response::method_not_allowed(),
    }
}

pub fn handle_unit_delete(method: &str, route: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "POST" {
        return Response::method_not_allowed();
    }
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguments from url");
        }
    };

    let unit_dao = unit::UnitDAOImpl::new(pool);
    match unit_dao.delete(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    Response::found("/unit")
}

pub fn handle_view_unit_tech(method: &str, route: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguments from url");
        }
    };
    let tech_dao = tech::TechDaoImpl::new(pool);
    match tech_dao.get_by_unit_id(arg) {
        Ok(tech) => Response::ok(tech_pages::table_page_view_unit(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}

pub fn handle_unit_tech(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }

    let unit_dao = unit::UnitDAOImpl::new(pool);
    match unit_dao.get_all() {
        Ok(tech) => Response::ok(unit_pages::table_page_view(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}
