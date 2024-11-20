use super::pages::*;
use super::{get_route_arg, Response, Status};
use crate::model::dao::*;
use crate::model::pool::PostgrePool;
use maud::{html, Markup};
use std::collections::HashMap;
use std::rc::Rc;

pub fn handle_employee_table(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }

    let employee_dao = employee::EmployeeDAOImpl::new(pool);
    match employee_dao.get_all() {
        Ok(tech) => Response::ok(employee_pages::table_page(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}

pub fn handle_employee_add(
    method: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    if method == "GET" {
        let unit_dao = unit::UnitDAOImpl::new(pool);
        let units = match unit_dao.get_all() {
            Ok(x) => x,
            Err(_) => return Response::internal_server_error("Tried accessing db"),
        };
        return Response::ok(employee_pages::add_form_get(units));
    } else if method == "POST" {
        let body = body.unwrap();
        let employee_dao = employee::EmployeeDAOImpl::new(pool.clone());
        let unit_dao = unit::UnitDAOImpl::new(pool.clone());
        let units = match unit_dao.get_all() {
            Ok(x) => x,
            Err(_) => return Response::internal_server_error("Tried accessing db"),
        };

        let is_supervisor: bool = body.get("is_supervisor").is_some();
        let is_accountable: bool = body.get("is_accountable").is_some();
        if is_supervisor == is_accountable {
            let message = Message::new(
                "The employee has to be either a supervisor or a accoutanble",
                MK::Notify,
            );
            return Response::ok(employee_pages::add_form_post(units, message));
        }

        let unit_id: i32 = match body["unit"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("The unit has to be selected", MK::Error);
                return Response::ok(employee_pages::add_form_post(units, message));
            }
        };

        let mut employee = employee::Employee::new(
            &body["first_name"],
            &body["middle_name"],
            &body["last_name"],
            is_supervisor,
            is_accountable,
            &body["job_title"],
            unit_id,
        );

        let message = match employee_dao.insert(&mut employee) {
            Ok(()) => Message::new("Added employee", MK::Notify),
            Err(_) => Message::new("Error on adding new employee", MK::Error),
        };
        let response = Response::ok(employee_pages::add_form_post(units, message));
        return response;
    } else {
        return Response::method_not_allowed();
    }
}

pub fn handle_employee_edit(
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

    let employee_dao = employee::EmployeeDAOImpl::new(pool.clone());
    let employee = match employee_dao.get_id(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let unit_dao = unit::UnitDAOImpl::new(pool.clone());
    let units = match unit_dao.get_all() {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    match method {
        "GET" => Response::ok(employee_pages::edit_form_get(&employee, units)),
        "POST" => {
            let body = body.unwrap();
            let unit_dao = unit::UnitDAOImpl::new(pool.clone());
            let employee_dao = employee::EmployeeDAOImpl::new(pool);
            let units = unit_dao.get_all().expect("Tried getting all units");
            let is_supervisor: bool = body.get("is_supervisor").is_some();
            let is_accountable: bool = body.get("is_accountable").is_some();
            if is_supervisor == is_accountable {
                let message = Message::new(
                    "The employee has to be either a supervisor or a accoutanble",
                    MK::Notify,
                );
                return Response::ok(employee_pages::add_form_post(units, message));
            }

            let unit_id: i32 = match body["unit"].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Response::ok(employee_pages::add_form_post(
                        units,
                        Message::new("The unit has to be specified", MK::Error),
                    ))
                }
            };
            let mut employee = employee::Employee::new(
                &body["first_name"],
                &body["middle_name"],
                &body["last_name"],
                is_supervisor,
                is_accountable,
                &body["job_title"],
                unit_id,
            );

            let message = match employee_dao.insert(&mut employee) {
                Ok(()) => Message::new("Updated employee", MK::Notify),
                Err(_) => Message::new("Error on adding new employee", MK::Error),
            };

            let response = Response::ok(employee_pages::edit_form_post(&employee, units, message));
            return response;
        }
        _ => Response::method_not_allowed(),
    }
}

pub fn handle_employee_delete(method: &str, route: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "POST" {
        return Response::method_not_allowed();
    }
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguments from url");
        }
    };

    let employee_dao = employee::EmployeeDAOImpl::new(pool);
    match employee_dao.delete(arg) {
        Ok(x) => x,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    Response::found("/employee")
}

pub fn handle_view_employee_tech(method: &str, route: &str, pool: Rc<PostgrePool>) -> Response {
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
    match tech_dao.get_by_employee_id(arg) {
        Ok(tech) => Response::ok(tech_pages::table_page_view(tech)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}

pub fn handle_employee_tech(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }

    let employee_dao = employee::EmployeeDAOImpl::new(pool);
    match employee_dao.get_all() {
        Ok(employees) => Response::ok(employee_pages::table_page_view(employees)),
        Err(e) => Response::internal_server_error(&e.to_string()),
    }
}
